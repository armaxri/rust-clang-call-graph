use serde::Deserialize;
use serde::Serialize;

use super::cpp_class::CppClass;
use super::func_decl::FuncDecl;
use super::func_impl::FuncImpl;
use super::virtual_func_impl::VirtualFuncImpl;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct CppFile {
    pub name: String,
    pub last_analyzed: i64,
    pub classes: Vec<CppClass>,
    pub func_decls: Vec<FuncDecl>,
    pub func_impls: Vec<FuncImpl>,
    pub virtual_func_impls: Vec<VirtualFuncImpl>,
}

pub const CPP_FILE_SQL_CREATE_TABLE: &str = "
CREATE TABLE cpp_files (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    file_name     TEXT UNIQUE NOT NULL,
    last_analyzed INTEGER
)
";
