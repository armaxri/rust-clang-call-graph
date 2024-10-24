use std::cell::RefCell;
use std::rc::Rc;

use rusqlite::params;

use crate::location::position::Position;
use crate::location::range::Range;

use super::super::database::database_sqlite_internal::DatabaseSqliteInternal;
use super::func_structure::FuncMentionType;
use super::func_structure::FuncStructure;
use super::helper::func_creation_args::FuncCreationArgs;
use super::FuncBasics;

impl FuncStructure {
    pub fn create_func_call(
        db_connection: &DatabaseSqliteInternal,
        args: &FuncCreationArgs,
        parent_id: (Option<u64>, Option<u64>),
    ) -> Self {
        let mut stmt = db_connection
            .db
            .prepare(
                "
            INSERT INTO func_calls (name, qualified_name, qual_type,
                range_start_line, range_start_column, range_end_line, range_end_column,
                func_impl_id, virtual_func_impl_id)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
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
        ]);

        FuncStructure::new(
            result.unwrap() as u64,
            Some(db_connection.clone()),
            args.name.clone(),
            args.qualified_name.clone(),
            None,
            args.qualified_type.clone(),
            args.range.clone(),
            Some(FuncMentionType::FuncCall),
        )
    }

    pub fn get_func_calls_from_id(
        db_connection: &DatabaseSqliteInternal,
        parent_id: (Option<u64>, Option<u64>),
    ) -> Vec<Rc<RefCell<FuncStructure>>> {
        let mut stmt = db_connection
            .db
            .prepare(
                "
            SELECT id, name, qualified_name, qual_type,
                range_start_line, range_start_column, range_end_line, range_end_column
            FROM func_calls
            WHERE func_impl_id = ?
                OR virtual_func_impl_id = ?",
            )
            .unwrap();
        let rows = stmt
            .query_map(params![parent_id.0, parent_id.1], |row| {
                Ok(FuncStructure::new(
                    row.get(0).unwrap(),
                    Some(db_connection.clone()),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                    None,
                    row.get(3).unwrap(),
                    Range::new(
                        Position::new(row.get(4).unwrap(), row.get(5).unwrap()),
                        Position::new(row.get(6).unwrap(), row.get(7).unwrap()),
                    ),
                    Some(FuncMentionType::FuncCall),
                ))
            })
            .unwrap();

        let mut func_calls = Vec::new();
        for func_call in rows {
            func_calls.push(Rc::new(RefCell::new(func_call.unwrap())));
        }

        func_calls
    }

    pub fn get_matching_calls(
        db_connection: &DatabaseSqliteInternal,
        func: &dyn FuncBasics,
    ) -> Vec<Rc<RefCell<FuncStructure>>> {
        let mut stmt = db_connection
            .db
            .prepare(
                "
            SELECT id, name, qualified_name, qual_type,
                range_start_line, range_start_column, range_end_line, range_end_column
            FROM func_calls
            WHERE name = ? AND qualified_name = ? AND qual_type = ?",
            )
            .unwrap();
        let rows = stmt
            .query_map(
                [
                    func.get_name(),
                    func.get_qualified_name(),
                    func.get_qual_type(),
                ],
                |row| {
                    Ok(FuncStructure::new(
                        row.get(0).unwrap(),
                        Some(db_connection.clone()),
                        row.get(1).unwrap(),
                        row.get(2).unwrap(),
                        None,
                        row.get(3).unwrap(),
                        Range::new(
                            Position::new(row.get(4).unwrap(), row.get(5).unwrap()),
                            Position::new(row.get(6).unwrap(), row.get(7).unwrap()),
                        ),
                        Some(FuncMentionType::FuncCall),
                    ))
                },
            )
            .unwrap();

        let mut func_calls = Vec::new();
        for func_call in rows {
            func_calls.push(Rc::new(RefCell::new(func_call.unwrap())));
        }

        func_calls
    }
}

pub const FUNC_CALL_SQL_CREATE_TABLE: &str = "
CREATE TABLE func_calls (
    id                   INTEGER PRIMARY KEY AUTOINCREMENT,
    name                 TEXT NOT NULL,
    qualified_name       TEXT NOT NULL,
    qual_type            TEXT NOT NULL,
    range_start_line     INTEGER,
    range_start_column   INTEGER,
    range_end_line       INTEGER,
    range_end_column     INTEGER,

    func_impl_id         INTEGER NULL,
    virtual_func_impl_id INTEGER NULL,

    FOREIGN KEY (func_impl_id) REFERENCES func_impls(id) ON DELETE CASCADE,
    FOREIGN KEY (virtual_func_impl_id) REFERENCES virtual_func_impls(id) ON DELETE CASCADE
)
";

pub fn create_database_tables(db_connection: &DatabaseSqliteInternal) {
    let _ = db_connection.db.execute_batch(FUNC_CALL_SQL_CREATE_TABLE);
}
