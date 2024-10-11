use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CompileCommand {
    pub directory: String,
    pub command: String,
    pub file: String,
    pub output: Option<String>,
}

fn read_compile_commands_json(file: &std::fs::File) -> Option<Vec<CompileCommand>> {
    let reader = std::io::BufReader::new(file);
    let entries: Result<Vec<CompileCommand>, serde_json::Error> = serde_json::from_reader(reader);
    match entries {
        Ok(entries) => Some(entries),
        Err(_) => None,
    }
}

pub fn read_compile_commands_json_file(file: &std::path::PathBuf) -> Option<Vec<CompileCommand>> {
    let file = std::fs::File::open(file).ok();
    match file {
        Some(file) => read_compile_commands_json(&file),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn read_compile_commands_json_file_test() {
        let compile_commands_json = PathBuf::from("tests/playground/compile_commands.json");
        let compile_commands = read_compile_commands_json_file(&compile_commands_json);
        assert!(compile_commands.is_some());
        let compile_commands = compile_commands.unwrap();
        assert_ne!(compile_commands.len(), 0);
    }

    #[test]
    fn read_compile_commands_json_one_member_test() {
        let virtual_file = r#"
        [
            {
                "directory": "/home/user/project",
                "command": "/usr/bin/clang++ -Iinclude src/main.cpp -o build/main.o",
                "file": "src/main.cpp",
                "output": "build/main.o"
            }
        ]
        "#;

        let compile_commands: Vec<CompileCommand> = serde_json::from_str(virtual_file).unwrap();
        assert_eq!(compile_commands.len(), 1);
        let compile_command = &compile_commands[0];
        assert_eq!(compile_command.directory, "/home/user/project");
        assert_eq!(
            compile_command.command,
            "/usr/bin/clang++ -Iinclude src/main.cpp -o build/main.o"
        );
        assert_eq!(compile_command.file, "src/main.cpp");
        match &compile_command.output {
            Some(output) => assert_eq!(output, "build/main.o"),
            None => assert!(false),
        }
    }

    #[test]
    fn read_compile_commands_json_one_member_no_output_test() {
        let virtual_file = r#"
        [
            {
                "directory": "/home/user/project",
                "command": "/usr/bin/clang++ -Iinclude src/main.cpp -o build/main.o",
                "file": "src/main.cpp"
            }
        ]
        "#;

        let compile_commands: Vec<CompileCommand> = serde_json::from_str(virtual_file).unwrap();
        assert_eq!(compile_commands.len(), 1);
        let compile_command = &compile_commands[0];
        assert_eq!(compile_command.directory, "/home/user/project");
        assert_eq!(
            compile_command.command,
            "/usr/bin/clang++ -Iinclude src/main.cpp -o build/main.o"
        );
        assert_eq!(compile_command.file, "src/main.cpp");
        assert!(compile_command.output.is_none());
    }

    #[test]
    fn read_compile_commands_json_one_member_windows_paths() {
        let virtual_file = r#"
        [
            {
                "directory": "C:\\home\\user\\project",
                "command": "C:\\usr\\bin\\clang++ -Iinclude src\\main.cpp -o build\\main.o",
                "file": "src\\main.cpp",
                "output": "build\\main.o"
            }
        ]
        "#;

        let compile_commands: Vec<CompileCommand> = serde_json::from_str(virtual_file).unwrap();
        assert_eq!(compile_commands.len(), 1);
        let compile_command = &compile_commands[0];
        assert_eq!(compile_command.directory, "C:\\home\\user\\project");
        assert_eq!(
            compile_command.command,
            "C:\\usr\\bin\\clang++ -Iinclude src\\main.cpp -o build\\main.o"
        );
        assert_eq!(compile_command.file, "src\\main.cpp");
        match &compile_command.output {
            Some(output) => assert_eq!(output, "build\\main.o"),
            None => assert!(false),
        }
    }

    #[test]
    fn read_compile_commands_json_two_members_test() {
        let virtual_file = r#"
        [
            {
                "directory": "/home/user/project",
                "command": "/usr/bin/clang++ -Iinclude src/main.cpp -o build/main.o",
                "file": "src/main.cpp",
                "output": "build/main.o"
            },
            {
                "directory": "/home/user/project",
                "command": "/usr/bin/clang++ -Iinclude src/helper.cpp -o build/helper.o",
                "file": "src/helper.cpp",
                "output": "build/helper.o"
            }
        ]
        "#;

        let compile_commands: Vec<CompileCommand> = serde_json::from_str(virtual_file).unwrap();
        assert_eq!(compile_commands.len(), 2);
        let compile_command = &compile_commands[0];
        assert_eq!(compile_command.directory, "/home/user/project");
        assert_eq!(
            compile_command.command,
            "/usr/bin/clang++ -Iinclude src/main.cpp -o build/main.o"
        );
        assert_eq!(compile_command.file, "src/main.cpp");
        match &compile_command.output {
            Some(output) => assert_eq!(output, "build/main.o"),
            None => assert!(false),
        }

        let compile_command = &compile_commands[1];
        assert_eq!(compile_command.directory, "/home/user/project");
        assert_eq!(
            compile_command.command,
            "/usr/bin/clang++ -Iinclude src/helper.cpp -o build/helper.o"
        );
        assert_eq!(compile_command.file, "src/helper.cpp");
        match &compile_command.output {
            Some(output) => assert_eq!(output, "build/helper.o"),
            None => assert!(false),
        }
    }

    #[test]
    fn read_compile_commands_json_empty_test() {
        let virtual_file = r#"
        []
        "#;

        let compile_commands: Vec<CompileCommand> = serde_json::from_str(virtual_file).unwrap();
        assert_eq!(compile_commands.len(), 0);
    }

    #[test]
    fn read_compile_commands_json_invalid_test() {
        let virtual_file = r#"
        [
            {
                "directory": "/home/user/project",
                "command": "/usr/bin/clang++ -Iinclude src/main.cpp -o build/main.o",
                "file": "src/main.cpp",
                "output": "build/main.o"
            },
            {
                "directory": "/home/user/project",
                "command": "/usr/bin/clang++ -Iinclude src/helper.cpp -o build/helper.o",
                "file": "src/helper.cpp",
                "output": "build/helper.o"
        ]
        "#;

        let compile_commands: Result<Vec<CompileCommand>, serde_json::Error> =
            serde_json::from_str(virtual_file);
        assert!(compile_commands.is_err());
    }

    #[test]
    fn read_compile_commands_json_invalid_member_test() {
        let virtual_file = r#"
        [
            {
                "directory": "/home/user/project",
                "command": "/usr/bin/clang++ -Iinclude src/main.cpp -o build/main.o",
                "file": "src/main.cpp",
                "output": "build/main.o"
            },
            {
                "directory": "/home/user/project",
                "command": "/usr/bin/clang++ -Iinclude src/helper.cpp -o build/helper.o"
            }
        ]
        "#;

        let compile_commands: Result<Vec<CompileCommand>, serde_json::Error> =
            serde_json::from_str(virtual_file);
        assert!(compile_commands.is_err());
    }

    #[test]
    fn read_compile_commands_json_invalid_member_type_test() {
        let virtual_file = r#"
        [
            {
                "directory": "/home/user/project",
                "command": "/usr/bin/clang++ -Iinclude src/main.cpp -o build/main.o",
                "file": "src/main.cpp",
                "output": "build/main.o"
            },
            "invalid"
        ]
        "#;

        let compile_commands: Result<Vec<CompileCommand>, serde_json::Error> =
            serde_json::from_str(virtual_file);
        assert!(compile_commands.is_err());
    }

    #[test]
    fn read_compile_commands_json_invalid_type_test() {
        let virtual_file = r#"
        "invalid"
        "#;

        let compile_commands: Result<Vec<CompileCommand>, serde_json::Error> =
            serde_json::from_str(virtual_file);
        assert!(compile_commands.is_err());
    }

    #[test]
    fn read_compile_commands_json_invalid_empty_test() {
        let virtual_file = r#"
        ""
        "#;

        let compile_commands: Result<Vec<CompileCommand>, serde_json::Error> =
            serde_json::from_str(virtual_file);
        assert!(compile_commands.is_err());
    }

    #[test]
    fn read_compile_commands_json_invalid_empty_array_test() {
        let virtual_file = r#"
        []
        "#;

        let compile_commands: Result<Vec<CompileCommand>, serde_json::Error> =
            serde_json::from_str(virtual_file);
        assert!(compile_commands.is_ok());
        let compile_commands = compile_commands.unwrap();
        assert_eq!(compile_commands.len(), 0);
    }

    #[test]
    fn read_compile_commands_json_invalid_empty_object_test() {
        let virtual_file = r#"
        {}
        "#;

        let compile_commands: Result<Vec<CompileCommand>, serde_json::Error> =
            serde_json::from_str(virtual_file);
        assert!(compile_commands.is_err());
    }
}
