//! Add a ShellError enum
//!
//! To be rewritten, for now based on csv-parser by Guillaume Gomez which is MIT licence.
//! See https://github.com/GuillaumeGomez/csv-parser

use std::cmp::PartialEq;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ShellError {
    InvalidCharacter(CharError),
    InvalidRowLength(SizeError),
    GenericError
}

impl fmt::Display for ShellError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ShellError::InvalidCharacter(ref c) => write!(f, "InvalidCharacter: {}: expected `{}`, got `{}`",
                                                        c.position, c.expected, c.got),
            ShellError::InvalidRowLength(ref c) => write!(f, "InvalidRowLength: {}: expected `{}` element{}, got `{}`",
                                                        c.position, c.nb_elements_expected,
                                                        if c.nb_elements_expected > 1 { "s" } else { "" }, c.nb_elements_got),
            ShellError::GenericError            => write!(f, "GenericError")
        }
    }
}

impl PartialEq for ShellError {
    fn eq(&self, other: &ShellError) -> bool {
        match (self, other) {
            (&ShellError::InvalidCharacter(ref c), &ShellError::InvalidCharacter(ref o)) => c == o,
            (&ShellError::InvalidRowLength(ref c), &ShellError::InvalidRowLength(ref o)) => c == o,
            (&ShellError::GenericError, &ShellError::GenericError)                       => true,
            _ => false,
        }
    }

    fn ne(&self, other: &ShellError) -> bool {
        self.eq(other) == false
    }
}

impl Error for ShellError {
    fn description(&self) -> &str {
        match *self {
            ShellError::InvalidCharacter(_) => "invalid character",
            ShellError::InvalidRowLength(_) => "invalid row length",
            ShellError::GenericError        => "generic parsing error"
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl Position {
    pub fn new(line: usize, column: usize) -> Position {
        Position {
            line: line,
            column: column,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CharError {
    pub expected: char,
    pub got: char,
    pub position: Position,
}

impl CharError {
    pub fn new(expected: char, got: char, pos: &Position) -> CharError {
        CharError {
            expected: expected,
            got: got,
            position: pos.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SizeError {
    pub nb_elements_expected: usize,
    pub nb_elements_got: usize,
    pub position: Position,
}

impl SizeError {
    pub fn new(nb_elements_expected: usize, nb_elements_got: usize,
               pos: &Position) -> SizeError {
        SizeError {
            nb_elements_expected: nb_elements_expected,
            nb_elements_got: nb_elements_got,
            position: pos.clone(),
        }
    }
}

