//! Utilities for testing.

/// Turns a `&str` to a `Vec<char>`
pub fn str_to_chars(string: &str) -> Vec<char> {
    string.chars().collect()
}

mod structs;

pub use structs::*;
