use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::Command;
use std::{env, io};

use self::commands::process_line;

mod commands;

fn main() {
    loop {
        eprint!("$ ");
        io::stderr().flush().expect("command should be valid");
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => process_line(&input),
            Err(e) => {
                eprintln!("error: reading input: {}", e);
                break;
            }
        }
    }
}
