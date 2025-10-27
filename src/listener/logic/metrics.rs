use std::sync::atomic::{AtomicU8, AtomicU32, Ordering};
use std::sync::Arc;
use std::{thread, fs, fs::File};
use std::io::{BufReader, BufRead, Error, ErrorKind};

// constants
const DEFAULT_PATH: &str = "src/data/stats.met";
const MT_TOTAL_PRESSED: &str = "Total-pressed";    // MT = metric
// Metrics states
const RELAXED:       u8 = 0;
const SHOULD_STOP:   u8 = 1;
const READY_TO_STOP: u8 = 2;

#[derive(Clone)]  // auto copy/clone
pub struct Metrics {
    file_path: String,
    signal: Arc<AtomicU8>,
    total_pressed: Arc<AtomicU32>,  // metric parameter
}

impl Metrics {
// ##### PRIVATE AREA #####
    fn start_metric_writter(&self) {
        // handles
        let path_handle = self.file_path.clone();
        let total_pressed_handle = self.total_pressed.clone();
        let signal_handle: Arc<AtomicU8> = self.signal.clone();

        // create a "metric_writer" thread
        let _ = thread::spawn(move || {
            // lambda function
            let save_metric_in_file = move || {

                let formatted_total_pressed = format!(
                    "{}: {}\n",
                    MT_TOTAL_PRESSED,
                    total_pressed_handle.load(Ordering::Acquire)
                );

                let _ = fs::write(
                    path_handle.clone(),
                    formatted_total_pressed
                );
            };

            // basic loop
            loop {
                // wait until metric be more
                thread::sleep(std::time::Duration::from_secs(3));
                
                // save metric
                save_metric_in_file();

                // check did signal to stop come
                let signal = signal_handle.load(Ordering::Acquire);
                if signal == SHOULD_STOP {
                    break;
                }
            }

            // save unsaved before metric
            save_metric_in_file();
            // send signal logger is ready shutdown
            signal_handle.store(READY_TO_STOP, Ordering::Release);
        });
    }

    // get total pressed conut from string
    fn read_total_pressed(&self, line: String) -> Result<String, Error> {
        let number_str = line.trim_start().split_whitespace().nth(1);

        // check does data exist
        if number_str.is_none() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "[matrics]: Incorrect data. Will be using default value"
            ));
        }

        // try to convert data into u32
        match number_str.unwrap().parse::<u32>() {
            Ok(value) => {
                // save value
                self.total_pressed.store(value, Ordering::Relaxed);
                return Ok(String::from("[matrics]: Read total pressed"));
            },
            _ => {
                // keed default value
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "[matrics]: Incorrect data. Will be using default value"
                ));
            }
        }
    }

    // read file for getting data
    fn read_data(&self, file: File) -> Result<String, Error> {
        let reader = BufReader::new(file);

        // Iterate over each line in the file.
        for line_result in reader.lines() {
            let line = line_result?;

            if line.starts_with(MT_TOTAL_PRESSED) {
                let res = self.read_total_pressed(line);
                if res.is_err() { return res; } // fast way stop process
            }
        }

        return Ok(String::from("[matrics]: Data was correctly read"));
    }

    fn set_ready_to_stop(&self) {
        self.signal.store(READY_TO_STOP, Ordering::Release);
    }

// ##### PUBLIC AREA #####
    pub fn shutdown(&self) {
        // case: metrics is ready shoutdown, but "metric_writter" doesn't work
        if self.signal.load(Ordering::Acquire) == READY_TO_STOP {
            println!("[metrics]: Metrics shutdown");
            return;
        }

        // sent to the internal systems signal to stop their processes
        self.signal.store(SHOULD_STOP, Ordering::Release);
        
        // wait until all processes stopped
        while self.signal.load(Ordering::Acquire) != READY_TO_STOP {
            println!("[metrics]: waiting (metric is saving)");
            thread::sleep(std::time::Duration::from_secs(1));
        }
        println!("[metrics]: Metrics shutdown");
    }

    pub fn update_metric(&self, _key_name: &str) {
        self.total_pressed.fetch_add(1, Ordering::Relaxed);
        
        // # Note: 
        // calculation about get more metric can be placed:
        // >>> here
    }

    pub fn start(&self) -> Result<String, Error> {
        let source = File::open(self.file_path.clone());
        
        // check does opening file finished success
        let file_copy: File;    // this variable makes code more readble
        match source {
            Ok(file)  => file_copy = file,
            Err(err) => {
                self.set_ready_to_stop();
                return Err(err);
            }
        };

        // run all iternal systems
        // let msg = self.read_data(file_copy);
        match self.read_data(file_copy) {
            Ok(ok) => {
                self.start_metric_writter();    // run metric writer
                return Ok(ok);
            },
            Err(err) => {
                self.set_ready_to_stop();
                return Err(err);
            }
        }
    }

    // constructor
    pub fn new() -> Metrics {
        let metrics = Metrics {
            file_path:      String::from(DEFAULT_PATH),
            signal:         Arc::new(AtomicU8::new(RELAXED)),
            total_pressed:  Arc::new(AtomicU32::new(0)),
        };

        return metrics;
    }    
}