use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "baulu")]
#[command(version = "0.1.0")]
#[command(author = "mustafaelrasheid")]
#[command(
	about = "A compiler for the programming language Baulu.",
	long_about = None
)]
pub struct Cli {
	#[command(subcommand)]
	pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
}
