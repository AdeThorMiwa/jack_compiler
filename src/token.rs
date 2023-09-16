use crate::lexical_elements::{Keywords, Symbols};

#[derive(Debug, Clone)]
pub enum Token {
    Keyword(Keywords),
    Symbol(Symbols),
    Identifier(String),
    IntConst(i16),
    StringConst(String),
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Self::Keyword(k) => format!("Keyword({})", k.to_string()),
            Self::Symbol(s) => format!("Symbol({})", s.to_string()),
            Self::Identifier(id) => format!("Identifier({})", id.to_string()),
            Self::IntConst(i) => format!("IntConst({})", i),
            Self::StringConst(s) => format!("StringConst({})", s),
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}
