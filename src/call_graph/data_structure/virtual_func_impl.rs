use serde::Deserialize;
use serde::Serialize;

use super::func_call::FuncCall;
    use super::helper::range::Range;
use super::virtual_func_call::VirtualFuncCall;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct VirtualFuncImpl {
    name: String,
    qualified_name: String,
    base_qualified_name: String,
    qual_type: String,
    range: Range,
    func_calls: Vec<FuncCall>,
    virtual_func_calls: Vec<VirtualFuncCall>,
}

pub const VIRTUAL_FUNC_IMPL_SQL_CREATE_TABLE: &str = "
CREATE TABLE virtual_func_impls (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    name                TEXT NOT NULL,
    qualified_name      TEXT NOT NULL,
    base_qualified_name TEXT NOT NULL,
    qual_type           TEXT NOT NULL,
    range_start_line    INTEGER,
    range_start_column  INTEGER,
    range_end_line      INTEGER,
    range_end_column    INTEGER,

    cpp_file_id         INTEGER NULL,
    hpp_file_id         INTEGER NULL,
    cpp_class_id        INTEGER NULL,

    FOREIGN KEY (cpp_file_id) REFERENCES cpp_files(id),
    FOREIGN KEY (hpp_file_id) REFERENCES hpp_files(id),
    FOREIGN KEY (cpp_class_id) REFERENCES cpp_classes(id)
)
";

pub fn create_database_tables(db_connection: &rusqlite::Connection) {
    let _ = db_connection.execute_batch(VIRTUAL_FUNC_IMPL_SQL_CREATE_TABLE);
}
