use std::path::Path;
use ptracer::Ptracer;
use nix::unistd::Pid;
use ansi_term::Colour;

use crate::utils::error::DbgError;
use crate::utils::arg::{Arg, ArgParser};
use crate::utils::constants::{commands, options};



pub fn exe(args: Vec<String>) -> Result<(), DbgError>{
    if args.len() == 0 {
        return Err(DbgError::new("No arguments provided"));
    }
    if args.len() == 1 {
        if args[0] == "help" {
            exe_help();
            return Ok(());
        }
    }
    match get_args(args) {
        Ok(arg_parser) => {
            let arg_parser: ArgParser = arg_parser;
        },
        Err(e) => {
            return Err(e);
        },
    }
    // let tracer: Ptracer = match run_file(file_path, args) {
    //     Ok(tracer) => tracer,
    //     Err(e) => {
    //         eprintln!("{}", e);
    //         return false;
    //     },
    // };

    return Ok(());
}


fn get_args(args: Vec<String>) -> Result<ArgParser, DbgError> {
    let mut arg_parser: ArgParser = ArgParser::new("exe");

    let file: Arg = Arg::new("file")
        .long("file")
        .short('f')
        .help("The path to the executable to run")
        .num_args(1)
        .required(true)
        .with_value(true);
    arg_parser.add_arg(file);

    let file_args: Arg = Arg::new("file_args")
        .long("file_args")
        .short('a')
        .help("Arguments to pass to the executable")
        .num_args(1)
        .required(false)
        .with_value(true);
    arg_parser.add_arg(file_args);

    let time_travel: Arg = Arg::new("time_travel")
        .long("time_travel")
        .short('t')
        .help("Run the executable in time travel mode")
        .num_args(0)
        .required(false)
        .with_value(false);
    arg_parser.add_arg(time_travel);

    match arg_parser.parse_args(args) {
        Ok(_) => {
            return Ok(arg_parser);
        },
        Err(e) => {
           return Err(e);
        },
    }
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


fn handle_command(command: &str, args: Vec<&str>) -> Result<String, DbgError> {
    match command {
        "" => {
            return Ok("".to_string());
        },
        _ => {
            return Err(DbgError::new(&format!("{} is not a recognized command", command)));
        },
    }
}


fn exe_help() {
    println!("{}", options());
    println!("  {} {}\t\t- The path to the executable to run", Colour::Fixed(14).paint("-f --file"), Colour::Cyan.paint("<FILE_PATH>"));
    println!("  {} {}\t\t- Arguments to pass to the executable", Colour::Fixed(14).paint("-a --file_args"), Colour::Cyan.paint("<ARGS>"));
    println!("  {}\t\t- Run the executable in time travel mode", Colour::Fixed(14).paint("-t --time_travel"));
}