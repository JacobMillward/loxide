use super::run;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

pub fn run_interactive() -> Result<()> {
    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline("lox > ");

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                run(&line);
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("Exiting...");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
