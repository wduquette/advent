//! # Console I/O
//! Create a Console to read input in "readline" fashion.  Use para() and the para!() macro
//! to output paragraphs of text.

use crate::conmark::*;

use rustyline::error::ReadlineError;
use rustyline::Editor;

/// A console input abstraction, wrapping the rustyline input processor.
pub struct Console {
    rusty: Editor<()>,
}

impl Console {
    /// Creates the console input abstraction.
    pub fn new() -> Console {
        Console {
            rusty: Editor::<()>::new(),
        }
    }

    /// Read a non-empty line from the console, using the given prompt.
    /// Ignores empty lines; halts on ^C, ^D.
    pub fn readline(&mut self, prompt: &str) -> String {
        loop {
            match self.rusty.readline(prompt) {
                Ok(line) => {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    } else {
                        self.rusty.add_history_entry(line);
                        return line.to_string();
                    }
                }
                Err(ReadlineError::Interrupted) => break,
                Err(ReadlineError::Eof) => break,
                Err(err) => {
                    println!("Input Error: {:?}", err);
                    continue;
                }
            }
        }

        ::std::process::exit(0);
    }
}

/// Outputs the text as a block paragraph, i.e., adds an extra newline.
/// This is the normal way to output text.
pub fn para(text: &str) {
    println!("{}\n", conwrap(&confmt(text)));
}

/// Formats its arguments using format!(), and outputs them as a wrapped
/// text block.  Uses `conmark` syntax for line breaks, etc.
#[macro_export]
macro_rules! para {
    ($($arg:tt)*) => (
        para(&format!($($arg)*));
    )
}
