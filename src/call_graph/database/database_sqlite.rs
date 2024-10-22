use std::cell::RefCell;
use std::{path::PathBuf, rc::Rc};

use rusqlite::Connection;

use crate::call_graph::data_structure::{
    cpp_class, cpp_file, func_call, func_decl, func_impl, hpp_file, virtual_func_call,
    virtual_func_decl, virtual_func_impl,
};

use super::super::data_structure::{cpp_file::CppFile, hpp_file::HppFile};
use super::database_content::DatabaseContent;
use super::database_sqlite_internal::DatabaseSqliteInternal;

pub struct DatabaseSqlite {
    db_connection: Option<DatabaseSqliteInternal>,
}

impl DatabaseSqlite {
    pub fn create_database(file: &PathBuf, clean: bool) -> Self {
        let existed_before = file.exists();
        let mut db_removed = false;

        if existed_before && clean {
            std::fs::remove_file(&file).unwrap();
            db_removed = true;
        }

        let db_inner = Rc::new(Connection::open(file).unwrap());
        let db_connection = DatabaseSqliteInternal::new(db_inner);

        if !existed_before || db_removed {
            create_database_tables(&db_connection);
        }

        DatabaseSqlite {
            db_connection: Some(db_connection),
        }
    }

    pub fn create_in_memory_database() -> Self {
        let db_inner = Rc::new(Connection::open_in_memory().unwrap());

        let db_connection = DatabaseSqliteInternal::new(db_inner);

        create_database_tables(&db_connection);

        DatabaseSqlite {
            db_connection: Some(db_connection),
        }
    }

    pub fn get_db_connection(&self) -> Option<DatabaseSqliteInternal> {
        self.db_connection.clone()
    }

    pub fn get_cpp_files(&self) -> Vec<Rc<RefCell<CppFile>>> {
        CppFile::get_cpp_files(&self.db_connection.as_ref().unwrap())
    }
    pub fn has_cpp_file(&self, name: &str) -> bool {
        self.get_cpp_file(name).is_some()
    }
    pub fn get_cpp_file(&self, name: &str) -> Option<Rc<RefCell<CppFile>>> {
        CppFile::get_cpp_file(&self.db_connection.as_ref().unwrap(), name)
    }
    pub fn get_or_add_cpp_file(&self, name: &str) -> Rc<RefCell<CppFile>> {
        let cpp_file = self.get_cpp_file(name);
        if let Some(cpp_file) = cpp_file {
            return cpp_file;
        }

        CppFile::create_cpp_file(&self.db_connection.as_ref().unwrap(), name, None)
    }
    pub fn remove_cpp_file_and_depending_content(&self, name: &str) {
        let cpp_file = self.get_cpp_file(name);
        if let Some(cpp_file) = cpp_file {
            cpp_file
                .borrow_mut()
                .remove_cpp_file_and_depending_content();
        }
    }

    pub fn get_hpp_files(&self) -> Vec<Rc<RefCell<HppFile>>> {
        HppFile::get_hpp_files(&self.db_connection.as_ref().unwrap())
    }
    pub fn has_hpp_file(&self, name: &str) -> bool {
        self.get_hpp_file(name).is_some()
    }
    pub fn get_hpp_file(&self, name: &str) -> Option<Rc<RefCell<HppFile>>> {
        HppFile::get_hpp_file(&self.db_connection.as_ref().unwrap(), name)
    }
    pub fn get_or_add_hpp_file(&self, name: &str) -> Rc<RefCell<HppFile>> {
        let hpp_file = self.get_hpp_file(name);
        if let Some(hpp_file) = hpp_file {
            return hpp_file;
        }

        HppFile::create_hpp_file(&self.db_connection.as_ref().unwrap(), name, None)
    }
    pub fn remove_hpp_file_and_depending_content(&self, name: &str) {
        let hpp_file = self.get_hpp_file(name);
        if let Some(hpp_file) = hpp_file {
            hpp_file
                .borrow_mut()
                .remove_hpp_file_and_depending_content();
        }
    }

    // TODO implement the following functions
    // pub fn get_func_impls_or_one_decl(func: func_basics) -> func_basics[] { todo!() }
    // pub fn get_func_callers(func: func_basics) -> func_basics[] { todo!() }

    pub fn get_db_content(&self) -> DatabaseContent {
        DatabaseContent {
            cpp_files: self.get_cpp_files(),
            hpp_files: self.get_hpp_files(),
        }
    }
}

pub fn reset_database(file: &PathBuf) -> DatabaseSqliteInternal {
    if file.exists() {
        std::fs::remove_file(&file).unwrap();
    }

    let db_inner = Rc::new(Connection::open(file).unwrap());
    let db_connection = DatabaseSqliteInternal::new(db_inner);

    create_database_tables(&db_connection);

    db_connection
}

pub fn create_in_memory_database() -> DatabaseSqliteInternal {
    let db_inner = Rc::new(Connection::open_in_memory().unwrap());

    let db_connection = DatabaseSqliteInternal::new(db_inner);

    create_database_tables(&db_connection);

    db_connection
}

fn create_database_tables(db_connection: &DatabaseSqliteInternal) {
    let _ = db_connection.db.execute_batch("PRAGMA foreign_keys = ON");

    cpp_class::create_database_tables(&db_connection);
    cpp_file::create_database_tables(&db_connection);
    func_call::create_database_tables(&db_connection);
    func_decl::create_database_tables(&db_connection);
    func_impl::create_database_tables(&db_connection);
    hpp_file::create_database_tables(&db_connection);
    virtual_func_call::create_database_tables(&db_connection);
    virtual_func_decl::create_database_tables(&db_connection);
    virtual_func_impl::create_database_tables(&db_connection);
}

#[cfg(test)]
mod tests {
    use crate::file_in_directory;

    use super::*;

    #[test]
    fn test_reset_database() {
        let file = PathBuf::from(file_in_directory!("test.db"));
        if file.exists() {
            std::fs::remove_file(&file).unwrap();
        }

        let _ = std::fs::File::create(&file);
        assert!(file.exists());
        reset_database(&file);
        assert!(file.exists());
    }
}
