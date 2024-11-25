use std::{cell::RefCell, path::PathBuf, rc::Rc};

use clap::{Args, Parser, Subcommand};
use rust_clang_call_graph::{
    call_graph::database::database_sqlite::DatabaseSqlite, run_ast_parser,
};

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
    /// The compile_commands.json file to use
    #[arg(short, long, value_name = "FILE")]
    compile_commands_json: PathBuf,
    /// Namespaces to ignore
    #[arg(short, long, value_name = "NAMESPACE", default_values_t = ["std".to_string(), "boost".to_string(), "mpl_".to_string()])]
    ignored_namespaces: Vec<String>,
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

                run_ast_parser(compile_commands_json, None, &vec![]);
            }
            None => {
                println!("No compile_commands_json file specified");
            }
        },
        Commands::NewDatabase(args) => {
            if !&args.compile_commands_json.exists() {
                println!(
                    "The file compile_commands.json file '{}' does not exist",
                    &args.compile_commands_json.display()
                );
                return;
            }
            println!(
                "Using compile_commands_json file: {}",
                &args.compile_commands_json.display()
            );
            println!("Creating new database at: {}", args.database_path.display());

            let db = Rc::new(RefCell::new(DatabaseSqlite::create_database(
                &args.database_path,
                true,
            )));

            run_ast_parser(
                &args.compile_commands_json,
                Some(db),
                args.ignored_namespaces.as_ref(),
            );
        }
    }
}
