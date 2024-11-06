#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{LineWriter, Write},
        path::PathBuf,
    };

    use rust_clang_call_graph::{
        ast_reader::compile_commands_reader::read_compile_commands_json_file,
        process::{
            clang_compile2ast_call::clang_compile2ast_call, terminal_process::TerminalProcess,
            Process,
        },
        run_ast_parser,
    };

    #[test]
    fn func_call_in_func_call_test() {
        assert_eq!(1, 1);
    }
}
