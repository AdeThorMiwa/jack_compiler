use std::io::Write;

use crate::Token;

pub struct CompilationEngine {}

impl CompilationEngine {
    pub fn compile(
        _tokenizer: &dyn Iterator<Item = Token>,
        _writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        Ok(())
    }
}
