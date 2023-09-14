use std::io::Write;

use crate::Token;

pub struct CompilationEngine {}

impl CompilationEngine {
    pub fn compile(
        tokenizer: &mut dyn Iterator<Item = Token>,
        _writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        while let Some(token) = tokenizer.next() {
            println!(">>> {:?}", token)
        }
        Ok(())
    }
}
