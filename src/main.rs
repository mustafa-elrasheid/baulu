mod args;

use clap::Parser;
use args::{Cli, Commands};

fn main() {
    let cli = Cli::parse();
    
    match cli.command {

    }
}
