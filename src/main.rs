use std::path::PathBuf;

use clap::Parser;
use jack_compiler::Analyzer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    source: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    Analyzer::analyze(&PathBuf::from(args.source))?;
    Ok(())
}
