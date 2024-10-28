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
    fn dry_run_ast_parser_test() {
        let compile_commands_json = PathBuf::from("tests/playground/compile_commands.json");
        run_ast_parser(&compile_commands_json, None);
    }

    #[test]
    fn process_files2ast_files() {
        let compile_commands_json = PathBuf::from("tests/playground/compile_commands.json");

        let entries = if read_compile_commands_json_file(&compile_commands_json).is_some() {
            read_compile_commands_json_file(&compile_commands_json).unwrap()
        } else {
            assert!(false);
            return;
        };

        for entry in entries {
            let mut terminal_process = TerminalProcess::new(clang_compile2ast_call(&entry.command));

            assert!(terminal_process.process());

            let file =
                File::create(entry.file[..entry.file.len() - 3].to_string() + "ast").unwrap();
            let mut writer = LineWriter::new(file);
            while terminal_process.has_next_line() {
                let buffer = terminal_process.get_next_line();
                writer.write(buffer.as_bytes()).unwrap();
                writer.write("\n".as_bytes()).unwrap();
            }
        }
    }
}
