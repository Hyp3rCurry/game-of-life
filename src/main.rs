mod double;
mod single;

use std::env;
use std::process;
use crate::double::main_double;
use crate::single::main_single;

fn main() {
    let mode: String = get_mode(env::args()).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);});

    if mode == "single" {
        main_single();
    }
    else if mode == "double" {
        main_double();
    }
    else {
        eprintln!("Invalid Mode entered");
        process::exit(1);
    }
}




fn get_mode(mut args: impl Iterator<Item = String>) -> Result<String, &'static str>{
    let _query = args.next();

    return match args.next() {
        Some(t) => Ok(t),
        None => Err("Error: No Mode entered"),
    };
}