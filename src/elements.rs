pub mod lexical_elements {
    use std::str::FromStr;

    use anyhow::anyhow;

    #[derive(Debug, Clone)]
    pub enum Keywords {
        Class,
        Constructor,
        Function,
        Method,
        Field,
        Static,
        Var,
        Int,
        Char,
        Boolean,
        Void,
        True,
        False,
        Null,
        This,
        Let,
        Do,
        If,
        Else,
        While,
        Return,
    }

    impl FromStr for Keywords {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let v = match s {
                "class" => Self::Class,
                "constructor" => Self::Constructor,
                "function" => Self::Function,
                "method" => Self::Method,
                "field" => Self::Field,
                "static" => Self::Static,
                "var" => Self::Var,
                "int" => Self::Int,
                "char" => Self::Char,
                "boolean" => Self::Boolean,
                "void" => Self::Void,
                "true" => Self::True,
                "false" => Self::False,
                "null" => Self::Null,
                "this" => Self::This,
                "let" => Self::Let,
                "do" => Self::Do,
                "if" => Self::If,
                "else" => Self::Else,
                "while" => Self::While,
                "return" => Self::Return,
                _ => return Err(anyhow!("Invalid keyword")),
            };

            Ok(v)
        }
    }

    impl ToString for Keywords {
        fn to_string(&self) -> String {
            match self {
                Self::Class => "class".to_owned(),
                Self::Constructor => "constructor".to_owned(),
                Self::Function => "function".to_owned(),
                Self::Method => "method".to_owned(),
                Self::Field => "field".to_owned(),
                Self::Static => "static".to_owned(),
                Self::Var => "var".to_owned(),
                Self::Int => "int".to_owned(),
                Self::Char => "char".to_owned(),
                Self::Boolean => "boolean".to_owned(),
                Self::Void => "void".to_owned(),
                Self::True => "true".to_owned(),
                Self::False => "false".to_owned(),
                Self::Null => "null".to_owned(),
                Self::This => "this".to_owned(),
                Self::Let => "let".to_owned(),
                Self::Do => "do".to_owned(),
                Self::If => "if".to_owned(),
                Self::Else => "else".to_owned(),
                Self::While => "while".to_owned(),
                Self::Return => "return".to_owned(),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub enum Symbols {
        OpenCurlyBrace,
        CloseCurlyBrace,
        OpenBrace,
        CloseBrace,
        OpenSquareBrace,
        CloseSquareBrace,
        Dot,
        Comma,
        SemiColon,
        Plus,
        Minus,
        Asterik,
        BackSlash,
        Ampersand,
        VerticalBar,
        LessThan,
        GreaterThan,
        Equal,
        Tilde,
    }

    impl FromStr for Symbols {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let v = match s {
                "{" => Self::OpenCurlyBrace,
                "}" => Self::CloseCurlyBrace,
                "(" => Self::OpenBrace,
                ")" => Self::CloseBrace,
                "[" => Self::OpenSquareBrace,
                "]" => Self::CloseSquareBrace,
                "." => Self::Dot,
                "," => Self::Comma,
                ";" => Self::SemiColon,
                "+" => Self::Plus,
                "-" => Self::Minus,
                "*" => Self::Asterik,
                "/" => Self::BackSlash,
                "&" => Self::Ampersand,
                "|" => Self::VerticalBar,
                "<" => Self::LessThan,
                ">" => Self::GreaterThan,
                "=" => Self::Equal,
                "~" => Self::Tilde,
                _ => return Err(anyhow!("Invalid symbol")),
            };

            Ok(v)
        }
    }

    impl ToString for Symbols {
        fn to_string(&self) -> String {
            match self {
                Self::OpenCurlyBrace => "{".to_owned(),
                Self::CloseCurlyBrace => "}".to_owned(),
                Self::OpenBrace => "(".to_owned(),
                Self::CloseBrace => ")".to_owned(),
                Self::OpenSquareBrace => "[".to_owned(),
                Self::CloseSquareBrace => "]".to_owned(),
                Self::Dot => ".".to_owned(),
                Self::Comma => ",".to_owned(),
                Self::SemiColon => ";".to_owned(),
                Self::Plus => "+".to_owned(),
                Self::Minus => "-".to_owned(),
                Self::Asterik => "*".to_owned(),
                Self::BackSlash => "/".to_owned(),
                Self::Ampersand => "&".to_owned(),
                Self::VerticalBar => "|".to_owned(),
                Self::LessThan => "<".to_owned(),
                Self::GreaterThan => ">".to_owned(),
                Self::Equal => "=".to_owned(),
                Self::Tilde => "~".to_owned(),
            }
        }
    }
}
