use std::fs;
use std::path::PathBuf;

pub use anyhow::Result;
use clap::Parser;

mod parser;
mod pyproject;
mod visitor;

use pyproject::parse_requirements_from_file;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(help = "Path to the file to process")]
    path: String,

    #[arg(help = "Path to the requirements file that has the requirement")]
    requirement_file: PathBuf,
}

fn main() -> Result<()> {
    let cli = Args::parse();
    let contents = fs::read_to_string(&cli.path)?;
    let imports = parser::parse_imports(&contents)?;

    for import in imports.into_iter() {
        println!("{}", import);
    }

    let requirements = parse_requirements_from_file(cli.requirement_file)?;

    for requirement in requirements {
        println!("{}, {:?}", requirement.name, requirement.extras);
    }

    Ok(())
}
