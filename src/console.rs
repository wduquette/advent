//! Console I/O

use std::io;
use std::io::prelude::*;

/// Outputs the standard command prompt to stdout.
pub fn prompt(prompt: &str) {
    print!("{} ", prompt);
    io::stdout().flush().expect("Could not flush stdout");
}

/// Reads a trimmed, lowercase line of input from stdin and returns
/// it.
pub fn get_line() -> String {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    guess.trim().to_lowercase()
}

/// Prompts the user to enter a command.  Empty commands are ignored.
pub fn get_command(prompt_text: &str) -> String {
    let mut command;
    loop {
        prompt(prompt_text);

        command = get_line();

        if !command.is_empty() {
            break;
        }
    }
    command
}
