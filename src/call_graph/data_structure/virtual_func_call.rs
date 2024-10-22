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
pub struct VirtualFuncCall {
    id: u64,
    #[serde(skip)]
    _db_connection: Option<DatabaseSqliteInternal>,

    name: String,
    qualified_name: String,
    base_qualified_name: String,
    qual_type: String,
    range: Range,
}

impl PartialEq for VirtualFuncCall {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
            && self.name == other.name
            && self.qualified_name == other.qualified_name
            && self.base_qualified_name == other.base_qualified_name
            && self.qual_type == other.qual_type
            && self.range == other.range;
    }
}

impl FuncBasics for VirtualFuncCall {
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

impl VirtualFuncBasics for VirtualFuncCall {
    fn get_base_qualified_name(&self) -> &str {
        &self.base_qualified_name
    }
}

impl VirtualFuncCall {
    pub fn new(
        id: u64,
        db_connection: Option<DatabaseSqliteInternal>,
        name: String,
        qualified_name: String,
        base_qualified_name: String,
        qual_type: String,
        range: Range,
    ) -> Self {
        VirtualFuncCall {
            id,
            _db_connection: db_connection,
            name,
            qualified_name,
            base_qualified_name,
            qual_type,
            range,
        }
    }

    pub fn create_virtual_func_call(
        db_connection: &DatabaseSqliteInternal,
        args: &VirtualFuncCreationArgs,
        parent_id: (Option<u64>, Option<u64>),
    ) -> Self {
        let mut stmt = db_connection
            .db
            .prepare(
                "
            INSERT INTO virtual_func_calls (name, qualified_name, base_qualified_name, qual_type,
                range_start_line, range_start_column, range_end_line, range_end_column,
                func_impl_id, virtual_func_impl_id)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
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
        ]);

        VirtualFuncCall::new(
            result.unwrap() as u64,
            Some(db_connection.clone()),
            args.name.clone(),
            args.qualified_name.clone(),
            args.base_qualified_name.clone(),
            args.qualified_type.clone(),
            args.range.clone(),
        )
    }

    pub fn get_virtual_func_calls(
        db_connection: &DatabaseSqliteInternal,
        parent_id: (Option<u64>, Option<u64>),
    ) -> Vec<Rc<RefCell<VirtualFuncCall>>> {
        let mut stmt = db_connection
            .db
            .prepare(
                "
            SELECT id, name, qualified_name, base_qualified_name, qual_type,
                range_start_line, range_start_column, range_end_line, range_end_column
            FROM virtual_func_calls
            WHERE func_impl_id = ?
                OR virtual_func_impl_id = ?",
            )
            .unwrap();
        let rows = stmt
            .query_map([parent_id.0, parent_id.1], |row| {
                Ok(VirtualFuncCall::new(
                    row.get(0).unwrap(),
                    Some(db_connection.clone()),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                    row.get(3).unwrap(),
                    row.get(4).unwrap(),
                    Range::new(
                        Location::new(row.get(5).unwrap(), row.get(6).unwrap()),
                        Location::new(row.get(7).unwrap(), row.get(8).unwrap()),
                    ),
                ))
            })
            .unwrap();

        let mut func_calls = Vec::new();
        for func_call in rows {
            func_calls.push(Rc::new(RefCell::new(func_call.unwrap())));
        }

        func_calls
    }

    pub fn get_matching_virtual_calls(
        db_connection: &DatabaseSqliteInternal,
        func: &dyn FuncBasics,
    ) -> Vec<Rc<RefCell<VirtualFuncCall>>> {
        let mut stmt = db_connection
            .db
            .prepare(
                "
            SELECT id, name, qualified_name, base_qualified_name, qual_type,
                range_start_line, range_start_column, range_end_line, range_end_column
            FROM virtual_func_calls
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
                    Ok(VirtualFuncCall::new(
                        row.get(0).unwrap(),
                        Some(db_connection.clone()),
                        row.get(1).unwrap(),
                        row.get(2).unwrap(),
                        row.get(3).unwrap(),
                        row.get(4).unwrap(),
                        Range::new(
                            Location::new(row.get(5).unwrap(), row.get(6).unwrap()),
                            Location::new(row.get(7).unwrap(), row.get(8).unwrap()),
                        ),
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

    FOREIGN KEY (func_impl_id) REFERENCES func_impls(id) ON DELETE CASCADE,
    FOREIGN KEY (virtual_func_impl_id) REFERENCES virtual_func_impls(id) ON DELETE CASCADE
)
";

pub fn create_database_tables(db_connection: &DatabaseSqliteInternal) {
    let _ = db_connection
        .db
        .execute_batch(VIRTUAL_FUNC_CALL_SQL_CREATE_TABLE);
}

#[cfg(test)]
mod tests {
    use crate::call_graph::{
        data_structure::{
            func_impl::FuncImpl, helper::func_creation_args::FuncCreationArgs,
            virtual_func_impl::VirtualFuncImpl,
        },
        database::database_sqlite::create_in_memory_database,
    };

    use super::*;

    #[test]
    fn test_new() {
        let virtual_func_call = VirtualFuncCall::new(
            0,
            None,
            "name".to_string(),
            "qualified_name".to_string(),
            "base_qualified_name".to_string(),
            "qual_type".to_string(),
            Range::new(Location::new(0, 0), Location::new(0, 0)),
        );

        assert_eq!(virtual_func_call.id, 0);
        assert_eq!(virtual_func_call.name, "name");
        assert_eq!(virtual_func_call.qualified_name, "qualified_name");
        assert_eq!(virtual_func_call.base_qualified_name, "base_qualified_name");
        assert_eq!(virtual_func_call.qual_type, "qual_type");
        assert_eq!(
            virtual_func_call.range,
            Range::new(Location::new(0, 0), Location::new(0, 0),)
        );
    }

    #[test]
    fn test_get_virtual_func_calls() {
        let db_connection = create_in_memory_database();

        FuncImpl::create_func_impl(
            &db_connection,
            &FuncCreationArgs::new(
                "func impl",
                "foo",
                "bar",
                Range::new(Location::new(0, 0), Location::new(0, 0)),
            ),
            (None, None, None),
        );
        VirtualFuncImpl::create_virtual_func_impl(
            &db_connection,
            &VirtualFuncCreationArgs::new(
                "virtual func impl",
                "foo",
                "bar",
                "cookie",
                Range::new(Location::new(0, 0), Location::new(0, 0)),
            ),
            (None, None, None),
        );

        VirtualFuncCall::create_virtual_func_call(
            &db_connection,
            &VirtualFuncCreationArgs::new(
                "name1",
                "qualified_name1",
                "base_qualified_name",
                "qual_type1",
                Range::new(Location::new(0, 0), Location::new(0, 0)),
            ),
            (Some(1), None),
        );
        VirtualFuncCall::create_virtual_func_call(
            &db_connection,
            &VirtualFuncCreationArgs::new(
                "name2",
                "qualified_name2",
                "base_qualified_name",
                "qual_type2",
                Range::new(Location::new(0, 0), Location::new(0, 0)),
            ),
            (None, Some(1)),
        );
        VirtualFuncCall::create_virtual_func_call(
            &db_connection,
            &VirtualFuncCreationArgs::new(
                "name3",
                "qualified_name4",
                "base_qualified_name",
                "qual_type1",
                Range::new(Location::new(0, 0), Location::new(0, 0)),
            ),
            (None, None),
        );

        let virtual_func_calls =
            VirtualFuncCall::get_virtual_func_calls(&db_connection, (Some(1), None));

        assert_eq!(virtual_func_calls.len(), 1);
        assert_eq!(virtual_func_calls[0].borrow().id, 1);
        assert_eq!(virtual_func_calls[0].borrow().name, "name1");
        assert_eq!(
            virtual_func_calls[0].borrow().qualified_name,
            "qualified_name1"
        );
        assert_eq!(
            virtual_func_calls[0].borrow().base_qualified_name,
            "base_qualified_name"
        );
        assert_eq!(virtual_func_calls[0].borrow().qual_type, "qual_type1");
        assert_eq!(
            virtual_func_calls[0].borrow().range,
            Range::new(Location::new(0, 0), Location::new(0, 0),)
        );
    }

    #[test]
    fn test_get_matching_virtual_calls() {
        let db_connection = create_in_memory_database();
        VirtualFuncCall::create_virtual_func_call(
            &db_connection,
            &VirtualFuncCreationArgs::new(
                "name1",
                "qualified_name1",
                "base_qualified_name",
                "qual_type1",
                Range::new(Location::new(1, 0), Location::new(0, 0)),
            ),
            (None, None),
        );
        VirtualFuncCall::create_virtual_func_call(
            &db_connection,
            &VirtualFuncCreationArgs::new(
                "name2",
                "qualified_name2",
                "base_qualified_name",
                "qual_type2",
                Range::new(Location::new(1, 1), Location::new(0, 0)),
            ),
            (None, None),
        );
        VirtualFuncCall::create_virtual_func_call(
            &db_connection,
            &VirtualFuncCreationArgs::new(
                "name1",
                "qualified_name1",
                "base_qualified_name",
                "qual_type1",
                Range::new(Location::new(0, 0), Location::new(1, 0)),
            ),
            (None, None),
        );

        let func_call = VirtualFuncCall::new(
            0,
            Some(db_connection.clone()),
            "name1".to_string(),
            "qualified_name1".to_string(),
            "base_qualified_name".to_string(),
            "qual_type1".to_string(),
            Range::new(Location::new(0, 0), Location::new(0, 0)),
        );
        let virtual_func_calls =
            VirtualFuncCall::get_matching_virtual_calls(&db_connection, &func_call);

        assert_eq!(virtual_func_calls.len(), 2);
        assert_eq!(virtual_func_calls[0].borrow().id, 1);
        assert_eq!(virtual_func_calls[0].borrow().name, "name1");
        assert_eq!(
            virtual_func_calls[0].borrow().qualified_name,
            "qualified_name1"
        );
        assert_eq!(
            virtual_func_calls[0].borrow().base_qualified_name,
            "base_qualified_name"
        );
        assert_eq!(virtual_func_calls[0].borrow().qual_type, "qual_type1");
        assert_eq!(
            virtual_func_calls[0].borrow().range,
            Range::new(Location::new(1, 0), Location::new(0, 0),)
        );

        assert_eq!(virtual_func_calls[1].borrow().id, 3);
        assert_eq!(virtual_func_calls[1].borrow().name, "name1");
        assert_eq!(
            virtual_func_calls[1].borrow().qualified_name,
            "qualified_name1"
        );
        assert_eq!(
            virtual_func_calls[1].borrow().base_qualified_name,
            "base_qualified_name"
        );
        assert_eq!(virtual_func_calls[1].borrow().qual_type, "qual_type1");
        assert_eq!(
            virtual_func_calls[1].borrow().range,
            Range::new(Location::new(0, 0), Location::new(1, 0),)
        );
    }
}
