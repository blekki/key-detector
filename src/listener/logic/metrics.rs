use std::sync::atomic::{AtomicU8, AtomicU32, Ordering};
use std::sync::{Arc};
use std::{fs, thread};
use std::fs::{File};
use std::io::{BufReader, BufRead};
use std::io::{Error, ErrorKind};

// use rdev::{*};

// use chrono::{*};

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

    total_pressed: Arc<AtomicU32>,
    // typing_speed
    // sum_keyboard_activity
    // favorite_key
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
                    "{}: {}\n", MT_TOTAL_PRESSED, total_pressed_handle.load(Ordering::Acquire)
                );

                let res = fs::write(
                    path_handle.clone(),
                    formatted_total_pressed
                );

                match res {
                    Ok(_ok) => println!("[metrics]: metric is saved stats"),
                    Err(_err) => println!("[error]: metric can't save stats"),
                }
            };

            // basic loop
            loop {
                thread::sleep(std::time::Duration::from_secs(3)); // wait until metric be more

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

    fn store_total_pressed(&self, line: String) -> Result<String, Error> {
        let number_str = line.trim_start().split_whitespace().nth(1);

        if number_str.is_none() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Data is not exists"
            ));
        }

        match number_str.unwrap().parse::<u32>() {
            Ok(value) => {
                self.total_pressed.store(value, Ordering::Relaxed);
                return Ok(String::from("Read total pressed"));
            },
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Incorrect data"
                ));
            }
        }
    }

    fn read_data(&self, file: File) -> Result<String, Error> {
        let reader = BufReader::new(file);

        // Iterate over each line in the file.
        for line_result in reader.lines() {
            let line = line_result?;

            if line.starts_with(MT_TOTAL_PRESSED) {
                let res = self.store_total_pressed(line);
                if res.is_err() { return res; } // fast way stop process
            }
        }

        return Ok(String::from("Data was correctly read"));
    }

// ##### PUBLIC AREA #####
    pub fn shutdown(&self) {
        // self.save_metric();
        // println!("[metrics]: Metrics shutdown");

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
    }

    // pub fn save_metric(&self) {
    //     let formatted_total_pressed = format!(
    //         "{}: {}\n", MT_TOTAL_PRESSED, self.total_pressed.load(Ordering::Acquire)
    //     );

    //     let res = fs::write(
    //         self.file_path.clone(), 
    //         formatted_total_pressed
    //     );

    //     match res {
    //         Ok(_ok) => println!("[metrics]: metric is saved stats"),
    //         Err(_err) => println!("[error]: metric can't save stats"),
    //     }
    // }

    pub fn start(&self) -> Result<String, Error> {
        let source = File::open(self.file_path.clone());
        
        // check does opening file finished success
        let file_copy: File;    // this cariable make code simpler for reading
        match source {
            Ok(file)  => file_copy = file,
            Err(err) => return Err(err),
        };

        // run all iternal systems
        match self.read_data(file_copy) {
            Ok(ok) => {
                self.start_metric_writter();    // run metric writer
                return Ok(ok);
            },
            Err(err) => return Err(err),
        }

    }

    pub fn new() -> Metrics {
        let metrics = Metrics {
            file_path:      String::from(DEFAULT_PATH),
            signal:         Arc::new(AtomicU8::new(RELAXED)),
            total_pressed:  Arc::new(AtomicU32::new(0)),
        };

        return metrics;
    }    
}