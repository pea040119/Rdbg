#![allow(unused)]

use clap::{Command, Arg};

mod test;
mod utils;
pub mod dbg {
    pub mod pea_dbg;
    pub mod mangocubes_dbg;
}



fn main() {
    let args: clap::ArgMatches = Command::new("Rgdb")
        .arg(
            Arg::new("test")
                .long("test")
                .short('t')
                .help("Input Tester Name")
                .num_args(1)
        )
        .get_matches();
    
    test::test(args.get_one::<String>("test"));
}