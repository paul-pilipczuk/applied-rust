//! This is a library that provides utilities for command-line tools.
//! So far it only provides a function to read a line from stdin.
//! #Examples:
//! ```
//! use lib_creation::read_stdin;
//! let input = read_stdin();
//! ```
//! # Panics:
//!  This function will panic if it fails to read the input line with a message "Failed to read input line".

use std::io::{BufRead, BufReader};

/// Reads a line from standard input and returns it as a `String`.
/// It will panic if it fails to read the input line with a message "Failed to read input line".
/// # Examples:
/// ```
/// let input = read_stdin();
/// ```
pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read input line");
    line.trim().to_string()
}