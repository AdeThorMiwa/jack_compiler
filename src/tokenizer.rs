use crate::Token;
use std::path::PathBuf;

pub struct Tokenizer;

impl Tokenizer {
    pub fn new(_source: &PathBuf) -> Self {
        Self
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
