use ctrlc;
use std::{
    io::{self, Write},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, TryRecvError},
        Arc,
    },
    thread,
};

use super::run;

pub fn run_interactive() {
    let has_quit = Arc::new(AtomicBool::new(false));

    let handle_quit = has_quit.clone();
    ctrlc::set_handler(move || {
        println!();
        handle_quit.store(true, Ordering::Relaxed)
    })
    .expect("Error setting Ctrl-C handler");

    let stdin_channel = spawn_stdin_channel();
    print_prompt();

    loop {
        match stdin_channel.try_recv() {
            Ok(line) => process_line(line, has_quit.clone()),
            Err(TryRecvError::Empty) => {
                if has_quit.load(Ordering::Relaxed) {
                    println!("Exiting...");
                    break;
                }
            }
            Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
        }
    }
}

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        tx.send(buffer).unwrap();
    });
    rx
}

fn print_prompt() {
    print!("lox > ");
    io::stdout().flush().unwrap();
}

fn process_line(line: String, handle_quit: Arc<AtomicBool>) {
    if line.trim() == "exit" {
        handle_quit.store(true, Ordering::Relaxed);
        return;
    }

    run(&line);
    print_prompt();
}
