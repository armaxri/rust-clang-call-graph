use std::cell::RefCell;
use std::rc::Rc;

use serde::Deserialize;
use serde::Serialize;

use super::super::data_structure::{cpp_file::CppFile, hpp_file::HppFile};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct DatabaseContent {
    pub cpp_files: Vec<Rc<RefCell<CppFile>>>,
    pub hpp_files: Vec<Rc<RefCell<HppFile>>>,
}

impl DatabaseContent {
    pub fn new(cpp_files: Vec<Rc<RefCell<CppFile>>>, hpp_files: Vec<Rc<RefCell<HppFile>>>) -> Self {
        DatabaseContent {
            cpp_files: cpp_files,
            hpp_files: hpp_files,
        }
    }

    pub fn load_from_file(file: &str) -> Self {
        let content = std::fs::read_to_string(file).unwrap();
        serde_json::from_str(&content).unwrap()
    }

    pub fn save_to_file(&self, file: &str) {
        let content = serde_json::to_string_pretty(&self).unwrap();
        std::fs::write(file, content).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{call_graph::database::database_sqlite::DatabaseSqlite, file_in_directory};

    use super::*;

    #[test]
    fn test_database_content() {
        let database_sqlite = DatabaseSqlite::create_database(
            &PathBuf::from(file_in_directory!("database_content_test.db")),
            false,
        );

        let sqlite_content = database_sqlite.get_db_content();

        let json_content =
            DatabaseContent::load_from_file(&file_in_directory!("database_content_test.json"));

        assert_eq!(sqlite_content, json_content);
    }
}
