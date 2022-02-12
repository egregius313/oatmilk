use std::error::Error;
use std::fs;

use clap::Parser;

use oat_parse::parse_program;

mod config;
mod frontend;

/// Whether or not the current platform is Linux
const IS_LINUX: bool = cfg!(linux);

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Use Linux-style names for labels when generating x86/ARM
    #[clap(long)]
    linux: Option<bool>,

    /// Run the test suite
    #[clap(long)]
    test: bool,

    /// Output path. Where to place the executable, or resulting files
    #[clap(short, long, default_value = "a.out")]
    output: String,

    /// Stop at assembly
    #[clap(short = 'S')]
    stop_at_assemble: bool,

    /// Stop at creation of object files
    #[clap(short = 'c')]
    stop_at_object: bool,

    /// Verbose
    #[clap(short, long)]
    verbose: bool,

    /// Files to compile
    files: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // println!("Hello, world!");
    // println!("--linux passed: {}", matches!(args.linux, Some(_)));
    // println!("Use linux: {}", args.linux.unwrap_or(IS_LINUX));
    //
    //
    // String
    let input = fs::read_to_string(&args.files[0])?;
    // let input = content.as_str();
    let program = parse_program(&input)?;

    Ok(())
}
