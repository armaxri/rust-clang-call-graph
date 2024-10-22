use std::cell::RefCell;
use std::rc::Rc;

use rusqlite::params;
use serde::Deserialize;
use serde::Serialize;

use super::super::database::database_sqlite_internal::DatabaseSqliteInternal;
use super::helper::func_creation_args::FuncCreationArgs;
use super::helper::range::Range;
use super::FuncBasics;

#[derive(Deserialize, Serialize, Debug, Clone, Eq)]
pub struct FuncDecl {
    id: u64,
    #[serde(skip)]
    _db_connection: Option<DatabaseSqliteInternal>,

    name: String,
    qualified_name: String,
    qual_type: String,
    range: Range,
}

impl PartialEq for FuncDecl {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
            && self.name == other.name
            && self.qualified_name == other.qualified_name
            && self.qual_type == other.qual_type
            && self.range == other.range;
    }
}

impl FuncBasics for FuncDecl {
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

impl FuncDecl {
    pub fn new(
        id: u64,
        db_connection: Option<DatabaseSqliteInternal>,
        name: String,
        qualified_name: String,
        qual_type: String,
        range: Range,
    ) -> Self {
        Self {
            id,
            _db_connection: db_connection,
            name,
            qualified_name,
            qual_type,
            range,
        }
    }

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
        let result = stmt.execute(params![
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

        FuncDecl::new(
            result.unwrap() as u64,
            Some(db_connection.clone()),
            args.name.clone(),
            args.qualified_name.clone(),
            args.qualified_type.clone(),
            args.range.clone(),
        )
    }

    pub fn get_func_decls(
        db_connection: &DatabaseSqliteInternal,
        parent_id: (Option<u64>, Option<u64>, Option<u64>),
    ) -> Vec<Rc<RefCell<FuncDecl>>> {
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
                Ok(FuncDecl::new(
                    row.get(0)?,
                    Some(db_connection.clone()),
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    Range::new(
                        super::helper::location::Location::new(row.get(4)?, row.get(5)?),
                        super::helper::location::Location::new(row.get(6)?, row.get(7)?),
                    ),
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

#[cfg(test)]
mod tests {
    use crate::call_graph::{
        data_structure::{cpp_file::CppFile, helper::location::Location},
        database::database_sqlite::create_in_memory_database,
    };

    use super::*;

    #[test]
    fn test_func_decl() {
        let func_decl = FuncDecl::new(
            0,
            None,
            "name".to_string(),
            "qualified_name".to_string(),
            "qual_type".to_string(),
            Range::new(Location::new(0, 0), Location::new(0, 0)),
        );

        assert_eq!(func_decl.id, 0);
        assert_eq!(func_decl.get_name(), "name");
        assert_eq!(func_decl.get_qualified_name(), "qualified_name");
        assert_eq!(func_decl.get_qual_type(), "qual_type");
        assert_eq!(
            func_decl.get_range(),
            &Range::new(Location::new(0, 0), Location::new(0, 0),)
        );
    }

    #[test]
    fn test_create_func_decl() {
        let db_connection = create_in_memory_database();
        create_database_tables(&db_connection);

        let func_decl = FuncDecl::create_func_decl(
            &db_connection,
            &FuncCreationArgs {
                name: "name".to_string(),
                qualified_name: "qualified_name".to_string(),
                qualified_type: "qual_type".to_string(),
                range: Range::new(Location::new(0, 0), Location::new(0, 0)),
            },
            (None, None, None),
        );

        assert_eq!(func_decl.get_name(), "name");
        assert_eq!(func_decl.get_qualified_name(), "qualified_name");
        assert_eq!(func_decl.get_qual_type(), "qual_type");
        assert_eq!(
            func_decl.get_range(),
            &Range::new(Location::new(0, 0), Location::new(0, 0),)
        );
    }

    #[test]
    fn test_get_func_decls() {
        let db_connection = create_in_memory_database();
        create_database_tables(&db_connection);

        CppFile::create_cpp_file(&db_connection, "cpp_file", None);

        FuncDecl::create_func_decl(
            &db_connection,
            &FuncCreationArgs {
                name: "name".to_string(),
                qualified_name: "qualified_name".to_string(),
                qualified_type: "qual_type".to_string(),
                range: Range::new(Location::new(0, 0), Location::new(0, 0)),
            },
            (Some(1), None, None),
        );

        let func_decls = FuncDecl::get_func_decls(&db_connection, (Some(1), None, None));
        assert_eq!(func_decls.len(), 1);
        assert_eq!(func_decls[0].borrow().get_name(), "name");
        assert_eq!(
            func_decls[0].borrow().get_qualified_name(),
            "qualified_name"
        );
        assert_eq!(func_decls[0].borrow().get_qual_type(), "qual_type");
        assert_eq!(
            func_decls[0].borrow().get_range(),
            &Range::new(Location::new(0, 0), Location::new(0, 0),)
        );
    }
}
