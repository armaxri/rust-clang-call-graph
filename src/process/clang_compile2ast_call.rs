use super::argument_splitter::split_arguments;

pub fn clang_compile2ast_call(compile_call: &str) -> String {
    let mut splitted_compile_call = split_arguments(compile_call);

    let mut adjusted_call_vec: Vec<String> = Vec::new();

    while splitted_compile_call.len() > 0 {
        let current_arg = splitted_compile_call.pop_front().unwrap();

        if current_arg == "-o" {
            match splitted_compile_call.pop_front() {
                Some(_) => {}
                None => {
                    // If there is no output file, we don't need to continue.
                    break;
                }
            }
        } else {
            adjusted_call_vec.push(current_arg);
        }
    }

    adjusted_call_vec.push("-Xclang".to_string());
    adjusted_call_vec.push("-ast-dump".to_string());
    adjusted_call_vec.push("-fsyntax-only".to_string());

    adjusted_call_vec.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clang_compile2ast_call() {
        let compile_call = "clang -c -o test.o test.c".to_string();
        let expected = "clang -c test.c -Xclang -ast-dump -fsyntax-only".to_string();

        assert_eq!(clang_compile2ast_call(&compile_call), expected);
    }
}
