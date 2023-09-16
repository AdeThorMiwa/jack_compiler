use std::{fs::File, io::Read, path::PathBuf, str::FromStr};

use anyhow::{anyhow, bail, Context, Result};

use crate::{
    lexical_elements::{Keywords, Symbols},
    Token,
};

pub struct StreamTokenizer {
    remaining_text: String,
    current_index: usize,
    iter_times: usize,
}

impl StreamTokenizer {
    pub fn new(source: &PathBuf) -> Self {
        let mut text = String::new();
        let _ = File::open(source).unwrap().read_to_string(&mut text);

        Self {
            remaining_text: text,
            current_index: 0,
            iter_times: 0,
        }
    }

    fn next_token(&mut self) -> Result<Token> {
        let (tok, bytes_read) = Self::tokenize_single_token(&self.remaining_text)?;
        self.chomp(bytes_read);

        Ok(tok)
    }

    fn chomp(&mut self, num_bytes: usize) {
        self.remaining_text = self.remaining_text[num_bytes..].to_owned();
        self.current_index += num_bytes;
    }

    fn tokenize_single_token(data: &str) -> Result<(Token, usize)> {
        let next = match data.chars().next() {
            Some(c) => c,
            None => bail!("EOF"),
        };

        let (tok, length) = match next {
            symbol if Symbols::from_str(symbol.to_string().as_str()).is_ok() => (
                Token::Symbol(Symbols::from_str(symbol.to_string().as_str()).unwrap()),
                1,
            ),
            '0'..='9' => Self::tokenize_digit(data).context("couldn't tokenize a number")?,
            '"' => {
                Self::tokenize_string_literal(data).context("couldnt tokenize string literal")?
            }
            c @ '_' | c if c.is_alphabetic() => Self::tokenize_ident_or_keyword(data)
                .context("couldnt tokenize an identifier/keyword")?,
            _ => bail!("unknown character"),
        };

        Ok((tok, length))
    }

    fn skip_whitespace(&mut self) {
        let skipped = Self::skip(&self.remaining_text);
        self.chomp(skipped);
    }

    fn skip_comments(src: &str) -> usize {
        let pairs = [("//", "\n"), ("/*", "*/")];

        for &(pattern, matcher) in &pairs {
            if src.starts_with(pattern) {
                let leftovers = Self::skip_until(src, matcher);
                return src.len() - leftovers.len();
            }
        }

        0
    }

    fn skip_until<'a>(mut src: &'a str, pattern: &str) -> &'a str {
        while !src.is_empty() && !src.starts_with(pattern) {
            let next_char_size = src
                .chars()
                .next()
                .expect("The string isn't empty")
                .len_utf8();
            src = &src[next_char_size..];
        }

        &src[pattern.len()..]
    }

    fn skip(src: &str) -> usize {
        let mut remaining = src;

        loop {
            let ws = Self::_skip_whitespace(remaining);
            remaining = &remaining[ws..];
            let comments = Self::skip_comments(remaining);
            remaining = &remaining[comments..];

            if ws + comments == 0 {
                return src.len() - remaining.len();
            }
        }
    }

    fn tokenize_ident_or_keyword(data: &str) -> Result<(Token, usize)> {
        match data.chars().next() {
            Some(ch) if ch.is_digit(10) => bail!("Identifiers can't start with a number"),
            None => bail!("EOF"),
            _ => {}
        }

        let (got, bytes_read) = Self::take_while(data, |ch| ch == '_' || ch.is_alphanumeric())?;

        let token = match got {
            s if Keywords::from_str(s).is_ok() => Token::Keyword(Keywords::from_str(s)?),
            _ => Token::Identifier(got.to_string()),
        };

        Ok((token, bytes_read))
    }

    fn tokenize_digit(data: &str) -> Result<(Token, usize)> {
        let (digit, bytes_read) =
            Self::take_while(data, |c| if c.is_digit(10) { true } else { false })?;

        let n: i16 = digit.parse()?;
        Ok((Token::IntConst(n), bytes_read))
    }

    fn tokenize_string_literal(data: &str) -> Result<(Token, usize)> {
        if data.chars().next() != Some('"') {
            return Err(anyhow!("Invalid string literal"));
        }

        let mut quotes = 0;
        let (got, bytes_read) = Self::take_while(data, |ch| {
            if quotes >= 2 {
                return false;
            }

            if ch == '"' {
                quotes += 1;
            }

            true
        })?;

        let token = Token::StringConst(got.replace(r#"""#, ""));

        Ok((token, bytes_read))
    }

    fn take_while<F>(data: &str, mut pred: F) -> Result<(&str, usize)>
    where
        F: FnMut(char) -> bool,
    {
        let mut current_index = 0;

        for ch in data.chars() {
            let should_continue = pred(ch);

            if !should_continue {
                break;
            }

            current_index += ch.len_utf8();
        }

        if current_index == 0 {
            Err(anyhow!("No Matches"))
        } else {
            Ok((&data[..current_index], current_index))
        }
    }

    fn _skip_whitespace(data: &str) -> usize {
        match Self::take_while(data, |ch| ch.is_whitespace()) {
            Ok((_, bytes_skipped)) => bytes_skipped,
            _ => 0,
        }
    }
}

impl Iterator for StreamTokenizer {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();

        self.iter_times += 1;
        if self.remaining_text.is_empty() || self.iter_times >= 1000 {
            None
        } else {
            let token = self.next_token();
            Some(token)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{lexical_elements::Keywords, StreamTokenizer, Token};

    #[test]
    fn tokenize_a_single_letter() {
        let src = "F";
        let should_be = Token::Identifier(src.to_string());

        let (got, _bytes_read) = StreamTokenizer::tokenize_ident_or_keyword(src).unwrap();
        assert_eq!(got, should_be, "Input was {:?}", src);
    }

    #[test]
    fn tokenize_an_identifer() {
        let src = "Foo";
        let should_be = Token::Identifier(src.to_string());

        let (got, _bytes_read) = StreamTokenizer::tokenize_ident_or_keyword(src).unwrap();
        assert_eq!(got, should_be, "Input was {:?}", src);
    }

    #[test]
    fn tokenize_ident_containing_an_underscore() {
        let src = "Foo_bar";
        let should_be = Token::Identifier(src.to_string());

        let (got, _bytes_read) = StreamTokenizer::tokenize_ident_or_keyword(src).unwrap();
        assert_eq!(got, should_be, "Input was {:?}", src);
    }

    #[test]
    fn tokenize_keyword() {
        let src = "class";
        let should_be = Token::Keyword(Keywords::from_str(src).unwrap());

        let (got, _bytes_read) = StreamTokenizer::tokenize_ident_or_keyword(src).unwrap();
        assert_eq!(got, should_be, "Input was {:?}", src);
    }

    #[test]
    fn tokenize_identifier_starting_with_keyword() {
        let src = "classifier";
        let should_be = Token::Identifier(src.to_string());

        let (got, _bytes_read) = StreamTokenizer::tokenize_ident_or_keyword(src).unwrap();
        assert_eq!(got, should_be, "Input was {:?}", src);
    }

    #[test]
    fn tokenize_ident_cant_start_with_number() {
        let src = "7Foo_bar";

        let got = StreamTokenizer::tokenize_ident_or_keyword(src);
        assert!(got.is_err(), "{:?} should be an error", got);
    }

    #[test]
    fn tokenize_ident_cant_start_with_dot() {
        let src = ".Foo_bar";

        let got = StreamTokenizer::tokenize_ident_or_keyword(src);
        assert!(got.is_err(), "{:?} should be an error", got);
    }

    #[test]
    fn tokenize_string_literal() {
        let src = r#""some string" some other weird stuff"#;
        let should_be = Token::StringConst("some string".to_string());

        let (got, _) = StreamTokenizer::tokenize_string_literal(src).unwrap();
        assert_eq!(got, should_be, "Input was {:?}", src);
    }

    #[test]
    fn tokenize_digit() {
        let src = "1000";
        let should_be = Token::IntConst(src.parse::<i16>().unwrap());

        let (got, _bytes_read) = StreamTokenizer::tokenize_digit(src).unwrap();
        assert_eq!(got, should_be, "Input was {:?}", src);
    }

    #[test]
    fn skip_past_several_whitespace_chars() {
        let src = " \t\n\r123";
        let should_be = 4;

        let num_skipped = StreamTokenizer::_skip_whitespace(src);
        assert_eq!(num_skipped, should_be);
    }

    #[test]
    fn skipping_whitespace_when_first_is_a_letter_returns_zero() {
        let src = "Hello World";
        let should_be = 0;

        let num_skipped = StreamTokenizer::_skip_whitespace(src);
        assert_eq!(num_skipped, should_be);
    }

    #[test]
    fn slash_slash_skips_to_end_of_line() {
        let src = "// foo bar { baz }\n 1234";
        let got = StreamTokenizer::skip_comments(src);
        assert_eq!(got, 19)
    }

    #[test]
    fn comment_skip_multi_line_comment() {
        let src = "/** foo bar { baz } */ 1234";
        let got = StreamTokenizer::skip_comments(src);
        assert_eq!(got, 22)
    }

    #[test]
    fn comment_skip_ignores_alphanumeric() {
        let src = "123 hello world";
        let got = StreamTokenizer::skip_comments(src);
        assert_eq!(got, 0)
    }

    #[test]
    fn comment_skip_ignores_whitespace() {
        let src = "   /* */ 123 hello world";
        let got = StreamTokenizer::skip_comments(src);
        assert_eq!(got, 0)
    }

    #[test]
    fn central_tokenizer_integer() {
        let src = "1234";
        let should_be = Token::IntConst(1234);

        let (got, _bytes_read) = StreamTokenizer::tokenize_single_token(src).unwrap();
        assert_eq!(got, should_be, "Input was {:?}", src);
    }

    #[test]
    fn central_tokenizer_open_curly_brace() {
        let src = "{";
        let should_be = Token::Symbol(crate::lexical_elements::Symbols::OpenCurlyBrace);

        let (got, _bytes_read) = StreamTokenizer::tokenize_single_token(src).unwrap();
        assert_eq!(got, should_be, "Input was {:?}", src);
    }

    #[test]
    fn central_tokenizer_close_curly_brace() {
        let src = "}";
        let should_be = Token::Symbol(crate::lexical_elements::Symbols::CloseCurlyBrace);

        let (got, _bytes_read) = StreamTokenizer::tokenize_single_token(src).unwrap();
        assert_eq!(got, should_be, "Input was {:?}", src);
    }

    #[test]
    fn central_tokenizer_open_brace() {
        let src = "(";
        let should_be = Token::Symbol(crate::lexical_elements::Symbols::OpenBrace);

        let (got, _bytes_read) = StreamTokenizer::tokenize_single_token(src).unwrap();
        assert_eq!(got, should_be, "Input was {:?}", src);
    }

    #[test]
    fn central_tokenizer_close_brace() {
        let src = ")";
        let should_be = Token::Symbol(crate::lexical_elements::Symbols::CloseBrace);

        let (got, _bytes_read) = StreamTokenizer::tokenize_single_token(src).unwrap();
        assert_eq!(got, should_be, "Input was {:?}", src);
    }

    #[test]
    fn central_tokenizer_open_square_brace() {
        let src = "[";
        let should_be = Token::Symbol(crate::lexical_elements::Symbols::OpenSquareBrace);

        let (got, _bytes_read) = StreamTokenizer::tokenize_single_token(src).unwrap();
        assert_eq!(got, should_be, "Input was {:?}", src);
    }

    #[test]
    fn central_tokenizer_close_square_brace() {
        let src = "]";
        let should_be = Token::Symbol(crate::lexical_elements::Symbols::CloseSquareBrace);

        let (got, _bytes_read) = StreamTokenizer::tokenize_single_token(src).unwrap();
        assert_eq!(got, should_be, "Input was {:?}", src);
    }

    #[test]
    fn central_tokenizer_dot() {
        let src = ".";
        let should_be = Token::Symbol(crate::lexical_elements::Symbols::Dot);

        let (got, _bytes_read) = StreamTokenizer::tokenize_single_token(src).unwrap();
        assert_eq!(got, should_be, "Input was {:?}", src);
    }

    #[test]
    fn central_tokenizer_comma() {
        let src = ",";
        let should_be = Token::Symbol(crate::lexical_elements::Symbols::Comma);

        let (got, _bytes_read) = StreamTokenizer::tokenize_single_token(src).unwrap();
        assert_eq!(got, should_be, "Input was {:?}", src);
    }

    #[test]
    fn central_tokenizer_semicolon() {
        let src = ";";
        let should_be = Token::Symbol(crate::lexical_elements::Symbols::SemiColon);

        let (got, _bytes_read) = StreamTokenizer::tokenize_single_token(src).unwrap();
        assert_eq!(got, should_be, "Input was {:?}", src);
    }

    #[test]
    fn central_tokenizer_plus() {
        let src = "+";
        let should_be = Token::Symbol(crate::lexical_elements::Symbols::Plus);

        let (got, _bytes_read) = StreamTokenizer::tokenize_single_token(src).unwrap();
        assert_eq!(got, should_be, "Input was {:?}", src);
    }

    #[test]
    fn central_tokenizer_minus() {
        let src = "-";
        let should_be = Token::Symbol(crate::lexical_elements::Symbols::Minus);

        let (got, _bytes_read) = StreamTokenizer::tokenize_single_token(src).unwrap();
        assert_eq!(got, should_be, "Input was {:?}", src);
    }

    #[test]
    fn central_tokenizer_asterik() {
        let src = "*";
        let should_be = Token::Symbol(crate::lexical_elements::Symbols::Asterik);

        let (got, _bytes_read) = StreamTokenizer::tokenize_single_token(src).unwrap();
        assert_eq!(got, should_be, "Input was {:?}", src);
    }

    #[test]
    fn central_tokenizer_backslash() {
        let src = "/";
        let should_be = Token::Symbol(crate::lexical_elements::Symbols::BackSlash);

        let (got, _bytes_read) = StreamTokenizer::tokenize_single_token(src).unwrap();
        assert_eq!(got, should_be, "Input was {:?}", src);
    }

    #[test]
    fn central_tokenizer_ampersand() {
        let src = "&";
        let should_be = Token::Symbol(crate::lexical_elements::Symbols::Ampersand);

        let (got, _bytes_read) = StreamTokenizer::tokenize_single_token(src).unwrap();
        assert_eq!(got, should_be, "Input was {:?}", src);
    }

    #[test]
    fn central_tokenizer_vertical_bar() {
        let src = "|";
        let should_be = Token::Symbol(crate::lexical_elements::Symbols::VerticalBar);

        let (got, _bytes_read) = StreamTokenizer::tokenize_single_token(src).unwrap();
        assert_eq!(got, should_be, "Input was {:?}", src);
    }

    #[test]
    fn central_tokenizer_less_than() {
        let src = "<";
        let should_be = Token::Symbol(crate::lexical_elements::Symbols::LessThan);

        let (got, _bytes_read) = StreamTokenizer::tokenize_single_token(src).unwrap();
        assert_eq!(got, should_be, "Input was {:?}", src);
    }

    #[test]
    fn central_tokenizer_greater_than() {
        let src = ">";
        let should_be = Token::Symbol(crate::lexical_elements::Symbols::GreaterThan);

        let (got, _bytes_read) = StreamTokenizer::tokenize_single_token(src).unwrap();
        assert_eq!(got, should_be, "Input was {:?}", src);
    }

    #[test]
    fn central_tokenizer_equal() {
        let src = "=";
        let should_be = Token::Symbol(crate::lexical_elements::Symbols::Equal);

        let (got, _bytes_read) = StreamTokenizer::tokenize_single_token(src).unwrap();
        assert_eq!(got, should_be, "Input was {:?}", src);
    }

    #[test]
    fn central_tokenizer_tilde() {
        let src = "~";
        let should_be = Token::Symbol(crate::lexical_elements::Symbols::Tilde);

        let (got, _bytes_read) = StreamTokenizer::tokenize_single_token(src).unwrap();
        assert_eq!(got, should_be, "Input was {:?}", src);
    }
}
