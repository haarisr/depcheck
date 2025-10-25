use std::fs;

use anyhow::Result;
use clap::Parser;

mod parser;
mod visitor;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(help = "Path to the file to process")]
    path: String,
}

fn main() -> Result<()> {
    let cli = Args::parse();
    let contents = fs::read_to_string(&cli.path)?;
    let imports = parser::parse_imports(&contents)?;

    for import in imports.into_iter() {
        println!("{}", import);
    }
    Ok(())
}
