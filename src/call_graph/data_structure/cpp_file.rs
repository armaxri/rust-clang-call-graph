use serde::Deserialize;
use serde::Serialize;

use super::cpp_class::CppClass;
use super::func_decl::FuncDecl;
use super::func_impl::FuncImpl;
use super::virtual_func_impl::VirtualFuncImpl;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct CppFile {
    name: String,
    last_analyzed: i64,
    classes: Vec<CppClass>,
    func_decls: Vec<FuncDecl>,
    func_impls: Vec<FuncImpl>,
    virtual_func_impls: Vec<VirtualFuncImpl>,
}

pub const CPP_FILE_SQL_CREATE_TABLE: &str = "
CREATE TABLE cpp_files (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    file_name     TEXT UNIQUE NOT NULL,
    last_analyzed INTEGER
)
";

pub fn create_database_tables(db_connection: &rusqlite::Connection) {
    let _ = db_connection.execute_batch(CPP_FILE_SQL_CREATE_TABLE);
}
