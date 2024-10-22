#[cfg(test)]
mod tests {
    use crate::{
        call_graph::database::{
            database_content::DatabaseContent, database_sqlite::DatabaseSqlite,
        },
        file_in_directory, func_file_in_directory,
    };

    #[test]
    fn test_equality_with_simple_reference() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("base.cpp");
        let hpp_file = database_sqlite.get_or_add_hpp_file("base.hpp");

        hpp_file
            .borrow_mut()
            .add_referenced_from_source_file(&cpp_file);

        assert_eq!(
            hpp_file.borrow().get_referenced_from_source_files().len(),
            1
        );
        assert_eq!(
            hpp_file.borrow().get_referenced_from_header_files().len(),
            0
        );

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "simple_referenced_from_cpp_file_expected_db.json"
        ));

        assert_eq!(sqlite_content, json_content);
    }

    #[test]
    fn test_equality_with_two_references_and_two_header() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("main.cpp");
        let first_hpp_file = database_sqlite.get_or_add_hpp_file("firstHeader.hpp");
        let second_hpp_file = database_sqlite.get_or_add_hpp_file("secondHeader.hpp");

        first_hpp_file
            .borrow_mut()
            .add_referenced_from_source_file(&cpp_file);
        first_hpp_file
            .borrow_mut()
            .add_referenced_from_header_file(&second_hpp_file);
        second_hpp_file
            .borrow_mut()
            .add_referenced_from_source_file(&cpp_file);

        assert_eq!(
            first_hpp_file
                .borrow()
                .get_referenced_from_source_files()
                .len(),
            1
        );
        assert_eq!(
            first_hpp_file
                .borrow()
                .get_referenced_from_header_files()
                .len(),
            1
        );

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "simple_referenced_from_hpp_file_expected_db.json"
        ));

        assert_eq!(sqlite_content, json_content);
    }

    #[test]
    fn test_no_equality_with_simple_parent_class_missing_connection() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        database_sqlite.get_or_add_cpp_file("base.cpp");
        database_sqlite.get_or_add_hpp_file("base.hpp");

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "simple_referenced_from_cpp_file_expected_db.json"
        ));

        assert_ne!(sqlite_content, json_content);
    }
}
