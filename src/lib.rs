use std::{path::PathBuf, time::Instant};

use ast_reader::{
    clang_ast_parser::{ClangAstParser, ClangAstParserImpl},
    compile_commands_reader::read_compile_commands_json_file,
};
use process::{clang_compile2ast_call::clang_compile2ast_call, terminal_process::TerminalProcess};

pub mod ast_reader;
pub mod location;
pub mod process;

pub fn dry_run_ast_parser(compile_commands_json: &PathBuf) {
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

        let mut terminal_process = Box::new(TerminalProcess::new(clang_compile2ast_call(
            &entry.command,
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
            entry.file
        );
    }

    let elapsed_all = start_time_all.elapsed();
    println!("Elapsed time: {:?}", elapsed_all);
}
