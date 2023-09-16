use std::io::Write;

use anyhow::Result;

use crate::Token;

pub struct CompilationEngine {}

impl CompilationEngine {
    pub fn compile<T: Iterator<Item = Result<Token>>>(
        tokenizer: &mut T,
        _writer: &mut dyn Write,
    ) -> Result<()> {
        while let Some(token) = tokenizer.next() {
            println!(">>> {:?}", token?)
        }
        Ok(())
    }
}
