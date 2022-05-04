use std::error::Error;
use std::fs;

use clap::Parser;

use oat_parse::parse_program;
use oat_symbol::create_session_if_not_set_then;
use oat_typecheck::type_check;

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

fn compile(input: &str) -> Result<(), Box<dyn Error>> {
    create_session_if_not_set_then(|_| {
        let program = dbg!(parse_program(&input)?);
        type_check(&program)?;
        Ok(())
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let use_linux_naming = IS_LINUX || matches!(args.linux, Some(true));

    println!("Linux naming?: {}", use_linux_naming);

    // println!("Hello, world!");
    // println!("--linux passed: {}", matches!(args.linux, Some(_)));
    // println!("Use linux: {}", args.linux.unwrap_or(IS_LINUX));
    //
    //
    // String
    let input = fs::read_to_string(&args.files[0])?;
    // let input = content.as_str();
    compile(&input)?;

    Ok(())
}
