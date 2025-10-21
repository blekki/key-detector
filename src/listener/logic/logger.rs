use std::sync::atomic::{AtomicU8, AtomicU32, Ordering};
use std::sync::{Mutex, Arc};
use std::thread;
use std::fs::File;
use std::io::{BufWriter, Write, Error};

use chrono::{*};

// constants
const DEFAULT_PATH: &str = "src/logs/list.log";
const RELAXED:       u8 = 0;
const SHOULD_STOP:   u8 = 1;
const READY_TO_STOP: u8 = 2;

#[derive(Clone)]  // auto copy/clone
pub struct Logger {
    file_path: String,
    next_log_id: Arc<AtomicU32>,
    logs: Arc<Mutex<Vec<String>>>,
    signal: Arc<AtomicU8>,
}

impl Logger {
// ##### PRIVATE AREA #####
    fn run_log_writter(&self, file: File) {
        let mut writer: BufWriter<File> = BufWriter::new(file);

        // create a "log_writer" thread
        let logs_clone: Arc<Mutex<Vec<String>>> = self.logs.clone();
        let signal_clone: Arc<AtomicU8> = self.signal.clone();
        let _ = thread::spawn(move || {

            // lambda function
            let mut save_logs_in_file = move || {
                let mut guard = logs_clone.lock().unwrap();

                // process front log and after remove it from vector
                for _ in 0..guard.len() {
                    // save logs in BufWritter as u8 (chars). It saves a lot of time
                    let _ = writer.write_all(guard[0].as_bytes());
                    guard.remove(0);
                }

                let _ = writer.flush(); // save buffer in file (saves logs)
                println!("[logger]: logs are saved");
            };

            // basic loop
            loop {
                thread::sleep(std::time::Duration::from_secs(3)); // wait until logs be more

                // # save logs in the file
                save_logs_in_file();

                // check did signal to stop come
                let ready_to_stop = {
                    signal_clone.load(Ordering::Acquire) == SHOULD_STOP
                };
                if ready_to_stop {
                    // save unsaved before logs
                    save_logs_in_file();

                    // send signal logger is ready shutdown
                    signal_clone.store(READY_TO_STOP, Ordering::Release);
                    break;
                }
            }
        });
    }

// ##### PUBLIC AREA #####
    pub fn shutdown(&self) {
        self.signal.store(SHOULD_STOP, Ordering::Release);
        
        // wait until all processes stopped
        while self.signal.load(Ordering::Acquire) != READY_TO_STOP {
            println!("[logger]: waiting (is saving logs)");
            thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    pub fn log_key(&self, key_name: &str) {
        // create log parts
        let time = prelude::Utc::now();
        let formatted_time = time.format("[%H:%M:%S:%3f]: ").to_string();
        let formatted_id = format!(
            "{}, ", self.next_log_id.load(Ordering::Acquire)
        );
        let formatted_key = format!(
            "\'{}\'\n", key_name
        );

        // save parts as &str
        let mut parts = ["", "", ""];
        parts[0] = formatted_time.as_str();
        parts[1] = formatted_id.as_str();
        parts[2] = formatted_key.as_str();
        
        // contain all parts in one string
        let mut log_line = String::new();
        log_line.extend(parts.iter().copied());

        // save log in the temporary vector before it will be wrote in the file
        let mut guard =  self.logs.lock().unwrap();
        guard.push(log_line);

        // update log id
        self.next_log_id.fetch_add(1, Ordering::Release);
    }

    pub fn start(&self) -> Result<String, Error> {
        let file = File::create(self.file_path.clone());

        // if everything is ok, run logger
        match file {
            Ok(file) => {
                // start write logs in the log-file (works such a thread)
                self.run_log_writter(file);

                // return msg "everything is ok"
                return Ok(String::from("File found"));
            },
            Err(err) => return Err(err),
        };
    }

// constructor
    pub fn new() -> Logger {
        Logger {
            file_path:   String::from(DEFAULT_PATH),
            next_log_id: Arc::new(AtomicU32::new(0)),
            logs:        Arc::new(Mutex::new(vec![])),
            signal:      Arc::new(AtomicU8::new(RELAXED)),
        }
    }
}