use std::env;

fn print_help(err: Option<String>) {
    if let Some(msg) = err {
        println!("{}", msg)
    }
    println!(
        "usage: loxide <string>
    Interprets the string as lox, and runs the resultant program"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => print_help(None),
        2 => println!("Run"),
        _ => print_help(Some(format!("Incorrect number of arguments"))),
    }
}
