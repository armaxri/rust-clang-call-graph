use std::path::PathBuf;

use rusqlite::Connection;

use crate::call_graph::data_structure::{
    cpp_class, cpp_file, func_call, func_decl, func_impl, hpp_file, virtual_func_call,
    virtual_func_decl, virtual_func_impl,
};

pub fn reset_database(file: &PathBuf) {
    if file.exists() {
        std::fs::remove_file(&file).unwrap();
    }

    let db_connection = Connection::open(file).unwrap();

    cpp_class::create_database_tables(&db_connection);
    cpp_file::create_database_tables(&db_connection);
    func_call::create_database_tables(&db_connection);
    func_decl::create_database_tables(&db_connection);
    func_impl::create_database_tables(&db_connection);
    hpp_file::create_database_tables(&db_connection);
    virtual_func_call::create_database_tables(&db_connection);
    virtual_func_decl::create_database_tables(&db_connection);
    virtual_func_impl::create_database_tables(&db_connection);

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
