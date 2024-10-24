use std::cell::RefCell;
use std::rc::Rc;

use rusqlite::params;

use super::super::database::database_sqlite_internal::DatabaseSqliteInternal;
use super::func_structure::{FuncMentionType, FuncStructure};
use super::helper::func_creation_args::FuncCreationArgs;
use super::helper::location::Location;
use super::helper::range::Range;

impl FuncStructure {
    pub fn create_func_impl(
        db_connection: &DatabaseSqliteInternal,
        args: &FuncCreationArgs,
        parent_id: (Option<u64>, Option<u64>, Option<u64>),
    ) -> Self {
        let mut stmt = db_connection
            .db
            .prepare(
                "
        INSERT INTO func_impls (name, qualified_name, qual_type,
            range_start_line, range_start_column, range_end_line, range_end_column,
            cpp_file_id, hpp_file_id, cpp_class_id)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            )
            .unwrap();
        let result = stmt.insert(params![
            args.name.clone(),
            args.qualified_name.clone(),
            args.qualified_type.clone(),
            args.range.start.line.to_string(),
            args.range.start.column.to_string(),
            args.range.end.line.to_string(),
            args.range.end.column.to_string(),
            parent_id.0,
            parent_id.1,
            parent_id.2,
        ]);

        FuncStructure::new(
            result.unwrap() as u64,
            Some(db_connection.clone()),
            args.name.clone(),
            args.qualified_name.clone(),
            None,
            args.qualified_type.clone(),
            args.range.clone(),
            Some(FuncMentionType::FuncImpl),
        )
    }

    pub fn get_func_impls(
        db_connection: &DatabaseSqliteInternal,
        parent_id: (Option<u64>, Option<u64>, Option<u64>),
    ) -> Vec<Rc<RefCell<FuncStructure>>> {
        let mut stmt = db_connection
            .db
            .prepare(
                "
            SELECT id, name, qualified_name, qual_type,
                range_start_line, range_start_column, range_end_line, range_end_column
            FROM func_impls
            WHERE cpp_file_id = ? OR hpp_file_id = ? OR cpp_class_id = ?",
            )
            .unwrap();
        let mut rows = stmt
            .query(params![parent_id.0, parent_id.1, parent_id.2])
            .unwrap();

        let mut virtual_func_decls = Vec::new();
        while let Some(row) = rows.next().unwrap() {
            virtual_func_decls.push(Rc::new(RefCell::new(FuncStructure::new(
                row.get(0).unwrap(),
                Some(db_connection.clone()),
                row.get(1).unwrap(),
                row.get(2).unwrap(),
                None,
                row.get(3).unwrap(),
                Range {
                    start: Location {
                        line: row.get(4).unwrap(),
                        column: row.get(5).unwrap(),
                    },
                    end: Location {
                        line: row.get(6).unwrap(),
                        column: row.get(7).unwrap(),
                    },
                },
                Some(FuncMentionType::FuncImpl),
            ))));
        }

        virtual_func_decls
    }
}

pub const FUNC_IMPL_SQL_CREATE_TABLE: &str = "
CREATE TABLE func_impls (
    id                 INTEGER PRIMARY KEY AUTOINCREMENT,
    name               TEXT NOT NULL,
    qualified_name     TEXT NOT NULL,
    qual_type          TEXT NOT NULL,
    range_start_line   INTEGER,
    range_start_column INTEGER,
    range_end_line     INTEGER,
    range_end_column   INTEGER,

    cpp_file_id        INTEGER NULL,
    hpp_file_id        INTEGER NULL,
    cpp_class_id       INTEGER NULL,

    FOREIGN KEY (cpp_file_id) REFERENCES cpp_files(id) ON DELETE CASCADE,
    FOREIGN KEY (hpp_file_id) REFERENCES hpp_files(id) ON DELETE CASCADE,
    FOREIGN KEY (cpp_class_id) REFERENCES cpp_classes(id) ON DELETE CASCADE
)
";

pub fn create_database_tables(db_connection: &DatabaseSqliteInternal) {
    let _ = db_connection.db.execute_batch(FUNC_IMPL_SQL_CREATE_TABLE);
}
