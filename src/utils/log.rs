use std::fs::File;
use std::io::Write;
use std::result::Result;

use crate::utils::error::DbgError;



pub struct DbgLog {
    log_buffer: Vec<String>,
    log_path: String,
    log_file: File,
    buffer_size: usize,
}


impl DbgLog {
    pub fn new(log_path: String, buffer_size: usize) -> Result<DbgLog, DbgError> {
        let log_file: File = match File::create(&log_path) {
            Ok(file) => file,
            Err(_) => {
                return Err(DbgError::new(&format!("Failed to create log file: {}", log_path)));
            },
        };

        Ok(DbgLog {
            log_buffer: Vec::new(),
            log_path: log_path,
            log_file: log_file,
            buffer_size: buffer_size,
        })
    }


    pub fn log(&mut self, message: &str) {
        self.log_buffer.push(message.to_string());
    }


    fn flush(&mut self) -> Result<(), DbgError> {
        for message in &self.log_buffer {
            match writeln!(&self.log_file, "{}", message) {
                Ok(_) => (),
                Err(_) => {
                    return Err(DbgError::new("Failed to write to log file"));
                },
            }
        }

        self.log_buffer.clear();
        Ok(())
    }
}