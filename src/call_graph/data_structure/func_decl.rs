use std::cell::RefCell;
use std::rc::Rc;

use rusqlite::params;

use crate::location::position::Position;
use crate::location::range::Range;

use super::super::database::database_sqlite_internal::DatabaseSqliteInternal;
use super::func_structure::FuncMentionType;
use super::func_structure::FuncStructure;
use super::helper::func_creation_args::FuncCreationArgs;

impl FuncStructure {
    pub fn create_func_decl(
        db_connection: &DatabaseSqliteInternal,
        args: &FuncCreationArgs,
        parent_id: (Option<u64>, Option<u64>, Option<u64>),
    ) -> Self {
        let mut stmt = db_connection
            .db
            .prepare(
                "
            INSERT INTO func_decls (name, qualified_name, qual_type,
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
            parent_id.2
        ]);

        FuncStructure::new(
            result.unwrap() as u64,
            Some(db_connection.clone()),
            args.name.clone(),
            args.qualified_name.clone(),
            None,
            args.qualified_type.clone(),
            args.range.clone(),
            Some(FuncMentionType::FuncDecl),
        )
    }

    pub fn get_func_decls(
        db_connection: &DatabaseSqliteInternal,
        parent_id: (Option<u64>, Option<u64>, Option<u64>),
    ) -> Vec<Rc<RefCell<FuncStructure>>> {
        let mut stmt = db_connection
            .db
            .prepare(
                "
            SELECT id, name, qualified_name, qual_type,
                range_start_line, range_start_column, range_end_line, range_end_column,
                cpp_file_id, hpp_file_id, cpp_class_id
            FROM func_decls
            WHERE cpp_file_id = ?
                OR hpp_file_id = ?
                OR cpp_class_id = ?",
            )
            .unwrap();
        let func_decl_iter = stmt
            .query_map(params![parent_id.0, parent_id.1, parent_id.2], |row| {
                Ok(FuncStructure::new(
                    row.get(0)?,
                    Some(db_connection.clone()),
                    row.get(1)?,
                    row.get(2)?,
                    None,
                    row.get(3)?,
                    Range::new(
                        Position::new(row.get(4)?, row.get(5)?),
                        Position::new(row.get(6)?, row.get(7)?),
                    ),
                    Some(FuncMentionType::FuncDecl),
                ))
            })
            .unwrap();

        let mut func_decls = Vec::new();
        for func_decl in func_decl_iter {
            func_decls.push(Rc::new(RefCell::new(func_decl.unwrap())));
        }

        func_decls
    }
}

pub const FUNC_DECL_SQL_CREATE_TABLE: &str = "
CREATE TABLE func_decls (
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
    let _ = db_connection.db.execute_batch(FUNC_DECL_SQL_CREATE_TABLE);
}
