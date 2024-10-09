use std::{fs::File, io::BufReader, path::PathBuf, time::Instant};

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
        let start_time = Instant::now();

        let terminal_process =
            Box::new(TerminalProcess::new(clang_compile2ast_call(&entry.command)));
        let mut parser: ClangAstParserImpl = ClangAstParserImpl::new(terminal_process);
        let ast = parser.parse_ast();

        let ast_element_count = if ast.is_some() { ast.unwrap().len() } else { 0 };

        println!(
            "Read {} AST Elements in {:?} in File: {}",
            ast_element_count,
            start_time.elapsed(),
            entry.file
        );
    }

    let elapsed_all = start_time_all.elapsed();
    println!("Elapsed time: {:?}", elapsed_all);
}
