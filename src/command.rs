//! The Command System
/// This system is for parsing commands and preparing them for execution,
/// not for executing them.

use crate::world::World;

pub struct Command {
    // The original text of the command, as entered by the user
    pub input: String,

    // The simplified command tokens.  This may eventually become a
    // vector of some enum type.
    pub words: Vec<String>,

    // If true, this is a debugging command.
    pub is_debug: bool,
}

impl Command {
    fn new(input: &str, words: Vec<String>) -> Command {
        Command {
            input: input.into(),
            words,
            is_debug: input.starts_with('!'),
        }
    }
}

pub fn parse(_world: &World, input: &str) -> Result<Command,String> {
    // FIRST, remove extraneous characters.
    let input = input.trim();
    let mut text = String::new();

    for c in input.chars() {
        match c {
            ',' | '!' => {}
            '.' => {
                return Err("Input contains '.'; multiple commands not yet support.".into());
            }
            _ => text.push(c)
        }
    }

    // NEXT, split into words
    let words: Vec<String> = text
        .split_whitespace()
        .map(|token| token.to_string())
        .collect();

    // NEXT, handle verb synonyms
    // TODO

    // NEXT, return the result.
    Ok(Command::new(input, words))
}
