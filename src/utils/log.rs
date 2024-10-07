use std::{fmt::Result, fs::File};

use super::error::DbgError;



pub struct DbgLog {
    log_buffer: Vec<String>,
    log_path: String,
    log_file: File,
}


impl DbgLog {
    pub fn new(log_path: String) -> Result<DbgLog, DbgError> {
        let log_file: File = match File::create(&log_path) {
            Ok(file) => file,
            Err(_) => {
                return Err(DbgError::new(&format!("Failed to create log file: {}", log_path)));
            },
        };

        Ok(DbgLog {
            log_buffer: Vec::new(),
            log_path,
            log_file,
        })
    }
}