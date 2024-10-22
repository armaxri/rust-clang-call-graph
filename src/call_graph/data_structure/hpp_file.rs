use std::cell::RefCell;
use std::rc::Rc;

use rusqlite::params;
use serde::Deserialize;
use serde::Serialize;

use super::super::database::database_sqlite_internal::DatabaseSqliteInternal;
use super::super::function_search::function_occurrence::FunctionOccurrence;
use super::cpp_class::CppClass;
use super::cpp_file::CppFile;
use super::func_decl::FuncDecl;
use super::func_impl::FuncImpl;
use super::virtual_func_impl::VirtualFuncImpl;
use super::File;
use super::MainDeclLocation;
use super::MatchingFuncs;

#[derive(Deserialize, Serialize, Debug, Clone, Eq)]
pub struct HppFile {
    id: u64,
    #[serde(skip)]
    db_connection: Option<DatabaseSqliteInternal>,

    name: String,
    last_analyzed: i64,
    classes: Vec<Rc<RefCell<CppClass>>>,
    func_decls: Vec<Rc<RefCell<FuncDecl>>>,
    func_impls: Vec<Rc<RefCell<FuncImpl>>>,
    virtual_func_impls: Vec<Rc<RefCell<VirtualFuncImpl>>>,
    referenced_from_header_files: Vec<String>,
    referenced_from_source_files: Vec<String>,
}

impl PartialEq for HppFile {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
            && self.name == other.name
            && self.classes == other.classes
            && self.func_decls == other.func_decls
            && self.func_impls == other.func_impls
            && self.virtual_func_impls == other.virtual_func_impls
            && self.referenced_from_header_files == other.referenced_from_header_files
            && self.referenced_from_source_files == other.referenced_from_source_files;
    }
}

impl MatchingFuncs for HppFile {
    fn get_matching_funcs(
        &self,
        _location: super::helper::location::Location,
    ) -> Vec<FunctionOccurrence> {
        todo!()
    }
}

impl MainDeclLocation for HppFile {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_db_connection(&self) -> Option<DatabaseSqliteInternal> {
        self.db_connection.clone()
    }

    fn get_id(&self) -> (Option<u64>, Option<u64>, Option<u64>) {
        (None, Some(self.id), None)
    }

    fn get_classes(&mut self) -> &mut Vec<Rc<RefCell<CppClass>>> {
        &mut self.classes
    }

    fn get_func_decls(&mut self) -> &mut Vec<Rc<RefCell<FuncDecl>>> {
        &mut self.func_decls
    }

    fn get_func_impls(&mut self) -> &mut Vec<Rc<RefCell<FuncImpl>>> {
        &mut self.func_impls
    }

    fn get_virtual_func_impls(&mut self) -> &mut Vec<Rc<RefCell<VirtualFuncImpl>>> {
        &mut self.virtual_func_impls
    }
}

impl File for HppFile {
    fn get_includes(&self) -> Vec<Rc<dyn File>> {
        todo!()
    }

    fn get_last_analyzed(&self) -> i64 {
        self.last_analyzed
    }

    fn set_last_analyzed(&mut self, last_analyzed: i64) {
        self.last_analyzed = last_analyzed;
        let mut stmt = self
            .db_connection
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
        stmt.execute(params![last_analyzed, self.id]).unwrap();
    }
}

impl HppFile {
    pub fn new(
        id: u64,
        db_connection: Option<DatabaseSqliteInternal>,
        name: String,
        last_analyzed: i64,
    ) -> Self {
        let mut hpp_file = Self {
            id,
            db_connection,
            name,
            last_analyzed,
            classes: Vec::new(),
            func_decls: Vec::new(),
            func_impls: Vec::new(),
            virtual_func_impls: Vec::new(),
            referenced_from_header_files: Vec::new(),
            referenced_from_source_files: Vec::new(),
        };

        if hpp_file.db_connection.is_some() {
            hpp_file.read_referenced_from_header_files();
            hpp_file.read_referenced_from_source_files();

            hpp_file.classes = CppClass::get_cpp_classes(
                &hpp_file.db_connection.as_ref().unwrap(),
                (None, Some(id), None),
            );

            hpp_file.func_decls = FuncDecl::get_func_decls(
                hpp_file.db_connection.as_ref().unwrap(),
                (None, Some(hpp_file.id), None),
            );
            hpp_file.func_impls = FuncImpl::get_func_impls(
                hpp_file.db_connection.as_ref().unwrap(),
                (None, Some(hpp_file.id), None),
            );
            hpp_file.virtual_func_impls = VirtualFuncImpl::get_virtual_func_impls(
                hpp_file.db_connection.as_ref().unwrap(),
                (None, Some(hpp_file.id), None),
            );
        }

        hpp_file
    }

    pub fn create_hpp_file(
        db_connection: &DatabaseSqliteInternal,
        name: &str,
        last_analyzed: Option<i64>,
    ) -> Rc<RefCell<HppFile>> {
        let time = match last_analyzed {
            Some(time) => time,
            None => std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
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

        Rc::new(RefCell::new(HppFile::new(
            result.unwrap() as u64,
            Some(db_connection.clone()),
            name.to_string(),
            time,
        )))
    }

    pub fn get_hpp_file(
        db_connection: &DatabaseSqliteInternal,
        name: &str,
    ) -> Option<Rc<RefCell<HppFile>>> {
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
            let last_analyzed: i64 = row.get(1).unwrap();

            Some(Rc::new(RefCell::new(HppFile::new(
                id,
                Some(db_connection.clone()),
                name.to_string(),
                last_analyzed,
            ))))
        } else {
            None
        }
    }

    pub fn get_hpp_files(db_connection: &DatabaseSqliteInternal) -> Vec<Rc<RefCell<HppFile>>> {
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
            let last_analyzed: i64 = row.get(2).unwrap();

            hpp_files.push(Rc::new(RefCell::new(HppFile::new(
                id,
                Some(db_connection.clone()),
                name,
                last_analyzed,
            ))));
        }

        hpp_files
    }

    pub fn remove_hpp_file_and_depending_content(&self) {
        let _ = self.db_connection.as_ref().unwrap().db.execute(
            "
            DELETE FROM hpp_files
            WHERE id = ?",
            params![self.id],
        );
    }

    pub fn get_referenced_from_header_files(&self) -> Vec<String> {
        self.referenced_from_header_files.clone()
    }

    pub fn add_referenced_from_header_file(&mut self, file: &Rc<RefCell<HppFile>>) {
        if self
            .referenced_from_header_files
            .contains(&file.borrow().name)
        {
            return;
        }

        let mut stmt = self
            .db_connection
            .as_ref()
            .unwrap()
            .db
            .prepare(
                "
            INSERT INTO hpp_files_2_hpp_files (current_hpp_file_id, hpp_file_id)
            VALUES (?, ?)",
            )
            .unwrap();
        stmt.insert(params![self.id, file.borrow().id]).unwrap();

        self.referenced_from_header_files
            .push(file.borrow().name.clone());
    }

    fn read_referenced_from_header_files(&mut self) {
        let mut stmt = self
            .db_connection
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
        let mut rows = stmt.query(params![self.id]).unwrap();

        while let Ok(Some(row)) = rows.next() {
            let file_name: String = row.get(0).unwrap();
            self.referenced_from_header_files.push(file_name);
        }
    }

    pub fn get_referenced_from_source_files(&self) -> Vec<String> {
        self.referenced_from_source_files.clone()
    }

    pub fn add_referenced_from_source_file(&mut self, file: &Rc<RefCell<CppFile>>) {
        if self
            .referenced_from_source_files
            .contains(&file.borrow().get_name().to_string())
        {
            return;
        }

        let mut stmt = self
            .db_connection
            .as_ref()
            .unwrap()
            .db
            .prepare(
                "
            INSERT INTO cpp_files_2_hpp_files (cpp_file_id, hpp_file_id)
            VALUES (?, ?)",
            )
            .unwrap();
        stmt.insert(params![file.borrow().get_id().0.unwrap(), self.id])
            .unwrap();

        self.referenced_from_source_files
            .push(file.borrow().get_name().to_string());
    }

    fn read_referenced_from_source_files(&mut self) {
        let mut stmt = self
            .db_connection
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
        let mut rows = stmt.query(params![self.id]).unwrap();

        while let Ok(Some(row)) = rows.next() {
            let file_name: String = row.get(0).unwrap();
            self.referenced_from_source_files.push(file_name);
        }
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
