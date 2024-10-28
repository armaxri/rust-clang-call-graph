use std::cell::RefCell;
use std::rc::Rc;

use rusqlite::params;

use super::super::database::database_sqlite_internal::DatabaseSqliteInternal;
use super::file_structure::FileStructure;
use super::MainDeclPosition;

impl FileStructure {
    pub fn create_hpp_file(
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
            INSERT INTO hpp_files (file_name, last_analyzed)
            VALUES (?, ?)",
            )
            .unwrap();
        let result = stmt.insert(params![name, time,]);

        Rc::new(RefCell::new(FileStructure::new(
            result.unwrap() as u64,
            Some(db_connection.clone()),
            name.to_string(),
            time,
            true,
        )))
    }

    pub fn get_hpp_file(
        db_connection: &DatabaseSqliteInternal,
        name: &str,
    ) -> Option<Rc<RefCell<FileStructure>>> {
        let mut stmt = db_connection
            .db
            .prepare(
                "
            SELECT id, last_analyzed
            FROM hpp_files
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
                true,
            ))))
        } else {
            None
        }
    }

    pub fn get_hpp_files(
        db_connection: &DatabaseSqliteInternal,
    ) -> Vec<Rc<RefCell<FileStructure>>> {
        let mut stmt = db_connection
            .db
            .prepare(
                "
            SELECT id, file_name, last_analyzed
            FROM hpp_files",
            )
            .unwrap();
        let mut rows = stmt.query(params![]).unwrap();

        let mut hpp_files = Vec::new();
        while let Ok(Some(row)) = rows.next() {
            let id: u64 = row.get(0).unwrap();
            let name: String = row.get(1).unwrap();
            let last_analyzed: usize = row.get(2).unwrap();

            hpp_files.push(Rc::new(RefCell::new(FileStructure::new(
                id,
                Some(db_connection.clone()),
                name,
                last_analyzed,
                true,
            ))));
        }

        hpp_files
    }

    pub fn set_last_analyzed_inner_hpp(&self, last_analyzed: usize) {
        let binding = self.get_db_connection();
        let mut stmt = binding
            .as_ref()
            .unwrap()
            .db
            .prepare(
                "
            UPDATE hpp_files
            SET last_analyzed = ?
            WHERE id = ?",
            )
            .unwrap();
        stmt.execute(params![last_analyzed, self.get_id()]).unwrap();
    }

    pub fn remove_hpp_file_and_depending_content(&self) {
        let _ = self.get_db_connection().as_ref().unwrap().db.execute(
            "
            DELETE FROM hpp_files
            WHERE id = ?",
            params![self.get_id()],
        );
    }

    pub fn add_referenced_from_header_file_inner(&self, file: &Rc<RefCell<FileStructure>>) {
        let binding = self.get_db_connection();
        let mut stmt = binding
            .as_ref()
            .unwrap()
            .db
            .prepare(
                "
            INSERT INTO hpp_files_2_hpp_files (current_hpp_file_id, hpp_file_id)
            VALUES (?, ?)",
            )
            .unwrap();
        stmt.insert(params![self.get_id(), file.borrow().get_id()])
            .unwrap();
    }

    pub fn read_referenced_from_header_files(&self) -> Vec<String> {
        let binding = self.get_db_connection();
        let mut stmt = binding
            .as_ref()
            .unwrap()
            .db
            .prepare(
                "
            SELECT h.file_name
            FROM hpp_files AS h
            JOIN hpp_files_2_hpp_files AS h2h ON h.id = h2h.hpp_file_id
            WHERE h2h.current_hpp_file_id = ?",
            )
            .unwrap();
        let mut rows = stmt.query(params![self.get_id()]).unwrap();

        let mut referenced_from_header_files = Vec::new();
        while let Ok(Some(row)) = rows.next() {
            let file_name: String = row.get(0).unwrap();
            referenced_from_header_files.push(file_name);
        }

        referenced_from_header_files
    }

    pub fn add_referenced_from_source_file_inner(&mut self, file: &Rc<RefCell<FileStructure>>) {
        let binding = self.get_db_connection();
        let mut stmt = binding
            .as_ref()
            .unwrap()
            .db
            .prepare(
                "
            INSERT INTO cpp_files_2_hpp_files (cpp_file_id, hpp_file_id)
            VALUES (?, ?)",
            )
            .unwrap();
        stmt.insert(params![file.borrow().get_id(), self.get_id()])
            .unwrap();
    }

    pub fn read_referenced_from_source_files(&mut self) -> Vec<String> {
        let binding = self.get_db_connection();
        let mut stmt = binding
            .as_ref()
            .unwrap()
            .db
            .prepare(
                "
            SELECT c.file_name
            FROM cpp_files AS c
            JOIN cpp_files_2_hpp_files AS c2h ON c.id = c2h.cpp_file_id
            WHERE c2h.hpp_file_id = ?",
            )
            .unwrap();
        let mut rows = stmt.query(params![self.get_id()]).unwrap();

        let mut referenced_from_source_files = Vec::new();
        while let Ok(Some(row)) = rows.next() {
            let file_name: String = row.get(0).unwrap();
            referenced_from_source_files.push(file_name);
        }

        referenced_from_source_files
    }
}

pub const HPP_FILE_SQL_CREATE_TABLE: &str = "
CREATE TABLE hpp_files (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    file_name     TEXT UNIQUE NOT NULL,
    last_analyzed INTEGER
)
";

pub const CPP_FILE_2_HPP_FILE_SQL_CREATE_TABLE: &str = "
CREATE TABLE cpp_files_2_hpp_files (
    cpp_file_id INTEGER,
    hpp_file_id INTEGER,

    PRIMARY KEY (cpp_file_id, hpp_file_id),
    FOREIGN KEY (cpp_file_id) REFERENCES cpp_files (id) ON DELETE CASCADE,
    FOREIGN KEY (hpp_file_id) REFERENCES hpp_files (id) ON DELETE CASCADE
)
";

pub const HPP_FILE_2_HPP_FILE_SQL_CREATE_TABLE: &str = "
CREATE TABLE hpp_files_2_hpp_files (
    current_hpp_file_id INTEGER,
    hpp_file_id         INTEGER,

    PRIMARY KEY (current_hpp_file_id, hpp_file_id),
    FOREIGN KEY (current_hpp_file_id) REFERENCES hpp_files (id) ON DELETE CASCADE,
    FOREIGN KEY (hpp_file_id) REFERENCES hpp_files (id) ON DELETE CASCADE
)
";

pub fn create_database_tables(db_connection: &DatabaseSqliteInternal) {
    let _ = db_connection.db.execute_batch(HPP_FILE_SQL_CREATE_TABLE);
    let _ = db_connection
        .db
        .execute_batch(CPP_FILE_2_HPP_FILE_SQL_CREATE_TABLE);
    let _ = db_connection
        .db
        .execute_batch(HPP_FILE_2_HPP_FILE_SQL_CREATE_TABLE);
}
