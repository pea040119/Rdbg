use std::io::{self, Write};
use std::str::SplitWhitespace;
use ansi_term::Colour;

use crate::utils::error::DbgError;
use crate::utils::arg::{Arg, ArgParser};
use crate::utils::constants::{commands, options};
use crate::dbg::pea_dbg::ptracer::Ptracer;



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

    let mut ptracer: Ptracer = Ptracer::new();

    let arg_parser: ArgParser = match get_args(args) {
        Ok(arg_parser) => arg_parser,
        Err(e) => {
            return Err(e);
        },
    };
    let file_path: String  = match arg_parser.get_values("file") {
        Ok(values) => values[0].clone(),
        Err(e) => {
            return Err(e);
        }
    };

    let file_args: String  = match arg_parser.get_values("file_args") {
        Ok(values) => {
            if values.len() > 0 {
                values[0].clone()
            } else {
                "".to_string()
            }
        },
        Err(e) => {
            return Err(e);
        }
    };
    match ptracer.run_file(file_path, file_args) {
        Ok(tracer) => tracer,
        Err(e) => {
            return Err(e);
        },
    };

    loop {
        print!("{}$ ", Colour::Green.paint("exe"));
        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input: &str = input.trim();

        if input == "exit"{
            break;
        }

        let mut inputs: SplitWhitespace<'_>  = input.split_whitespace();
        let command: &str = inputs.next().unwrap_or("");
        let args: Vec<String> = inputs.map(|s| s.to_string()).collect();

        match handle_command(command) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{}", e);
            },
        }
    }

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


fn handle_command(command: &str) -> Result<String, DbgError> {
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