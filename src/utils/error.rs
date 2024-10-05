use std::fmt;
use std::error::Error;
use crate::utils::constants::error_header;



pub struct DbgError {
    msg: String,
}


impl DbgError {
    pub fn new(msg: &str) -> Self {
        return DbgError {
           msg: msg.to_string(),
       }
    }
}


impl fmt::Display for DbgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}: {}", error_header(), self.msg);
    }
}


impl fmt::Debug for DbgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", error_header(), self.msg)
    }
}


impl Error for DbgError {
    fn description(&self) -> &str {
        return &self.msg;
    }
}