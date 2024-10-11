use serde::Deserialize;
use serde::Serialize;

    use super::helper::range::Range;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct VirtualFuncCall {
    name: String,
    qualified_name: String,
    base_qualified_name: String,
    qual_type: String,
    range: Range,
}

pub const VIRTUAL_FUNC_CALL_SQL_CREATE_TABLE: &str = "
CREATE TABLE virtual_func_calls (
    id                   INTEGER PRIMARY KEY AUTOINCREMENT,
    name                 TEXT NOT NULL,
    qualified_name       TEXT NOT NULL,
    base_qualified_name  TEXT NOT NULL,
    qual_type            TEXT NOT NULL,
    range_start_line     INTEGER,
    range_start_column   INTEGER,
    range_end_line       INTEGER,
    range_end_column     INTEGER,

    func_impl_id         INTEGER NULL,
    virtual_func_impl_id INTEGER NULL,

    FOREIGN KEY (func_impl_id) REFERENCES func_impls(id),
    FOREIGN KEY (virtual_func_impl_id) REFERENCES virtual_func_impls(id)
)
";

pub fn create_database_tables(db_connection: &rusqlite::Connection) {
    let _ = db_connection.execute_batch(VIRTUAL_FUNC_CALL_SQL_CREATE_TABLE);
}
