pub mod lexical_elements {
    use std::str::FromStr;

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
        type Err = ();

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
                _ => return Err(()),
            };

            Ok(v)
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
        OrPipe,
        LessThan,
        GreaterThan,
        Equal,
        Tilde,
    }

    impl FromStr for Symbols {
        type Err = ();

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
                "|" => Self::OrPipe,
                "<" => Self::LessThan,
                ">" => Self::GreaterThan,
                "=" => Self::Equal,
                "~" => Self::Tilde,
                _ => return Err(()),
            };

            Ok(v)
        }
    }

    #[derive(Debug, Clone)]
    pub struct Identifier(String);

    impl FromStr for Identifier {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if s.chars().next().unwrap().is_numeric() {
                return Err(());
            }

            Ok(Self(s.to_owned()))
        }
    }
}
