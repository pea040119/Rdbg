use std::io::{self, Write};
use std::str::SplitWhitespace;
use ansi_term::Colour;

use crate::utils::error::DbgError;
use crate::utils::constants::commands;

pub mod exe;



pub fn dbg() {
    loop {
        print!(">>> ");
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

        match handle_command(command, args) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{}", e);
            },
        }
    }
}


fn handle_command(command: &str, args: Vec<String>) -> Result<(), DbgError> {
    match command {
        "exe" => {
            return exe::exe(args);
        },
        "help" => {
            help();
            return Ok(());
        },
        "" => {
            return Ok(());
        },
        _ => {
            return Err(DbgError::new(&format!("{} is not a recognized command", command)));
        },
    }
}


fn help() {
    println!("{}", commands());
    println!("  {} {}\t\t- Launch executable with the given [args]", Colour::Fixed(14).paint("exe"), Colour::Cyan.paint("[args]"));
    println!("  {} \t\t\t- Display this help message", Colour::Fixed(14).paint("help"));
    println!("  {}\t\t\t- Exit the debugger", Colour::Fixed(14).paint("exit"));
}