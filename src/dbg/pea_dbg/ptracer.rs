use std::path::Path;
use nix::sys::ptrace;
use nix::sys::signal::Signal;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::Pid;

use crate::utils::error::DbgError;



pub struct Ptracer {
    pid: Pid,
}


impl Ptracer {
    pub fn new() -> Self {
        return Ptracer {   
            pid: Pid::from_raw(0),  
        };
    }


    pub fn attach(&mut self, pid: i32) -> Result<(), DbgError> {
        self.pid = Pid::from_raw(pid);
        ptrace::attach(self.pid).map_err(|e| DbgError::new(&format!("Failed to attach to process: {}", pid)))?;
        return Ok(());
    }


    pub fn run_file(&mut self, _file_path:String, mut args: String ) -> Result<Ptracer, DbgError> {
        let file_path = Path::new(&_file_path);
        if !file_path.exists() {
            return Err(DbgError::new(&format!("{} does not exist", file_path.display())));
        }
    
        args = args.trim().to_string();
        let args: Vec<String> = args.split_whitespace().map(|s| s.to_string()).collect();
    
        match Ptracer::spawn(file_path, &args) {
            Ok(tracer) => {
                return Ok(tracer);
            },
            Err(e) => {
                return Err(DbgError::new(&format!("Failed to run {}: {}", file_path.display(), e)));
            }
        }
    }
}