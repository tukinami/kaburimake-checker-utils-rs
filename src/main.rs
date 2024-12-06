use std::process;

use clap::Parser;

mod ast;
mod config;
mod io;
mod procedure;

fn main() {
    let cli = config::Cli::parse();

    if let Err(e) = procedure::procedure(&cli) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    };
}
