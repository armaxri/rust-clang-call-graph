#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use rust_clang_call_graph::dry_run_ast_parser;

    #[test]
    fn dry_run_ast_parser_test() {
        let compile_commands_json = PathBuf::from("tests/playground/compile_commands.json");
        dry_run_ast_parser(&compile_commands_json);
    }
}
