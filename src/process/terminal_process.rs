use std::{collections::VecDeque, process::Command};

use super::Process;

pub struct TerminalProcess {
    process_args: String,
    output_lines: VecDeque<String>,
}

impl TerminalProcess {
    pub fn new(process_args: String) -> Self {
        TerminalProcess {
            process_args,
            output_lines: VecDeque::new(),
        }
    }
}

impl Process for TerminalProcess {
    fn process(&mut self) -> bool {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", self.process_args.as_str()])
                .output()
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(self.process_args.as_str())
                .output()
        };

        match output {
            Ok(output) => {
                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines() {
                    self.output_lines.push_back(line.to_string());
                }

                return output.status.success();
            }
            Err(_) => {
                return false;
            }
        }
    }

    fn has_next_line(&self) -> bool {
        self.output_lines.len() > 0
    }

    fn fetch_next_line(&self) -> String {
        if self.output_lines.is_empty() {
            return "".to_string();
        }

        self.output_lines.front().unwrap().to_string()
    }

    fn get_next_line(&mut self) -> String {
        if self.output_lines.is_empty() {
            return "".to_string();
        }

        self.output_lines.pop_front().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_echo() {
        let mut process = TerminalProcess::new("echo Hello World!".to_string());

        assert!(process.process());

        assert!(process.has_next_line());
        assert_eq!("Hello World!", process.get_next_line());
        assert!(!process.has_next_line());

        // Avoid unnecessary crashes.
        assert_eq!("", process.get_next_line());
    }

    #[test]
    fn echo_multiple_lines() {
        let mut process =
            TerminalProcess::new("echo Hello World! && echo && echo How are you?".to_string());

        assert!(process.process());

        assert!(process.has_next_line());
        assert_eq!("Hello World!", process.get_next_line());
        assert!(process.has_next_line());
        assert_eq!("", process.get_next_line());
        assert!(process.has_next_line());
        assert_eq!("How are you?", process.get_next_line());
        assert!(!process.has_next_line());

        // Avoid unnecessary crashes.
        assert_eq!("", process.get_next_line());
    }

    #[test]
    fn invalid_command() {
        let mut process = TerminalProcess::new("invalid_command".to_string());

        assert!(!process.process());

        // Avoid unnecessary crashes.
        assert!(!process.has_next_line());
        assert_eq!("", process.get_next_line());
    }
}
