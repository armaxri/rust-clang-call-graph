use std::{cell::RefCell, path::PathBuf, rc::Rc, time::Instant};

use ast_reader::{
    clang_ast_parser::{ClangAstParser, ClangAstParserImpl},
    compile_commands_reader::read_compile_commands_json_file,
};
use call_graph::{
    ast_walker::clang_ast_walker::walk_ast_2_func_call_db,
    database::database_sqlite::DatabaseSqlite,
};
use process::{
    clang_compile2ast_call::clang_compile2ast_call, terminal_process::TerminalProcess, Process,
};

pub mod ast_reader;
pub mod call_graph;
#[macro_use]
pub mod macros;
pub mod location;
pub mod process;

pub fn run_ast_parser(
    compile_commands_json: &PathBuf,
    db: Option<Rc<RefCell<DatabaseSqlite>>>,
    ignored_namespaces: &Vec<String>,
) {
    let start_time_all = Instant::now();

    let entries = if read_compile_commands_json_file(compile_commands_json).is_some() {
        read_compile_commands_json_file(compile_commands_json).unwrap()
    } else {
        println!("Error reading compile_commands.json file");
        return;
    };

    println!(
        "Found {} entries in file {}",
        entries.len(),
        compile_commands_json.display()
    );

    for entry in entries {
        let timer = Instant::now();
        let mut sub_timer = Instant::now();

        let mut terminal_process =
            Box::new(TerminalProcess::new(clang_compile2ast_call(&entry.command)));

        if !terminal_process.process() {
            println!("Error code returned while processing file {}", entry.file);
        }

        let elapsed_compiler = sub_timer.elapsed();

        if !terminal_process.has_next_line() {
            println!("Error processing file: {}", entry.file);
            continue;
        }

        while !terminal_process
            .fetch_next_line()
            .starts_with("TranslationUnitDecl")
            && terminal_process.has_next_line()
        {
            terminal_process.get_next_line();
        }

        if !terminal_process.has_next_line() {
            println!("Process didn't return AST output. File: {}", entry.file);
            continue;
        }

        sub_timer = Instant::now();

        let mut parser: ClangAstParserImpl = ClangAstParserImpl::new(terminal_process);
        let ast = parser.parse_ast();

        let elapsed_parser = sub_timer.elapsed();

        sub_timer = Instant::now();

        if let (Some(ast), Some(db_ref)) = (ast, db.as_ref()) {
            walk_ast_2_func_call_db(&entry.file, ast, db_ref.clone());
        }

        let elapsed_walking = sub_timer.elapsed();
        let elapsed = timer.elapsed();

        println!(
            "Handled file in {} total, {} compiler, {} parsing and {} AST walking of File: {}",
            duration2str(elapsed),
            duration2str(elapsed_compiler),
            duration2str(elapsed_parser),
            duration2str(elapsed_walking),
            entry.file
        );
    }

    let elapsed_all = start_time_all.elapsed();
    println!("Elapsed time: {:?}", elapsed_all);
}

fn duration2str(duration: std::time::Duration) -> String {
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();
    format!("{}.{:02}s", secs, millis)
}
