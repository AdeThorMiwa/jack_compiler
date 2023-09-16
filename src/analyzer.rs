use std::{
    ffi::OsStr,
    fs::{DirEntry, File, FileType},
    io::{BufWriter, Write},
    path::PathBuf,
};

use anyhow::Result;

use crate::{CompilationEngine, StreamTokenizer};

pub struct Analyzer;

impl Analyzer {
    pub fn analyze(source: &PathBuf) -> Result<()> {
        let files = Self::read_source_files(source)?;

        for file in files {
            // instatiate a new Tokenizer
            // let mut tokenizer = NaiveTokenizer::new(&file);
            let mut tokenizer = StreamTokenizer::new(&file);

            // create a output file
            let output_file = File::create(file.with_extension("xml"))?;
            let mut writer = BufWriter::new(output_file);

            // use compilation engine to compile tokens from the tokenizer
            CompilationEngine::compile(&mut tokenizer, &mut writer)?;

            // save compilation output into output file
            writer.flush()?;
        }
        Ok(())
    }

    fn read_source_files(source: &PathBuf) -> std::io::Result<Vec<PathBuf>> {
        if source.is_dir() {
            let mut files: Vec<PathBuf> = Vec::new();
            for entry in std::fs::read_dir(source)? {
                let entry = entry?;
                if Self::is_jack_file(&entry) {
                    files.push(entry.path())
                }
            }

            Ok(files)
        } else {
            Ok(vec![source.to_path_buf()])
        }
    }

    fn is_jack_file(entry: &DirEntry) -> bool {
        FileType::is_file(&entry.file_type().unwrap())
            && entry.path().extension().and_then(OsStr::to_str) == Some("jack")
    }
}
