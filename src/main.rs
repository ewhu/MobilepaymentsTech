// src/main.rs
/*
 * Main executable for MobilepaymentsTech
 */

use clap::Parser;
use mobilepaymentstech::{Result, run};

#[derive(Parser)]
#[command(version, about = MobilepaymentsTech - A Rust implementation)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
