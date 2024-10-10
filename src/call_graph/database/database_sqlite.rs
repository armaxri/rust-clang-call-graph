use std::path::PathBuf;

use rusqlite::Connection;

use crate::call_graph::data_structure::{
    cpp_class::{CPP_CLASS_2_CLASS_SQL_CREATE_TABLE, CPP_CLASS_SQL_CREATE_TABLE},
    cpp_file::CPP_FILE_SQL_CREATE_TABLE,
    func_call::FUNC_CALL_SQL_CREATE_TABLE,
    func_decl::FUNC_DECL_SQL_CREATE_TABLE,
    func_impl::FUNC_IMPL_SQL_CREATE_TABLE,
    hpp_file::{
        CPP_FILE_2_HPP_FILE_SQL_CREATE_TABLE, HPP_FILE_2_HPP_FILE_SQL_CREATE_TABLE,
        HPP_FILE_SQL_CREATE_TABLE,
    },
    virtual_func_call::VIRTUAL_FUNC_CALL_SQL_CREATE_TABLE,
    virtual_func_decl::VIRTUAL_FUNC_DECL_SQL_CREATE_TABLE,
    virtual_func_impl::VIRTUAL_FUNC_IMPL_SQL_CREATE_TABLE,
};

pub fn reset_database(file: &PathBuf) {
    if file.exists() {
        std::fs::remove_file(&file).unwrap();
    }

    let db_connection = Connection::open(file).unwrap();

    let _ = db_connection.execute(CPP_CLASS_SQL_CREATE_TABLE, ());
    let _ = db_connection.execute(CPP_CLASS_2_CLASS_SQL_CREATE_TABLE, ());
    let _ = db_connection.execute(CPP_FILE_SQL_CREATE_TABLE, ());
    let _ = db_connection.execute(FUNC_CALL_SQL_CREATE_TABLE, ());
    let _ = db_connection.execute(FUNC_DECL_SQL_CREATE_TABLE, ());
    let _ = db_connection.execute(FUNC_IMPL_SQL_CREATE_TABLE, ());
    let _ = db_connection.execute(HPP_FILE_SQL_CREATE_TABLE, ());
    let _ = db_connection.execute(CPP_FILE_2_HPP_FILE_SQL_CREATE_TABLE, ());
    let _ = db_connection.execute(HPP_FILE_2_HPP_FILE_SQL_CREATE_TABLE, ());
    let _ = db_connection.execute(VIRTUAL_FUNC_CALL_SQL_CREATE_TABLE, ());
    let _ = db_connection.execute(VIRTUAL_FUNC_DECL_SQL_CREATE_TABLE, ());
    let _ = db_connection.execute(VIRTUAL_FUNC_IMPL_SQL_CREATE_TABLE, ());

    db_connection.close().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reset_database() {
        let file = PathBuf::from("src/call_graph/database/test.db");
        if file.exists() {
            std::fs::remove_file(&file).unwrap();
        }

        let _ = std::fs::File::create(&file);
        assert!(file.exists());
        reset_database(&file);
        assert!(file.exists());
    }
}
