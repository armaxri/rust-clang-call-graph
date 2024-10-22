use std::cell::RefCell;
use std::rc::Rc;

use rusqlite::params;
use serde::Deserialize;
use serde::Serialize;

use super::super::database::database_sqlite_internal::DatabaseSqliteInternal;
use super::helper::location::Location;
use super::helper::range::Range;
use super::helper::virtual_func_creation_args::VirtualFuncCreationArgs;
use super::FuncBasics;
use super::VirtualFuncBasics;

#[derive(Deserialize, Serialize, Debug, Clone, Eq)]
pub struct VirtualFuncDecl {
    id: u64,
    #[serde(skip)]
    _db_connection: Option<DatabaseSqliteInternal>,

    name: String,
    qualified_name: String,
    base_qualified_name: String,
    qual_type: String,
    range: Range,
}

impl PartialEq for VirtualFuncDecl {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
            && self.name == other.name
            && self.qualified_name == other.qualified_name
            && self.base_qualified_name == other.base_qualified_name
            && self.qual_type == other.qual_type
            && self.range == other.range;
    }
}

impl FuncBasics for VirtualFuncDecl {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_qualified_name(&self) -> &str {
        &self.qualified_name
    }

    fn get_qual_type(&self) -> &str {
        &self.qual_type
    }

    fn get_range(&self) -> &Range {
        &self.range
    }
}

impl VirtualFuncBasics for VirtualFuncDecl {
    fn get_base_qualified_name(&self) -> &str {
        &self.base_qualified_name
    }
}

impl VirtualFuncDecl {
    pub fn new(
        id: u64,
        db_connection: Option<DatabaseSqliteInternal>,
        name: String,
        qualified_name: String,
        base_qualified_name: String,
        qual_type: String,
        range: Range,
    ) -> Self {
        Self {
            id,
            _db_connection: db_connection,
            name,
            qualified_name,
            base_qualified_name,
            qual_type,
            range,
        }
    }

    pub fn create_virtual_func_decl(
        db_connection: &DatabaseSqliteInternal,
        args: &VirtualFuncCreationArgs,
        parent_id: (Option<u64>, Option<u64>, Option<u64>),
    ) -> Self {
        let mut stmt = db_connection
            .db
            .prepare(
                "
            INSERT INTO virtual_func_decls (name, qualified_name, base_qualified_name, qual_type,
                range_start_line, range_start_column, range_end_line, range_end_column,
                cpp_file_id, hpp_file_id, cpp_class_id)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            )
            .unwrap();
        let result = stmt.insert(params![
            args.name.clone(),
            args.qualified_name.clone(),
            args.base_qualified_name.clone(),
            args.qualified_type.clone(),
            args.range.start.line.to_string(),
            args.range.start.column.to_string(),
            args.range.end.line.to_string(),
            args.range.end.column.to_string(),
            parent_id.0,
            parent_id.1,
            parent_id.2,
        ]);

        VirtualFuncDecl::new(
            result.unwrap() as u64,
            Some(db_connection.clone()),
            args.name.clone(),
            args.qualified_name.clone(),
            args.base_qualified_name.clone(),
            args.qualified_type.clone(),
            args.range.clone(),
        )
    }

    pub fn get_virtual_func_decls(
        db_connection: &DatabaseSqliteInternal,
        parent_id: (Option<u64>, Option<u64>, Option<u64>),
    ) -> Vec<Rc<RefCell<VirtualFuncDecl>>> {
        let mut stmt = db_connection
            .db
            .prepare(
                "
            SELECT id, name, qualified_name, base_qualified_name, qual_type,
                range_start_line, range_start_column, range_end_line, range_end_column
            FROM virtual_func_decls
            WHERE cpp_file_id = ? OR hpp_file_id = ? OR cpp_class_id = ?",
            )
            .unwrap();
        let mut rows = stmt
            .query(params![parent_id.0, parent_id.1, parent_id.2])
            .unwrap();

        let mut virtual_func_decls = Vec::new();
        while let Some(row) = rows.next().unwrap() {
            virtual_func_decls.push(Rc::new(RefCell::new(VirtualFuncDecl::new(
                row.get(0).unwrap(),
                Some(db_connection.clone()),
                row.get(1).unwrap(),
                row.get(2).unwrap(),
                row.get(3).unwrap(),
                row.get(4).unwrap(),
                Range {
                    start: Location {
                        line: row.get(5).unwrap(),
                        column: row.get(6).unwrap(),
                    },
                    end: Location {
                        line: row.get(7).unwrap(),
                        column: row.get(8).unwrap(),
                    },
                },
            ))));
        }

        virtual_func_decls
    }
}

pub const VIRTUAL_FUNC_DECL_SQL_CREATE_TABLE: &str = "
CREATE TABLE virtual_func_decls (
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

    FOREIGN KEY (cpp_file_id) REFERENCES cpp_files(id) ON DELETE CASCADE,
    FOREIGN KEY (hpp_file_id) REFERENCES hpp_files(id) ON DELETE CASCADE,
    FOREIGN KEY (cpp_class_id) REFERENCES cpp_classes(id) ON DELETE CASCADE
)
";

pub fn create_database_tables(db_connection: &DatabaseSqliteInternal) {
    let _ = db_connection
        .db
        .execute_batch(VIRTUAL_FUNC_DECL_SQL_CREATE_TABLE);
}
