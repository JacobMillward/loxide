use std::{env, error::Error};

use loxide::frontend::{run_file, run_interactive};

fn print_help() {
    println!(
        "usage: loxide [script]
    Run the Loxide interpreter in interactive mode if no script is provided."
    );
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => Ok(run_interactive()?),
        2 => Ok(run_file(&args[1])?),
        _ => {
            print_help();
            Err("Incorrect number of arguments.")?
        }
    }
}
