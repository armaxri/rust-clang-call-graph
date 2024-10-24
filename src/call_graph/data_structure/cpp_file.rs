use std::cell::RefCell;
use std::rc::Rc;

use rusqlite::params;
use serde::Deserialize;
use serde::Serialize;

use super::super::database::database_sqlite_internal::DatabaseSqliteInternal;
use super::cpp_class::CppClass;
use super::func_structure::FuncStructure;
use super::File;
use super::MainDeclLocation;
use super::MatchingFuncs;

#[derive(Deserialize, Serialize, Debug, Clone, Eq)]
pub struct CppFile {
    id: u64,
    #[serde(skip)]
    db_connection: Option<DatabaseSqliteInternal>,

    name: String,
    last_analyzed: i64,
    classes: Vec<Rc<RefCell<CppClass>>>,
    func_decls: Vec<Rc<RefCell<FuncStructure>>>,
    func_impls: Vec<Rc<RefCell<FuncStructure>>>,
    virtual_func_impls: Vec<Rc<RefCell<FuncStructure>>>,
}

impl PartialEq for CppFile {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
            && self.name == other.name
            && self.classes == other.classes
            && self.func_decls == other.func_decls
            && self.func_impls == other.func_impls
            && self.virtual_func_impls == other.virtual_func_impls;
    }
}

impl MatchingFuncs for CppFile {
    fn get_matching_funcs(
        &self,
        _location: super::helper::location::Location,
    ) -> Vec<Rc<RefCell<FuncStructure>>> {
        todo!()
    }
}

impl MainDeclLocation for CppFile {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_db_connection(&self) -> Option<DatabaseSqliteInternal> {
        self.db_connection.clone()
    }

    fn get_id(&self) -> (Option<u64>, Option<u64>, Option<u64>) {
        (Some(self.id), None, None)
    }

    fn get_classes(&mut self) -> &mut Vec<Rc<RefCell<CppClass>>> {
        &mut self.classes
    }

    fn get_func_decls(&mut self) -> &mut Vec<Rc<RefCell<FuncStructure>>> {
        &mut self.func_decls
    }

    fn get_func_impls(&mut self) -> &mut Vec<Rc<RefCell<FuncStructure>>> {
        &mut self.func_impls
    }

    fn get_virtual_func_impls(&mut self) -> &mut Vec<Rc<RefCell<FuncStructure>>> {
        &mut self.virtual_func_impls
    }
}

impl File for CppFile {
    fn get_last_analyzed(&self) -> i64 {
        self.last_analyzed
    }

    fn set_last_analyzed(&mut self, time: i64) {
        self.last_analyzed = time;

        let mut stmt = self
            .db_connection
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
        stmt.execute(params![time, self.id]).unwrap();
    }

    fn get_includes(&self) -> Vec<Rc<dyn File>> {
        todo!()
    }
}

impl CppFile {
    pub fn new(
        id: u64,
        db_connection: Option<DatabaseSqliteInternal>,
        name: String,
        last_analyzed: i64,
    ) -> Self {
        let mut cpp_file = Self {
            id,
            db_connection,
            name,
            last_analyzed,
            classes: Vec::new(),
            func_decls: Vec::new(),
            func_impls: Vec::new(),
            virtual_func_impls: Vec::new(),
        };

        if cpp_file.db_connection.is_some() {
            cpp_file.classes = CppClass::get_cpp_classes(
                &cpp_file.db_connection.as_ref().unwrap(),
                (Some(id), None, None),
            );

            cpp_file.func_decls = FuncStructure::get_func_decls(
                cpp_file.db_connection.as_ref().unwrap(),
                (Some(cpp_file.id), None, None),
            );
            cpp_file.func_impls = FuncStructure::get_func_impls(
                cpp_file.db_connection.as_ref().unwrap(),
                (Some(cpp_file.id), None, None),
            );
            cpp_file.virtual_func_impls = FuncStructure::get_virtual_func_impls(
                cpp_file.db_connection.as_ref().unwrap(),
                (Some(cpp_file.id), None, None),
            );
        }

        cpp_file
    }

    pub fn create_cpp_file(
        db_connection: &DatabaseSqliteInternal,
        name: &str,
        last_analyzed: Option<i64>,
    ) -> Rc<RefCell<CppFile>> {
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
            INSERT INTO cpp_files (file_name, last_analyzed)
            VALUES (?, ?)",
            )
            .unwrap();
        let result = stmt.insert(params![name, time,]);

        Rc::new(RefCell::new(CppFile::new(
            result.unwrap() as u64,
            Some(db_connection.clone()),
            name.to_string(),
            time,
        )))
    }

    pub fn get_cpp_file(
        db_connection: &DatabaseSqliteInternal,
        name: &str,
    ) -> Option<Rc<RefCell<CppFile>>> {
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
            let last_analyzed: i64 = row.get(1).unwrap();

            Some(Rc::new(RefCell::new(CppFile::new(
                id,
                Some(db_connection.clone()),
                name.to_string(),
                last_analyzed,
            ))))
        } else {
            None
        }
    }

    pub fn get_cpp_files(db_connection: &DatabaseSqliteInternal) -> Vec<Rc<RefCell<CppFile>>> {
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
            let last_analyzed: i64 = row.get(2).unwrap();

            cpp_files.push(Rc::new(RefCell::new(CppFile::new(
                id,
                Some(db_connection.clone()),
                name,
                last_analyzed,
            ))));
        }

        cpp_files
    }

    pub fn remove_cpp_file_and_depending_content(&self) {
        let _ = self.db_connection.as_ref().unwrap().db.execute(
            "
            DELETE FROM cpp_files
            WHERE id = ?",
            params![self.id],
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
