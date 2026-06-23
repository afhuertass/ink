use clap::{Parser, Subcommand};
use std::fs;
use std::process::Command as Cmd;

#[derive(Parser)]
#[command(name = "narrative")]
#[command(about = "Narrative framework compiler for ink stories", long_about = None)]
#[command(version)]
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

        /// Pretty-print JSON output
        #[arg(long)]
        pretty: bool,

        /// Definitions file (.inkdef.yaml) for directive validation
        #[arg(long)]
        definitions: Option<String>,
    },
    /// Check an ink file for errors without compiling
    Check {
        /// Path to the ink file
        #[arg(value_name = "FILE")]
        file: String,

        /// Show warnings as well as errors
        #[arg(short, long)]
        warnings: bool,
    },
    /// Play an ink file interactively using inkjs
    Play {
        /// Path to the ink file
        #[arg(value_name = "FILE")]
        file: String,
    },
    /// Validate a definitions file
    CheckDefs {
        /// Path to the .inkdef.yaml file
        #[arg(value_name = "FILE")]
        file: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compile { file, output, pretty, definitions } => {
            let source = read_file(&file);

            if let Some(defs_file) = definitions {
                // Full compilation with definitions
                let defs_source = read_file(&defs_file);
                match narrative_compiler::compile_full(&source, &file, &defs_source, &defs_file) {
                    Ok(compilation) => {
                        let ink_json = if pretty {
                            compilation.ink_json
                        } else {
                            compilation.ink_json
                        };
                        if let Some(outfile) = output {
                            if let Err(e) = fs::write(&outfile, &ink_json) {
                                eprintln!("Error writing output: {}", e);
                                std::process::exit(1);
                            }
                            // Also write manifest and schema alongside
                            let base = outfile.trim_end_matches(".json");
                            let _ = fs::write(format!("{}.directives.json", base), &compilation.directives_manifest);
                            let _ = fs::write(format!("{}.schema.json", base), &compilation.definitions_schema);
                            eprintln!("Compiled to {} (+ manifest + schema)", outfile);
                        } else {
                            println!("{}", ink_json);
                        }
                    }
                    Err(errors) => {
                        eprintln!("Compilation errors in {}:", file);
                        for e in &errors {
                            eprintln!("  {}", e);
                        }
                        std::process::exit(1);
                    }
                }
            } else {
                let source = read_file(&file);

            match narrative_compiler::compile_ink(&source, &file) {
                Ok(json) => {
                    let output_json = if pretty {
                        match serde_json::from_str::<serde_json::Value>(&json) {
                            Ok(val) => serde_json::to_string_pretty(&val).unwrap_or(json),
                            Err(_) => json,
                        }
                    } else {
                        json
                    };

                    if let Some(outfile) = output {
                        if let Err(e) = fs::write(&outfile, &output_json) {
                            eprintln!("Error writing output: {}", e);
                            std::process::exit(1);
                        }
                        eprintln!("Compiled to {}", outfile);
                    } else {
                        println!("{}", output_json);
                    }
                }
                Err(errors) => {
                    eprintln!("Compilation errors in {}:", file);
                    for e in &errors {
                        eprintln!("  {}", e);
                    }
                    std::process::exit(1);
                }
            }
        }
        Commands::Check { file, warnings } => {
            let source = read_file(&file);

            let parsed = ink_parser::parse_story(&source, &file);
            if parsed.has_errors() {
                eprintln!("Errors in {}:", file);
                for e in &parsed.errors {
                    if e.is_error() {
                        eprintln!("  ERROR: {}", e);
                    } else if warnings {
                        eprintln!("  WARNING: {}", e);
                    }
                }
                std::process::exit(1);
            } else {
                if warnings {
                    let warns: Vec<_> = parsed.errors.iter().filter(|e| !e.is_error()).collect();
                    if !warns.is_empty() {
                        eprintln!("Warnings in {}:", file);
                        for w in warns {
                            eprintln!("  WARNING: {}", w);
                        }
                    }
                }
                println!("✓ No errors found in {}", file);
            }
        }
        Commands::Play { file } => {
            let source = read_file(&file);

            // Compile to JSON
            let json = match narrative_compiler::compile_ink(&source, &file) {
                Ok(j) => j,
                Err(errors) => {
                    eprintln!("Compilation errors:");
                    for e in &errors {
                        eprintln!("  {}", e);
                    }
                    std::process::exit(1);
                }
            };

            // Write JSON to temp file
            let tmp_json = format!("/tmp/narrative_play_{}.json", std::process::id());
            fs::write(&tmp_json, &json).expect("Failed to write temp JSON");

            // Run with inkjs via node
            let script = format!(
                r#"
const {{Story}} = require('inkjs');
const fs = require('fs');
try {{
    const json = fs.readFileSync('{}', 'utf8');
    const story = new Story(json);
    while (story.canContinue) {{
        process.stdout.write(story.Continue());
    }}
    if (story.currentChoices.length > 0) {{
        for (let i = 0; i < story.currentChoices.length; i++) {{
            console.log(`${{i+1}}: ${{story.currentChoices[i].text}}`);
        }}
    }}
}} catch(e) {{
    console.error('Runtime error:', e.message);
    process.exit(1);
}}"#,
                tmp_json.replace('\\', "\\\\").replace('\'', "\\'")
            );

            let result = Cmd::new("node")
                .arg("-e")
                .arg(&script)
                .output();

            let _ = fs::remove_file(&tmp_json);

            match result {
                Ok(output) => {
                    if output.status.success() {
                        print!("{}", String::from_utf8_lossy(&output.stdout));
                        if !output.stderr.is_empty() {
                            eprint!("{}", String::from_utf8_lossy(&output.stderr));
                        }
                    } else {
                        eprintln!("Runtime error: {}", String::from_utf8_lossy(&output.stderr));
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to run node: {} (is Node.js installed?)", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::CheckDefs { file } => {
            let source = read_file(&file);
            match def_parser::parse_definitions(&source, &file) {
                Ok(defs) => {
                    println!("✓ Valid definitions in {} (v{})", file, defs.version);
                    println!("  {} assets, {} characters, {} scenes, {} actions, {} state vars, {} events",
                        defs.assets.len(), defs.characters.len(), defs.scenes.len(),
                        defs.actions.len(), defs.state.len(), defs.events.len());
                }
                Err(errors) => {
                    eprintln!("Validation errors in {}:", file);
                    for e in &errors {
                        eprintln!("  {}", e);
                    }
                    std::process::exit(1);
                }
            }
        }
    }
}

fn read_file(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", path, e);
            std::process::exit(1);
        }
    }
}