use crate::lexical_elements::{self, Keywords, Symbols};

#[derive(Debug, Clone)]
pub enum Token {
    Keyword(Keywords),
    Symbol(Symbols),
    Identifier(lexical_elements::Identifier),
    IntConst(i16),
    StringConst(String),
}
