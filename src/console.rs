//! Console I/O

use std::io;
use std::io::prelude::*;
use crate::conmark::*;

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

/// Formats the text using confmt/conwrap, and outputs it using print!()
#[allow(dead_code)]
pub fn puts(text: &str) {
    print!("{}", conwrap(&confmt(text)));
}

/// Formats the text using confmt/conwrap, and outputs it using println!()
#[allow(dead_code)]
pub fn putln(text: &str) {
    println!("{}", conwrap(&confmt(text)));
}

/// Outputs the text as a block paragraph, i.e., adds an extra newline.
/// This is the normal way to output text.
pub fn para(text: &str) {
    println!("{}\n", conwrap(&confmt(text)));
}

#[macro_export]
macro_rules! para {
    ($($arg:tt)*) => (
        para(&format!($($arg)*));
    )
}
