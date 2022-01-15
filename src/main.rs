use clap::Parser;

const IS_LINUX: bool = cfg!(linux);

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Use Linux-style names for labels when generating x86/ARM
    #[clap(long)]
    linux: bool,

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

fn main() {
    let args = Args::parse();
    println!("Hello, world!");
    println!("Use linux: {}", args.linux);
}
