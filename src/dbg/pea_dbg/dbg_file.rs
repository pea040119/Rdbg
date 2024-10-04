use std::path::Path;
use ptracer::Ptracer;
use nix::unistd::Pid;
use crate::utils::error::DbgError;



pub fn dbg(file_path: &str, args: &[&str]) -> bool{
    let tracer: Ptracer = match run_file(file_path, args) {
        Ok(tracer) => tracer,
        Err(e) => {
            eprintln!("{}", e);
            return false;
        },
    };

    return true;
}


fn run_file(_file_path: &str, args: &[&str] ) -> Result<Ptracer, DbgError> {
    let file_path: &Path = Path::new(_file_path);
    if !file_path.exists() {
        return Err(DbgError::new(&format!("{} does not exist", file_path.display())));
    }

    let args: Vec<String> = args.iter().map(|&arg| arg.to_string()).collect();

    match Ptracer::spawn(file_path, &args) {
        Ok(tracer) => {
            return Ok(tracer);
        },
        Err(e) => {
            return Err(DbgError::new(&format!("Failed to run {}: {}", file_path.display(), e)));
        }
    }
}