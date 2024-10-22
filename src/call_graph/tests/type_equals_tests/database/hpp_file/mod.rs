#[cfg(test)]
mod tests {
    use crate::{
        call_graph::database::{
            database_content::DatabaseContent, database_sqlite::DatabaseSqlite,
        },
        file_in_directory, func_file_in_directory,
    };

    #[test]
    fn test_simple_equality_with_one_file() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        database_sqlite.get_or_add_hpp_file("TestFile.h");

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "simple_hpp_file_expected_db.json"
        ));

        assert_eq!(sqlite_content, json_content);
    }

    #[test]
    fn test_simple_equality_with_one_double_added_file() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        database_sqlite.get_or_add_hpp_file("TestFile.h");
        database_sqlite.get_or_add_hpp_file("TestFile.h");

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "simple_hpp_file_expected_db.json"
        ));

        assert_eq!(sqlite_content, json_content);
    }

    #[test]
    fn test_equality_with_multiple_files() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        database_sqlite.get_or_add_hpp_file("TestFile.h");
        database_sqlite.get_or_add_hpp_file("FooFile.h");
        database_sqlite.get_or_add_hpp_file("BarFile.h");

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "multiple_simple_hpp_file_expected_db.json"
        ));

        assert_eq!(sqlite_content, json_content);
    }

    #[test]
    fn test_no_equality_with_multiple_files_missing_file() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        database_sqlite.get_or_add_hpp_file("TestFile.h");
        database_sqlite.get_or_add_hpp_file("FooFile.h");

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "multiple_simple_hpp_file_expected_db.json"
        ));

        assert_ne!(sqlite_content, json_content);
    }

    #[test]
    fn test_no_equality_one_additional_file() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        database_sqlite.get_or_add_hpp_file("TestFile.h");
        database_sqlite.get_or_add_hpp_file("FooFile.h");

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "simple_hpp_file_expected_db.json"
        ));

        assert_ne!(sqlite_content, json_content);
    }

    #[test]
    fn test_no_equality_with_multiple_files_wrong_file_name() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        database_sqlite.get_or_add_hpp_file("TestFile.h");
        database_sqlite.get_or_add_hpp_file("FooFile2.h");
        database_sqlite.get_or_add_hpp_file("FooFile.h");

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "multiple_simple_hpp_file_expected_db.json"
        ));

        assert_ne!(sqlite_content, json_content);
    }

    #[test]
    fn test_no_equality_with_one_file_wrong_file_name() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        database_sqlite.get_or_add_hpp_file("Foo.h");

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "simple_hpp_file_expected_db.json"
        ));

        assert_ne!(sqlite_content, json_content);
    }

    #[test]
    fn test_no_equality_added_cpp_instead() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        database_sqlite.get_or_add_cpp_file("TestFile.h");

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "simple_hpp_file_expected_db.json"
        ));

        assert_ne!(sqlite_content, json_content);
    }

    #[test]
    fn test_check_has_hpp_file_after_add() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let file_name = "TestFile.h";

        database_sqlite.get_or_add_hpp_file(file_name);

        assert!(database_sqlite.has_hpp_file(file_name));
        assert!(!database_sqlite.has_cpp_file(file_name));
        assert!(database_sqlite.get_hpp_file(file_name).is_some());

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "simple_hpp_file_expected_db.json"
        ));

        assert_eq!(sqlite_content, json_content);
    }

    #[test]
    fn test_add_and_remove_second_file() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let file_name = "TestFile.h";
        let second_file_name = "Foo.h";

        database_sqlite.get_or_add_hpp_file(file_name);
        database_sqlite.get_or_add_hpp_file(second_file_name);

        assert!(database_sqlite.has_hpp_file(file_name));
        assert!(database_sqlite.has_hpp_file(second_file_name));

        let mut sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "simple_hpp_file_expected_db.json"
        ));

        assert_ne!(sqlite_content, json_content);

        database_sqlite.remove_cpp_file_and_depending_content(second_file_name);

        sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        assert_ne!(sqlite_content, json_content);
    }
}
