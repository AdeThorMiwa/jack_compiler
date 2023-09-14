use crate::{
    lexical_elements::{Identifier, Keywords, Symbols},
    Token,
};
use std::{fs::File, io::BufRead, io::BufReader, path::PathBuf, str::FromStr};

pub struct Tokenizer {
    tokens: Vec<Token>,
    i: usize,
}

impl Tokenizer {
    pub fn new(source: &PathBuf) -> Self {
        let mut tokens = Vec::new();

        // read line by line
        let file = File::open(source).unwrap();
        let file_buffer = BufReader::new(file);

        // method 1 - loop through line - whenever we encounter a symbol we add space both sides
        // then we split on space

        for line in file_buffer.lines() {
            // trim whitespace and comments
            let line = Self::strip_comments(line.unwrap().trim());

            // if line remains an empty string goto next line
            if line.is_empty() {
                continue;
            }

            let mut processed_line = String::new();
            let mut string_const = Vec::new();
            for c in line.chars() {
                if c == '"' {
                    if string_const.is_empty() {
                        string_const.push('"'.to_string());
                        continue;
                    } else {
                        processed_line.push_str(&string_const.join("").replace(" ", "_"));
                        string_const.clear();
                        continue;
                    }
                }

                if !string_const.is_empty() {
                    string_const.push(c.to_string());
                    continue;
                }

                if Symbols::from_str(&c.to_string()).is_ok() {
                    processed_line.push_str(&format!(" {} ", c));
                    continue;
                } else {
                    processed_line.push(c);
                }
            }

            let line = processed_line;

            // else read each char on line and match with language lexicon
            for t in line.split(" ") {
                let t = t.trim();
                if t.is_empty() {
                    continue;
                }

                let token = Keywords::from_str(t);
                if token.is_ok() {
                    tokens.push(Token::Keyword(token.unwrap()));
                    continue;
                }

                let token = Symbols::from_str(t);
                if token.is_ok() {
                    tokens.push(Token::Symbol(token.unwrap()));
                    continue;
                }

                let token = t.parse::<i16>();
                if token.is_ok() {
                    tokens.push(Token::IntConst(token.unwrap()));
                    continue;
                }

                if t.chars().next() == Some('"') {
                    let token = t.to_string().replace("_", " ");
                    tokens.push(Token::StringConst(token.replace("\"", "")));
                    continue;
                }

                let token = Identifier::from_str(t).unwrap();
                tokens.push(Token::Identifier(token));
            }
        }

        Self { tokens, i: 0 }
    }

    fn strip_comments(str: &str) -> String {
        str.splitn(2, "//")
            .next()
            .unwrap()
            .trim()
            .splitn(2, "/*")
            .next()
            .unwrap()
            .trim()
            .to_owned()
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.tokens.iter().nth(self.i);
        self.i += 1;
        item.map(|i| i.clone())
    }
}
