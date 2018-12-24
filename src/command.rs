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

pub fn parse(world: &World, input: &str) -> Result<Command,String> {
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
    let raw_words: Vec<&str> = text
        .split_whitespace()
        .collect();

    // NEXT, strip articles and translate synonyms.
    let mut words: Vec<String> = Vec::new();

    for word in raw_words {
        match word {
            "a" | "an" | "the" => (),
            _ => {
                if let Some(canon) = world.synonyms.get(word) {
                    words.push(canon.to_string());
                } else {
                    words.push(word.to_string());
                }
            }
        }
    }

    // NEXT, return the result.
    Ok(Command::new(input, words))
}
