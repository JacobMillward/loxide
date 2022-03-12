use loxide::run_file;
use std::env;

fn print_help() {
    println!(
        "usage: loxide <script>
    Interprets and runs the passed path as a lox script"
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            print_help();
            Err("No path specified.")?
        }
        2 => Ok(run_file(&args[1])?),
        _ => {
            print_help();
            Err("Incorrect number of arguments.")?
        }
    }
}
