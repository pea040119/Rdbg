use ansi_term::Colour;
use crate::utils::constants::{error_header, success_header};

pub mod pea_test;
pub mod mangocubes_test;



pub fn test(tester: Option<&String>){
    match tester {
        Some(tester) => {
            let tester_print: ansi_term::ANSIGenericString<'_, str> = Colour::Blue.paint(tester.as_str());
            match tester.as_str() {
                "pea" => {
                    println!("{}: Running the {}'s test...", success_header(), tester_print);
                    pea_test::test();
                    println!("{}: {}'s test completed successfully.", success_header(), tester_print);
                },
                "MangoCubes" => {
                    println!("{}: Running the {}'s test...", success_header(), tester_print);
                    mangocubes_test::test();
                    println!("{}: {}'s test completed successfully.", success_header(), tester_print);
                },
                _ => {
                    eprintln!("{}: {} is not listed as a recognized tester.", error_header(), tester_print);
                }
            }
        },
        None => (),
    }
}