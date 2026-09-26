mod ast;
mod parser;
mod generator;
mod quoting;

use clap::Parser as ClapParser; // renamed to avoid clash with our own `parser` module
use std::fs;

#[derive(ClapParser)]
#[command(name = "bashrec")]
#[command(about = "Compile .brec files to .sh")]
struct Cli {
    input: String,

    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let content = fs::read_to_string(&cli.input).expect("Failed to read input file");

    let statements = parser::parse(&content);
    let generated = generator::generate(&statements);

    let output_path = cli.output.unwrap_or_else(|| cli.input.replace(".brec", ".sh"));
    fs::write(&output_path, &generated).expect("Failed to write output file");
    println!("Compiled {} -> {}", cli.input, output_path);
}