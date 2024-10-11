use serde::Deserialize;
use serde::Serialize;

use super::func_decl::FuncDecl;
use super::func_impl::FuncImpl;
use super::virtual_func_decl::VirtualFuncDecl;
use super::virtual_func_impl::VirtualFuncImpl;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct CppClass {
    id: i64,

    name: String,
    parent_classes: Vec<String>,
    classes: Vec<CppClass>,
    func_decls: Vec<FuncDecl>,
    func_impls: Vec<FuncImpl>,
    virtual_func_decls: Vec<VirtualFuncDecl>,
    virtual_func_impls: Vec<VirtualFuncImpl>,
}

pub const CPP_CLASS_SQL_CREATE_TABLE: &str = "
CREATE TABLE cpp_classes (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    class_name   TEXT NOT NULL,

    cpp_file_id  INTEGER NULL,
    hpp_file_id  INTEGER NULL,
    cpp_class_id INTEGER NULL,

    FOREIGN KEY (cpp_file_id) REFERENCES cpp_files(id),
    FOREIGN KEY (hpp_file_id) REFERENCES hpp_files(id),
    FOREIGN KEY (cpp_class_id) REFERENCES cpp_classes(id)
)
";

pub const CPP_CLASS_2_CLASS_SQL_CREATE_TABLE: &str = "
CREATE TABLE cpp_classes_2_cpp_classes (
    parent_class_id INTEGER,
    child_class_id  INTEGER,

    PRIMARY KEY (parent_class_id, child_class_id),
    FOREIGN KEY (parent_class_id) REFERENCES cpp_classes(id),
    FOREIGN KEY (child_class_id) REFERENCES cpp_classes(id)
)
";

pub fn create_database_tables(db_connection: &rusqlite::Connection) {
    let _ = db_connection.execute_batch(CPP_CLASS_SQL_CREATE_TABLE);
    let _ = db_connection.execute_batch(CPP_CLASS_2_CLASS_SQL_CREATE_TABLE);
}
