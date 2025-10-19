use std::sync::{Mutex, Arc};
use std::thread;
use std::fs::File;
use std::io::{BufWriter, Write, Error};

const DEFAULT_PATH: &str = "src/logs/list.log";

#[derive(Clone)]  // auto copy/clone
pub struct Logger {
    file_path: String,
    logs: Arc<Mutex<Vec<String>>>,
}

impl Logger {
    // todo: need add comfortable interface for using logger

// ##### PUBLIC AREA #####
    pub fn log_key(&self) {
        let mut guard =  self.logs.lock().unwrap();

        let mut pattern = ["", "", "", ""];
        // How it loks: 
        // [--:--:--]: A, pressed, ID
        pattern[0] = "[--:--:--]: ";
        pattern[1] = "A, ";
        pattern[2] = "pressed, ";
        pattern[3] = "ID\n";

        let mut log_line = String::new();
        log_line.extend(pattern.iter().copied());

        guard.push(log_line);
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
                        let guard =  logs_ptr.lock().unwrap();
                        for line in guard.iter() {
                            let _ = writer.write_all(line.as_bytes());
                        }

                        //print all in file
                        let _ = writer.flush();

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
            logs: Arc::new(Mutex::new(vec![])),
        }
    }
}