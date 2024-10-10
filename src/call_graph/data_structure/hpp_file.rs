use serde::Deserialize;
use serde::Serialize;

use super::cpp_class::CppClass;
use super::func_decl::FuncDecl;
use super::func_impl::FuncImpl;
use super::virtual_func_impl::VirtualFuncImpl;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct HppFile {
    pub name: String,
    pub last_analyzed: i64,
    pub classes: Vec<CppClass>,
    pub func_decls: Vec<FuncDecl>,
    pub func_impls: Vec<FuncImpl>,
    pub virtual_func_impls: Vec<VirtualFuncImpl>,
    pub referenced_from_files: Vec<String>,
}

pub const HPP_FILE_SQL_CREATE_TABLE: &str = "
CREATE TABLE hpp_files (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    file_name     TEXT UNIQUE NOT NULL,
    last_analyzed INTEGER
)
";

pub const CPP_FILE_2_HPP_FILE_SQL_CREATE_TABLE: &str = "
CREATE TABLE cpp_files_2_hpp_files (
    cpp_file_id INTEGER,
    hpp_file_id INTEGER,

    PRIMARY KEY (cpp_file_id, hpp_file_id),
    FOREIGN KEY (cpp_file_id) REFERENCES cpp_files (id),
    FOREIGN KEY (hpp_file_id) REFERENCES hpp_files (id)
)
";

pub const HPP_FILE_2_HPP_FILE_SQL_CREATE_TABLE: &str = "
CREATE TABLE hpp_files_2_hpp_files (
    current_hpp_file_id INTEGER,
    hpp_file_id         INTEGER,

    PRIMARY KEY (current_hpp_file_id, hpp_file_id),
    FOREIGN KEY (current_hpp_file_id) REFERENCES hpp_files (id),
    FOREIGN KEY (hpp_file_id) REFERENCES hpp_files (id)
)
";

pub fn create_database_tables(db_connection: &rusqlite::Connection) {
    let _ = db_connection.execute_batch(HPP_FILE_SQL_CREATE_TABLE);
    let _ = db_connection.execute_batch(CPP_FILE_2_HPP_FILE_SQL_CREATE_TABLE);
    let _ = db_connection.execute_batch(HPP_FILE_2_HPP_FILE_SQL_CREATE_TABLE);
}
