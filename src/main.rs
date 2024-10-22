use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use rust_clang_call_graph::dry_run_ast_parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new call graph database
    NewDatabase(NewDatabaseArgs),
    /// Make a dry run of the AST parser
    DryRun(DryRunArgs),
}

#[derive(Args)]
struct NewDatabaseArgs {
    /// The SQLite database file to create
    #[arg(short, long, value_name = "FILE")]
    database_path: PathBuf,
}

#[derive(Args)]
struct DryRunArgs {
    /// The compile_commands.json file to use
    #[arg(short, long, value_name = "FILE")]
    compile_commands_json: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    // Check more examples later https://docs.rs/clap/latest/clap/_derive/_tutorial/chapter_0/index.html
    match &cli.command {
        Commands::DryRun(args) => match &args.compile_commands_json {
            Some(compile_commands_json) => {
                if !compile_commands_json.exists() {
                    println!(
                        "The file compile_commands.json file '{}' does not exist",
                        compile_commands_json.display()
                    );
                    return;
                }
                println!(
                    "Using compile_commands_json file: {}",
                    compile_commands_json.display()
                );

                dry_run_ast_parser(compile_commands_json);
            }
            None => {
                println!("No compile_commands_json file specified");
            }
        },
        Commands::NewDatabase(new_database_args) => {
            println!(
                "Creating new database at: {}",
                new_database_args.database_path.display()
            );
        }
    }
}
