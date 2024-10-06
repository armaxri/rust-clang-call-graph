use std::collections::VecDeque;

fn get_escaped_char(current_char: char, current_str_char: char) -> String {
    match current_char {
        '\\' => current_char.to_string(),
        '0' => "\0".to_string(),
        'a' => "a".to_string(),
        'b' => r"\b".to_string(),
        't' => "\t".to_string(),
        'n' => "\n".to_string(),
        'v' => r"\v".to_string(),
        'f' => r"\f".to_string(),
        'r' => "\r".to_string(),
        '"' | '\'' => {
            // We need to escape the character if it is the same as the current character.
            if current_char == current_str_char {
                current_char.to_string()
            } else {
                "\\".to_string() + &current_char.to_string()
            }
        }
        // This might be shady but if there is no correct replacement, we use both characters.
        _ => "\\".to_string() + &current_char.to_string(),
    }
}

pub fn split_arguments(input: &str) -> VecDeque<String> {
    let mut args = VecDeque::new();
    let mut current_arg = String::new();

    // Store if a string was started and if yes with the kind of character.
    let mut current_str_char = '\0';
    let mut index = 0;

    while index < input.len() {
        let current_char = input.chars().nth(index).unwrap();

        if current_char == '\\' {
            match input.chars().nth(index + 1) {
                Some(next_char) => {
                    current_arg.push_str(&get_escaped_char(next_char, current_str_char));
                    index += 1;
                }
                None => {
                    // Workaround since we don't have error handling in this area.
                    current_arg.push(current_char);
                }
            }
        } else if current_char == '"' || current_char == '\'' {
            if current_str_char == current_char {
                // If we are in a string and the type is matching, we end here.
                // Keep in mind that escaped characters are already dealt with.
                current_str_char = '\0';
            } else if current_str_char != '\0' {
                // If we are in a string and there was no match, it's a different string character. So we simply use it.
                current_arg.push(current_char);
            } else {
                // This case deals with string starts. The character is not added but we memories the start of the string.
                current_str_char = current_char;
            }
        } else if current_char == ' ' {
            if current_str_char != '\0' {
                current_arg.push(current_char);
            } else {
                if !current_arg.is_empty() {
                    args.push_back(current_arg);
                    current_arg = String::new();
                }
            }
        } else {
            current_arg.push(current_char);
        }

        index += 1;
    }

    if !current_arg.is_empty() {
        args.push_back(current_arg);
    }

    args
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_escaped_char() {
        assert_eq!(get_escaped_char('\\', '"'), "\\");
        assert_eq!(get_escaped_char('\\', '\''), "\\");
        assert_eq!(get_escaped_char('0', '"'), "\0");
        assert_eq!(get_escaped_char('0', '\''), "\0");
        assert_eq!(get_escaped_char('a', '"'), "a");
        assert_eq!(get_escaped_char('a', '\''), "a");
        assert_eq!(get_escaped_char('b', '"'), r"\b");
        assert_eq!(get_escaped_char('b', '\''), r"\b");
        assert_eq!(get_escaped_char('t', '"'), "\t");
        assert_eq!(get_escaped_char('t', '\''), "\t");
        assert_eq!(get_escaped_char('n', '"'), "\n");
        assert_eq!(get_escaped_char('n', '\''), "\n");
        assert_eq!(get_escaped_char('v', '"'), r"\v");
        assert_eq!(get_escaped_char('v', '\''), r"\v");
        assert_eq!(get_escaped_char('f', '"'), r"\f");
        assert_eq!(get_escaped_char('f', '\''), r"\f");
        assert_eq!(get_escaped_char('r', '"'), "\r");
        assert_eq!(get_escaped_char('r', '\''), "\r");
        assert_eq!(get_escaped_char('\'', '\''), "'");
        assert_eq!(get_escaped_char('\'', '"'), "\\'");
        assert_eq!(get_escaped_char('"', '"'), "\"");
        assert_eq!(get_escaped_char('"', '\''), "\\\"");
        assert_eq!(get_escaped_char('x', '"'), "\\x");
        assert_eq!(get_escaped_char('x', '\''), "\\x");
    }

    #[test]
    fn test_split_arguments() {
        assert_eq!(
            split_arguments("echo Hello World!"),
            vec!["echo", "Hello", "World!"]
        );
        assert_eq!(
            split_arguments("echo 'Hello World!'"),
            vec!["echo", "Hello World!"]
        );
        assert_eq!(
            split_arguments("echo \"Hello World!\""),
            vec!["echo", "Hello World!"]
        );
    }

    /*
    Interesting suggestion from copilot. However, I'm not sure if they are correct and they are also not really realistic in our use case.
    #[test]
    fn test_split_arguments_with_escaped_characters() {
        assert_eq!(
            split_arguments("echo Hello\\ World!"),
            vec!["echo", "Hello\\", "World!"]
        );
        assert_eq!(
            split_arguments("echo 'Hello\\ World!'"),
            vec!["echo", "Hello\\ World!"]
        );
        assert_eq!(
            split_arguments("echo \"Hello\\ World!\""),
            vec!["echo", "Hello\\ World!"]
        );
        assert_eq!(
            split_arguments("echo 'Hello\\' World!'"),
            vec!["echo", "Hello' World!"]
        );
        assert_eq!(
            split_arguments("echo \"Hello\\\" World!\""),
            vec!["echo", "Hello\" World!"]
        );
    }
    */

    #[test]
    fn test_split_args_simple_test1() {
        assert_eq!(
            split_arguments("-p \"hello b\\\"ar baz\" -f /^ [^ ]+ $/ -c -d -e"),
            vec![
                "-p",
                "hello b\"ar baz",
                "-f",
                "/^",
                "[^",
                "]+",
                "$/",
                "-c",
                "-d",
                "-e"
            ]
        );
    }

    #[test]
    fn test_split_args_simple_test2() {
        assert_eq!(
            split_arguments("-p 'hello b\\\"ar baz' -f /^ [^ ]+ $/ -c -d -e"),
            vec![
                "-p",
                "hello b\\\"ar baz",
                "-f",
                "/^",
                "[^",
                "]+",
                "$/",
                "-c",
                "-d",
                "-e"
            ]
        );
    }

    #[test]
    fn test_split_args_extra_spaces() {
        assert_eq!(
            split_arguments("echo    Hello World!"),
            vec!["echo", "Hello", "World!"]
        );
        assert_eq!(
            split_arguments("-p    -f   -c -d -e"),
            vec!["-p", "-f", "-c", "-d", "-e"]
        );
    }

    #[test]
    fn test_split_args_split_with_string_connected_to_arg() {
        assert_eq!(
            split_arguments("-p=\"hello b\\\"ar baz\" -f"),
            vec!["-p=hello b\"ar baz", "-f"]
        );
    }

    #[test]
    fn test_split_args() {
        assert_eq!(split_arguments("\\"), vec!["\\"]);
        assert_eq!(split_arguments("'\""), vec!["\""]);
        assert_eq!(split_arguments("\"'"), vec!["'"]);
        assert_eq!(split_arguments(""), Vec::<String>::new());
    }

    #[test]
    fn test_split_args_clang_call() {
        assert_eq!(
            split_arguments("/usr/bin/clang++  -I/Users/xxxx/work/git/vscode-clang-call-graph/test_workspaces/workspace00 -g -arch arm64 -isysroot /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk -o CMakeFiles/Workspace00Exe.dir/simple_c_style_func.cpp.o -c /Users/xxxx/work/git/vscode-clang-call-graph/test_workspaces/workspace00/simple_c_style_func.cpp"),
            vec!["/usr/bin/clang++",
            "-I/Users/xxxx/work/git/vscode-clang-call-graph/test_workspaces/workspace00",
            "-g",
            "-arch",
            "arm64",
            "-isysroot",
            "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX12.3.sdk",
            "-o",
            "CMakeFiles/Workspace00Exe.dir/simple_c_style_func.cpp.o",
            "-c",
            "/Users/xxxx/work/git/vscode-clang-call-graph/test_workspaces/workspace00/simple_c_style_func.cpp"]
        );
    }
}
