use anyhow::Result;
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(help = "Path to the file to process")]
    path: String,
}

fn main() -> Result<()> {
    let cli = Args::parse();
    let contents = fs::read_to_string(&cli.path)?;
    println!("Contents of the file");
    println!("{}", contents);

    Ok(())
}
