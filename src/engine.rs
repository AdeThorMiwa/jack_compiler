use crate::{
    lexical_elements::{Keywords, Symbols},
    Token,
};
use anyhow::{anyhow, bail, Context, Result};
use peekmore::{PeekMore, PeekMoreIterator};
use std::io::Write;

pub struct CompilationEngine<'a, T: Iterator<Item = Result<Token>>> {
    writer: &'a mut dyn Write,
    tokenizer: PeekMoreIterator<&'a mut T>,
}

impl<'a, T: Iterator<Item = Result<Token>>> CompilationEngine<'a, T> {
    pub fn new<W: Write>(writer: &'a mut W, tokenizer: &'a mut T) -> Self {
        let peekable = tokenizer.peekmore();
        Self {
            writer,
            tokenizer: peekable,
        }
    }

    fn write_tagged(&mut self, token_name: &str, value: &str) {
        self.write_opening_tag(token_name);
        self.write(value);
        self.write_closing_tag(token_name);
    }

    pub fn compile(&mut self) -> Result<()> {
        self.write_class()?;
        Ok(())
    }

    fn write_opening_tag(&mut self, tag_name: &str) {
        self.write(&format!("\n<{}> ", tag_name))
    }

    fn write_closing_tag(&mut self, tag_name: &str) {
        self.write(&format!(" </{}>\n", tag_name))
    }

    fn write(&mut self, value: &str) {
        write!(&mut self.writer, "{}", value).unwrap()
    }

    fn write_class(&mut self) -> Result<()> {
        self.write_opening_tag("class");
        self.write_keyword(&Keywords::Class)?;
        self.write_identifier()?;
        self.write_symbol(Symbols::OpenCurlyBrace)?;

        loop {
            if self.write_class_var_dec().is_err() {
                break;
            }
        }

        loop {
            if self.write_subroutine_dec().is_err() {
                break;
            }
        }

        self.write_symbol(Symbols::CloseCurlyBrace)?;
        self.write_closing_tag("class");
        Ok(())
    }

    fn write_class_var_dec(&mut self) -> Result<()> {
        self.write_opening_tag("classVarDec");
        let is_static = match self.tokenizer.peek() {
            Some(Ok(Token::Keyword(k))) if k == &Keywords::Static => true,
            _ => false,
        };

        if is_static {
            self.write_keyword(&Keywords::Static)?;
        } else {
            let is_field = match self.tokenizer.peek() {
                Some(Ok(Token::Keyword(k))) if k == &Keywords::Field => true,
                _ => false,
            };

            if is_field {
                self.write_keyword(&Keywords::Field)?;
            } else {
                self.write_closing_tag("classVarDec");
                bail!("Invalid class variable declaration")
            }
        }

        self.write_type()?;
        self.write_var_name()?;

        loop {
            let has_more_param = match self.tokenizer.peek() {
                Some(Ok(Token::Symbol(s))) if s != &Symbols::SemiColon => true,
                _ => false,
            };

            if !has_more_param {
                break;
            }

            self.write_symbol(Symbols::Comma)?;
            self.write_var_name()?;
        }

        self.write_symbol(Symbols::SemiColon)?;
        self.write_closing_tag("classVarDec");
        Ok(())
    }

    fn write_subroutine_dec(&mut self) -> Result<()> {
        self.write_opening_tag("subroutineDec");
        let is_constructor = match self.tokenizer.peek() {
            Some(Ok(Token::Keyword(k))) if k == &Keywords::Constructor => true,
            _ => false,
        };

        if is_constructor {
            self.write_keyword(&Keywords::Constructor)?;
        } else {
            let is_method = match self.tokenizer.peek() {
                Some(Ok(Token::Keyword(k))) if k == &Keywords::Method => true,
                _ => false,
            };

            if is_method {
                self.write_keyword(&Keywords::Method)?;
            } else {
                let is_function = match self.tokenizer.peek() {
                    Some(Ok(Token::Keyword(k))) if k == &Keywords::Function => true,
                    _ => false,
                };

                if is_function {
                    self.write_keyword(&Keywords::Function)?;
                } else {
                    self.write_closing_tag("subroutineDec");
                    bail!("Invalid subroutine")
                }
            }
        }

        if let Some(Ok(t)) = self.tokenizer.peek() {
            match t {
                Token::Keyword(k) if k == &Keywords::Void => self.write_keyword(&Keywords::Void)?,
                _ => self.write_type()?,
            }
        }

        self.write_subroutine_name()?;
        self.write_symbol(Symbols::OpenBrace)?;
        self.write_parameter_list()?;
        self.write_symbol(Symbols::CloseBrace)?;
        self.write_subroutine_body()?;
        self.write_closing_tag("subroutineDec");
        Ok(())
    }

    fn write_parameter_list(&mut self) -> Result<()> {
        self.write_opening_tag("parameterList");
        if let Some(Ok(Token::Symbol(s))) = self.tokenizer.peek() {
            if s != &Symbols::CloseBrace {
                self.write_type()?;
                self.write_var_name()?;
            } else {
                self.write_closing_tag("parameterList");
                return Ok(());
            }
        }

        loop {
            let has_more_param = match self.tokenizer.peek() {
                Some(Ok(Token::Symbol(s))) if s != &Symbols::CloseBrace => true,
                _ => false,
            };

            if !has_more_param {
                break;
            }

            self.write_symbol(Symbols::Comma)?;
            self.write_type()?;
            self.write_var_name()?;
        }

        self.write_closing_tag("parameterList");
        Ok(())
    }

    fn write_subroutine_body(&mut self) -> Result<()> {
        self.write_opening_tag("subroutineBody");
        self.write_symbol(Symbols::OpenCurlyBrace)?;

        loop {
            let has_more_var_declaration = match self.tokenizer.peek() {
                Some(Ok(Token::Keyword(k))) if k == &Keywords::Var => true,
                _ => false,
            };

            if !has_more_var_declaration {
                break;
            }

            self.write_var_dec()?;
        }

        self.write_statements()?;
        self.write_symbol(Symbols::CloseCurlyBrace)?;
        self.write_closing_tag("subroutineBody");
        Ok(())
    }

    fn write_var_dec(&mut self) -> Result<()> {
        self.write_opening_tag("varDec");
        self.write_keyword(&Keywords::Var)?;
        self.write_type()?;

        loop {
            let has_more_var_declaration = match self.tokenizer.peek() {
                Some(Ok(Token::Symbol(s))) if s == &Symbols::SemiColon => false,
                _ => true,
            };

            if !has_more_var_declaration {
                break;
            }

            self.write_var_name()?;

            match self.tokenizer.peek() {
                Some(Ok(Token::Symbol(s))) if s == &Symbols::Comma => {
                    self.write_symbol(Symbols::Comma)?;
                }
                _ => {}
            }
        }

        self.write_symbol(Symbols::SemiColon)?;
        self.write_closing_tag("varDec");

        Ok(())
    }

    fn write_type(&mut self) -> Result<()> {
        if let Some(Ok(token)) = self.tokenizer.peek() {
            match token {
                Token::Keyword(k) if k == &Keywords::Int => {
                    self.write_keyword(&Keywords::Int)?;
                }
                Token::Keyword(k) if k == &Keywords::Char => {
                    self.write_keyword(&Keywords::Char)?;
                }
                Token::Keyword(k) if k == &Keywords::Boolean => {
                    self.write_keyword(&Keywords::Boolean)?;
                }
                Token::Identifier(_) => self.write_identifier()?,
                _ => {
                    return Err(anyhow!("invalid type")).with_context(|| {
                        format!("type `{}` is not a valid type", token.to_string())
                    })
                }
            }
        }

        Ok(())
    }

    fn write_statements(&mut self) -> Result<()> {
        self.write_opening_tag("statements");
        loop {
            if self.write_statement().is_err() {
                break;
            }
        }
        self.write_closing_tag("statements");
        Ok(())
    }

    fn write_statement(&mut self) -> Result<()> {
        if let Some(Ok(token)) = self.tokenizer.peek() {
            match token {
                Token::Keyword(k) if k == &Keywords::Let => self.write_let_statement()?,
                Token::Keyword(k) if k == &Keywords::If => self.write_if_statement()?,
                Token::Keyword(k) if k == &Keywords::While => self.write_while_statement()?,
                Token::Keyword(k) if k == &Keywords::Do => self.write_do_statement()?,
                Token::Keyword(k) if k == &Keywords::Return => self.write_return_statement()?,
                token => {
                    return Err(anyhow!("invalid statement")).with_context(|| {
                        format!(
                            "`{}` is not valid at this position to be statement",
                            token.to_string()
                        )
                    })
                }
            }
        }

        Ok(())
    }

    fn write_let_statement(&mut self) -> Result<()> {
        self.write_opening_tag("letStatement");
        self.write_keyword(&Keywords::Let)?;
        self.write_identifier()?;

        if let Some(Ok(Token::Symbol(s))) = self.tokenizer.peek() {
            if s == &Symbols::OpenSquareBrace {
                self.write_symbol(Symbols::OpenSquareBrace)?;
                self.write_expression()?;
                self.write_symbol(Symbols::CloseSquareBrace)?;
            }
        }

        self.write_symbol(Symbols::Equal)?;
        self.write_expression()?;
        self.write_symbol(Symbols::SemiColon)?;
        self.write_closing_tag("letStatement");
        Ok(())
    }

    fn write_if_statement(&mut self) -> Result<()> {
        self.write_opening_tag("ifStatement");
        self.write_keyword(&Keywords::If)?;
        self.write_symbol(Symbols::OpenBrace)?;
        self.write_expression()?;
        self.write_symbol(Symbols::CloseBrace)?;

        self.write_symbol(Symbols::OpenCurlyBrace)?;
        self.write_statements()?;
        self.write_symbol(Symbols::CloseCurlyBrace)?;

        if let Some(Ok(Token::Keyword(k))) = self.tokenizer.peek() {
            if k == &Keywords::Else {
                self.write_symbol(Symbols::OpenCurlyBrace)?;
                self.write_statements()?;
                self.write_symbol(Symbols::CloseCurlyBrace)?;
            }
        }
        self.write_closing_tag("ifStatement");
        Ok(())
    }

    fn write_while_statement(&mut self) -> Result<()> {
        self.write_opening_tag("whileStatement");
        self.write_keyword(&Keywords::While)?;
        self.write_symbol(Symbols::OpenBrace)?;
        self.write_expression()?;
        self.write_symbol(Symbols::CloseBrace)?;

        self.write_symbol(Symbols::OpenCurlyBrace)?;
        self.write_statements()?;
        self.write_symbol(Symbols::CloseCurlyBrace)?;
        self.write_closing_tag("whileStatement");
        Ok(())
    }

    fn write_do_statement(&mut self) -> Result<()> {
        self.write_opening_tag("doStatement");
        self.write_keyword(&Keywords::Do)?;
        self.write_subroutine_call()?;
        self.write_symbol(Symbols::SemiColon)?;
        self.write_closing_tag("doStatement");
        Ok(())
    }

    fn write_return_statement(&mut self) -> Result<()> {
        self.write_opening_tag("returnStatement");
        self.write_keyword(&Keywords::Return)?;

        if let Some(Ok(Token::Symbol(s))) = self.tokenizer.peek() {
            if s != &Symbols::SemiColon {
                self.write_expression()?;
            }
        }
        self.write_symbol(Symbols::SemiColon)?;
        self.write_closing_tag("returnStatement");
        Ok(())
    }

    fn write_expression_list(&mut self) -> Result<()> {
        // (2*3, ade, a.b())
        // ()
        // (2*3)
        self.write_opening_tag("expressionList");
        loop {
            let has_more_expression = match self.tokenizer.peek() {
                Some(Ok(Token::Symbol(s))) if s == &Symbols::CloseBrace => false,
                _ => true,
            };

            if !has_more_expression {
                break;
            }

            self.write_expression()?;

            match self.tokenizer.peek() {
                Some(Ok(Token::Symbol(s))) if s == &Symbols::Comma => {
                    self.write_symbol(Symbols::Comma)?;
                }
                _ => {}
            }
        }
        self.write_closing_tag("expressionList");
        Ok(())
    }

    fn write_expression(&mut self) -> Result<()> {
        self.write_opening_tag("expression");
        self.write_term()?;

        loop {
            let token = self.tokenizer.peek();
            let next_operator = match &token {
                Some(Ok(Token::Symbol(v))) => {
                    if Self::is_operator(v) {
                        Some(v)
                    } else {
                        None
                    }
                }
                _ => None,
            };

            if next_operator.is_none() {
                break;
            }

            self.write_operator()?;
            self.write_term()?;
        }
        self.write_closing_tag("expression");
        Ok(())
    }

    fn write_subroutine_call(&mut self) -> Result<()> {
        // (Class|varName).subRoutine(?expressionList)
        // subRoutine(?expressionList)
        self.write_identifier()?;

        if let Some(Ok(Token::Symbol(s))) = self.tokenizer.peek() {
            if s == &Symbols::Dot {
                self.write_symbol(Symbols::Dot)?;
                self.write_identifier()?;
            }
        }

        self.write_symbol(Symbols::OpenBrace)?;
        self.write_expression_list()?;
        self.write_symbol(Symbols::CloseBrace)?;
        Ok(())
    }

    fn write_term(&mut self) -> Result<()> {
        self.write_opening_tag("term");
        let token = self.tokenizer.peek();
        if let Some(Ok(token)) = token {
            match token {
                Token::IntConst(_) => self.write_const()?,
                Token::StringConst(_) => self.write_const()?,
                Token::Keyword(k) if k == &Keywords::Function => {
                    self.write_keyword(&Keywords::Function)?
                }
                Token::Keyword(k) if k == &Keywords::Method => {
                    self.write_keyword(&Keywords::Method)?
                }
                Token::Keyword(_) => self.write_keyword_constant()?,
                Token::Identifier(_) => self.write_term_identifier()?,
                Token::Symbol(s) if s == &Symbols::OpenBrace => {
                    self.write_symbol(Symbols::OpenBrace)?;
                    self.write_expression()?;
                    self.write_symbol(Symbols::OpenBrace)?;
                }
                Token::Symbol(s) if s == &Symbols::Minus => self.write_symbol(Symbols::Minus)?,
                Token::Symbol(s) if s == &Symbols::Tilde => self.write_symbol(Symbols::Tilde)?,
                _ => self.write_subroutine_call()?,
            }
        }
        self.write_closing_tag("term");
        Ok(())
    }

    fn write_term_identifier(&mut self) -> Result<()> {
        let _ = self.tokenizer.advance_cursor();
        let next_token = self.tokenizer.peek();
        if let Some(Ok(token)) = next_token {
            if token == &Token::Symbol(Symbols::OpenBrace) || token == &Token::Symbol(Symbols::Dot)
            {
                return self.write_subroutine_call();
            }
        }

        self.write_var_name()?;
        if let Some(next_token) = self.tokenizer.peek() {
            match next_token {
                Ok(Token::Symbol(s)) if s == &Symbols::OpenSquareBrace => {
                    self.write_symbol(Symbols::OpenSquareBrace)?;
                    self.write_expression()?;
                    self.write_symbol(Symbols::CloseSquareBrace)?;
                }
                Err(_) => bail!("Invalid token after identifier"),
                _ => {}
            }
        }

        Ok(())
    }

    fn write_const(&mut self) -> Result<()> {
        if let Some(Ok(token)) = self.tokenizer.next() {
            match token {
                Token::IntConst(i) => {
                    self.write_opening_tag("integerConstant");
                    self.write(&format!("{}", i));
                    self.write_closing_tag("integerConstant");
                }
                Token::StringConst(s) => {
                    self.write_opening_tag("stringConstant");
                    self.write(&format!("{}", s));
                    self.write_closing_tag("stringConstant");
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn write_var_name(&mut self) -> Result<()> {
        self.write_identifier()
    }

    fn write_subroutine_name(&mut self) -> Result<()> {
        self.write_identifier()
    }

    fn write_operator(&mut self) -> Result<()> {
        let op = self.tokenizer.next().unwrap()?;
        if let Token::Symbol(op) = &op {
            if Self::is_operator(&op) {
                self.write_opening_tag("symbol");
                self.write(&op.to_string());
                self.write_closing_tag("symbol");
                return Ok(());
            }
        }

        Err(anyhow!("Invalid operator"))
            .with_context(|| format!("operator `{}` is not a valid operator", op.to_string()))
    }

    fn is_operator(op: &Symbols) -> bool {
        match op {
            Symbols::Plus
            | Symbols::Minus
            | Symbols::Asterik
            | Symbols::BackSlash
            | Symbols::Ampersand
            | Symbols::VerticalBar
            | Symbols::LessThan
            | Symbols::GreaterThan
            | Symbols::Equal => true,
            _ => false,
        }
    }

    fn write_keyword_constant(&mut self) -> Result<()> {
        let token = self.tokenizer.next();
        if let Some(token) = token {
            if let Token::Keyword(keyword) = token? {
                match keyword {
                    Keywords::True | Keywords::False | Keywords::Null | Keywords::This => {
                        return Ok(self.write(&keyword.to_string()));
                    }
                    _ => {
                        return Err(anyhow!("Invalid keyword")).with_context(|| {
                            format!("keyword `{}` is not a valid keyword", keyword.to_string())
                        })
                    }
                }
            }
        }

        Err(anyhow!(""))
    }

    fn write_identifier(&mut self) -> Result<()> {
        let token = self.tokenizer.next().unwrap()?;
        if let Token::Identifier(k) = token {
            self.write_tagged("identifier", &k);
            return Ok(());
        }

        Err(anyhow!("invalid token"))
            .with_context(|| format!("`{}` is not a valid identifier", token.to_string()))
    }

    fn write_keyword(&mut self, keyword: &Keywords) -> Result<()> {
        let token = self.tokenizer.next().unwrap()?;
        if let Token::Keyword(k) = &token {
            if k == keyword {
                self.write_tagged("keyword", &keyword.to_string());
                return Ok(());
            }
        }

        Err(anyhow!("invalid token"))
            .with_context(|| format!("`{}` is not a valid keyword", token.to_string()))
    }

    fn write_symbol(&mut self, symbol: Symbols) -> Result<()> {
        let token = self.tokenizer.next().unwrap()?;
        if let Token::Symbol(s) = &token {
            if s == &symbol {
                self.write_tagged("symbol", &symbol.to_string());
                return Ok(());
            }
        }

        Err(anyhow!("invalid token"))
            .context(format!("`{}` is not a valid symbol", &token.to_string()))
            .context(format!("should print {}", symbol.to_string()))
    }
}
