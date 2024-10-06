use std::collections::VecDeque;
use std::fs::read_to_string;

use super::Process;

pub struct DummyProcess {
    pub success: bool,
    pub lines: VecDeque<String>,
}

impl DummyProcess {
    pub fn new() -> Self {
        DummyProcess {
            success: true,
            lines: VecDeque::new(),
        }
    }

    pub fn new_from_file(file_name: &String) -> Self {
        let mut lines = VecDeque::new();

        match read_to_string(file_name) {
            Ok(content) => {
                for line in content.lines() {
                    lines.push_back(line.to_string())
                }
            }
            Err(_) => {
                return DummyProcess {
                    success: false,
                    lines,
                };
            }
        }

        DummyProcess {
            success: true,
            lines,
        }
    }

    pub fn add_line(&mut self, line: String) {
        self.lines.push_back(line);
    }
}

impl Process for DummyProcess {
    fn process(&mut self) -> bool {
        return self.success;
    }

    fn has_next_line(&self) -> bool {
        return !self.lines.is_empty();
    }

    fn fetch_next_line(&self) -> String {
        if self.lines.is_empty() {
            return "".to_string();
        }

        self.lines.front().unwrap().to_string()
    }

    fn get_next_line(&mut self) -> String {
        if self.lines.is_empty() {
            return "".to_string();
        }

        self.lines.pop_front().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_sample_file() {
        let file_name = "./src/process/test_compiler_process_file.txt".to_string();
        let mut prepared_file = DummyProcess::new_from_file(&file_name);

        assert!(prepared_file.process());

        assert!(prepared_file.has_next_line());
        assert_eq!("Hello World!", prepared_file.get_next_line());
        assert!(prepared_file.has_next_line());
        assert_eq!("", prepared_file.get_next_line());
        assert!(prepared_file.has_next_line());
        assert_eq!("How are you?", prepared_file.get_next_line());
        assert!(!prepared_file.has_next_line());

        // Avoid unnecessary crashes.
        assert_eq!("", prepared_file.get_next_line());
    }

    #[test]
    fn read_not_existing_file() {
        let file_name = "./not_existing_file.txt".to_string();
        let mut prepared_file = DummyProcess::new_from_file(&file_name);

        assert!(!prepared_file.process());

        // Avoid unnecessary crashes.
        assert!(!prepared_file.has_next_line());
        assert_eq!("", prepared_file.get_next_line());
    }

    #[test]
    fn add_line() {
        let mut prepared_file = DummyProcess::new();

        prepared_file.add_line("Hello World!".to_string());
        prepared_file.add_line("".to_string());
        prepared_file.add_line("How are you?".to_string());

        assert!(prepared_file.process());

        assert!(prepared_file.has_next_line());
        assert_eq!("Hello World!", prepared_file.get_next_line());
        assert!(prepared_file.has_next_line());
        assert_eq!("", prepared_file.get_next_line());
        assert!(prepared_file.has_next_line());
        assert_eq!("How are you?", prepared_file.get_next_line());
        assert!(!prepared_file.has_next_line());

        // Avoid unnecessary crashes.
        assert_eq!("", prepared_file.get_next_line());
    }
}
