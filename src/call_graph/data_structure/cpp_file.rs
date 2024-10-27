use std::cell::RefCell;
use std::rc::Rc;

use rusqlite::params;

use super::super::database::database_sqlite_internal::DatabaseSqliteInternal;
use super::file_structure::FileStructure;
use super::MainDeclPosition;

impl FileStructure {
    pub fn create_cpp_file(
        db_connection: &DatabaseSqliteInternal,
        name: &str,
        last_analyzed: Option<usize>,
    ) -> Rc<RefCell<FileStructure>> {
        let time = match last_analyzed {
            Some(time) => time,
            None => std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize,
        };

        let mut stmt = db_connection
            .db
            .prepare(
                "
            INSERT INTO cpp_files (file_name, last_analyzed)
            VALUES (?, ?)",
            )
            .unwrap();
        let result = stmt.insert(params![name, time,]);

        Rc::new(RefCell::new(FileStructure::new(
            result.unwrap() as u64,
            Some(db_connection.clone()),
            name.to_string(),
            time,
            false,
        )))
    }

    pub fn get_cpp_file(
        db_connection: &DatabaseSqliteInternal,
        name: &str,
    ) -> Option<Rc<RefCell<FileStructure>>> {
        let mut stmt = db_connection
            .db
            .prepare(
                "
            SELECT id, last_analyzed
            FROM cpp_files
            WHERE file_name = ?",
            )
            .unwrap();
        let mut rows = stmt.query(params![name]).unwrap();

        if let Ok(Some(row)) = rows.next() {
            let id: u64 = row.get(0).unwrap();
            let last_analyzed: usize = row.get(1).unwrap();

            Some(Rc::new(RefCell::new(FileStructure::new(
                id,
                Some(db_connection.clone()),
                name.to_string(),
                last_analyzed,
                false,
            ))))
        } else {
            None
        }
    }

    pub fn get_cpp_files(
        db_connection: &DatabaseSqliteInternal,
    ) -> Vec<Rc<RefCell<FileStructure>>> {
        let mut stmt = db_connection
            .db
            .prepare(
                "
            SELECT id, file_name, last_analyzed
            FROM cpp_files",
            )
            .unwrap();
        let mut rows = stmt.query(params![]).unwrap();

        let mut cpp_files = Vec::new();
        while let Ok(Some(row)) = rows.next() {
            let id: u64 = row.get(0).unwrap();
            let name: String = row.get(1).unwrap();
            let last_analyzed: usize = row.get(2).unwrap();

            cpp_files.push(Rc::new(RefCell::new(FileStructure::new(
                id,
                Some(db_connection.clone()),
                name,
                last_analyzed,
                false,
            ))));
        }

        cpp_files
    }

    pub fn set_last_analyzed_inner_cpp(&mut self, last_analyzed: usize) {
        let binding = self.get_db_connection();
        let mut stmt = binding
            .as_ref()
            .unwrap()
            .db
            .prepare(
                "
            UPDATE cpp_files
            SET last_analyzed = ?
            WHERE id = ?",
            )
            .unwrap();
        stmt.execute(params![last_analyzed, self.get_id()]).unwrap();
    }

    pub fn remove_cpp_file_and_depending_content(&self) {
        let _ = self.get_db_connection().as_ref().unwrap().db.execute(
            "
            DELETE FROM cpp_files
            WHERE id = ?",
            params![self.get_id()],
        );
    }
}

pub const CPP_FILE_SQL_CREATE_TABLE: &str = "
CREATE TABLE cpp_files (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    file_name     TEXT UNIQUE NOT NULL,
    last_analyzed INTEGER
)
";

pub fn create_database_tables(db_connection: &DatabaseSqliteInternal) {
    let _ = db_connection.db.execute_batch(CPP_FILE_SQL_CREATE_TABLE);
}
