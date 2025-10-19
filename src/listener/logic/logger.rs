use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, Arc};
use std::thread;
use std::fs::File;
use std::io::{BufWriter, Write, Error};

use chrono::{*};

const DEFAULT_PATH: &str = "src/logs/list.log";

#[derive(Clone)]  // auto copy/clone
pub struct Logger {
    file_path: String,
    log_id: Arc<AtomicU32>,
    logs: Arc<Mutex<Vec<String>>>,
}

impl Logger {
    // todo: need add comfortable interface for using logger

// ##### PUBLIC AREA #####
    pub fn log_key(&self, key_name: String) {
        // create log parts
        let time = prelude::Utc::now();
        let formatted_time = time.format("[%H:%M:%S.%3f]: ").to_string();
        let formatted_id = format!(
            "{}, ", self.log_id.load(Ordering::Acquire)
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

        // save log
        let mut guard =  self.logs.lock().unwrap();
        guard.push(log_line);

        // update log id
        self.log_id.fetch_add(1, Ordering::Release);
    }

    pub fn start(&self) -> Result<String, Error> {
        let file = File::create(self.file_path.clone());

        // if everything is ok, run logger
        match file {
            Ok(file) => {
                let logs_ptr = self.logs.clone();
                let mut writer = BufWriter::new(file);

                // create a "log_writer" thread
                let _ = thread::spawn(move || {
                    // write logs in file
                    loop {
                        // wait until logs be more
                        thread::sleep(std::time::Duration::from_secs(3));

                        // save logs in BufWritter as u8 (chars) 
                        let mut guard =  logs_ptr.lock().unwrap();

                        for _ in 0..guard.len() {
                            let _ = writer.write_all(guard[0].as_bytes());
                            guard.remove(0);
                        }

                        //print all in file
                        let _ = writer.flush();
                        println!("log saved");

                        // !!!
                        // todo: when system decided shutdown,
                        // this thread must save other logs and stop
                    }
                });
                
                return Ok(String::from("File found"));
            },
            Err(err) => return Err(err),
        };
    }

// constructor
    pub fn new() -> Logger {
        Logger {
            file_path: String::from(DEFAULT_PATH),
            log_id:    Arc::new(AtomicU32::new(0)),
            logs:      Arc::new(Mutex::new(vec![])),
        }
    }
}