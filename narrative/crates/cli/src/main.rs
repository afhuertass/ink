use clap::{Parser, Subcommand};
use std::fs;

#[derive(Parser)]
#[command(name = "narrative")]
#[command(about = "Narrative framework compiler", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile an ink file to JSON
    Compile {
        /// Path to the ink file
        #[arg(value_name = "FILE")]
        file: String,

        /// Output file (default: stdout)
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Check an ink file for errors without compiling
    Check {
        /// Path to the ink file
        #[arg(value_name = "FILE")]
        file: String,
    },
    /// Play/interactive mode (basic REPL - stub for now)
    Play {
        /// Path to the ink file
        #[arg(value_name = "FILE")]
        file: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile { file, output } => {
            let source = match fs::read_to_string(&file) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error reading file '{}': {}", file, e);
                    std::process::exit(1);
                }
            };

            match narrative_compiler::compile_ink(&source, &file) {
                Ok(json) => {
                    if let Some(outfile) = output {
                        if let Err(e) = fs::write(&outfile, &json) {
                            eprintln!("Error writing output: {}", e);
                            std::process::exit(1);
                        }
                        println!("Compiled to {}", outfile);
                    } else {
                        println!("{}", json);
                    }
                }
                Err(errors) => {
                    eprintln!("Compilation errors:");
                    for e in errors {
                        eprintln!("  {}", e);
                    }
                    std::process::exit(1);
                }
            }
        }
        Commands::Check { file } => {
            let source = match fs::read_to_string(&file) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error reading file '{}': {}", file, e);
                    std::process::exit(1);
                }
            };

            let parsed = ink_parser::parse_story(&source, &file);
            if parsed.has_errors() {
                eprintln!("Errors found:");
                for e in &parsed.errors {
                    eprintln!("  {}", e);
                }
                std::process::exit(1);
            } else {
                println!("No errors found.");
            }
        }
        Commands::Play { file: _ } => {
            println!("Play mode not yet implemented. Use 'compile' to generate JSON.");
        }
    }
}