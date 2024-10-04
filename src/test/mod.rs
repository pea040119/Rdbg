use ansi_term::Colour;
use crate::utils::constants::{error_header, success_header};

pub mod pea_test;



pub fn test(tester: Option<&String>){
    match tester {
        Some(tester) => {
            let tester_print: ansi_term::ANSIGenericString<'_, str> = Colour::Blue.paint(tester.as_str());
            match tester.as_str() {
                "pea" => {
                    println!("{}: Running the test for {} tester...", success_header(), tester_print);
                    pea_test::test();
                    println!("{}: {} test completed successfully.", success_header(), tester_print);
                },
                _ => {
                    eprintln!("{}: {} is not listed as a recognized tester.", error_header(), tester_print);
                }
            }
        },
        None => (),
    }
}