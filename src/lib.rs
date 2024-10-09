use std::{fs::File, io::BufReader, path::PathBuf, time::Instant};

use ast_reader::clang_ast_parser::{ClangAstParser, ClangAstParserImpl};
use process::{
    clang_compile2ast_call::clang_compile2ast_call, terminal_process::TerminalProcess, Process,
};

pub mod ast_reader;
pub mod location;
pub mod process;

pub fn dry_run_ast_parser(compile_commands_json: &PathBuf) {
    let start_time_all = Instant::now();

    let file = File::open(compile_commands_json).expect("Failed to open file");
    let reader = BufReader::new(file);
    let entries: Vec<_> = json_compilation_db::read(reader).collect();

    println!(
        "Found {} entries in file {}",
        entries.len(),
        compile_commands_json.display()
    );

    for entry in entries {
        match entry {
            Ok(entry) => {
                let timer = Instant::now();
                let mut sub_timer = Instant::now();

                let mut terminal_process = Box::new(TerminalProcess::new(clang_compile2ast_call(
                    &entry.arguments.join(" "),
                )));
                if !terminal_process.process() {
                    println!("Error processing file: {}", entry.file.display());
                    continue;
                }

                let elapsed_compiler = sub_timer.elapsed();

                sub_timer = Instant::now();

                let mut parser: ClangAstParserImpl = ClangAstParserImpl::new(terminal_process);
                let ast = parser.parse_ast();

                let elapsed_parser = sub_timer.elapsed();
                let elapsed = timer.elapsed();

                let ast_element_count = if ast.is_some() { ast.unwrap().len() } else { 0 };

                println!(
                    "Read {} AST Elements {:?} total, {:?} compiler and {:?} parsing of File: {}",
                    ast_element_count,
                    elapsed,
                    elapsed_compiler,
                    elapsed_parser,
                    entry.file.display()
                );
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    let elapsed_all = start_time_all.elapsed();
    println!("Elapsed time: {:?}", elapsed_all);
}
