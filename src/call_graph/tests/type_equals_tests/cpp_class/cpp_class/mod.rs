#[cfg(test)]
mod tests {
    use crate::{
        call_graph::{
            data_structure::MainDeclLocation,
            database::{database_content::DatabaseContent, database_sqlite::DatabaseSqlite},
        },
        file_in_directory, func_file_in_directory,
    };

    #[test]
    fn test_simple_equality_with_one_class() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("simple_cpp_class.json");
        let cpp_class = cpp_file.borrow_mut().add_class("BarClass");
        cpp_class.borrow_mut().add_class("FooClass");

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "simple_cpp_class_expected_db.json"
        ));

        assert_eq!(sqlite_content, json_content);
    }

    #[test]
    fn test_simple_get_or_add_with_one_class() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("simple_cpp_class.json");
        let cpp_class = cpp_file.borrow_mut().add_class("BarClass");
        cpp_class.borrow_mut().get_or_add_class("FooClass");
        cpp_class.borrow_mut().get_or_add_class("FooClass");

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "simple_cpp_class_expected_db.json"
        ));

        assert_eq!(sqlite_content, json_content);
    }

    #[test]
    fn test_equality_with_multiple_classes() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("simple_cpp_class.json");
        let cpp_class = cpp_file.borrow_mut().add_class("BarClass");
        cpp_class.borrow_mut().add_class("FooClassA");
        cpp_class.borrow_mut().add_class("FooClassB");
        cpp_class.borrow_mut().add_class("FooClassC");
        cpp_class.borrow_mut().add_class("FooClassD");

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "multiple_simple_cpp_class_expected_db.json"
        ));

        assert_eq!(sqlite_content, json_content);
    }

    #[test]
    fn test_no_equality_with_multiple_classes_missing_class() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("simple_cpp_class.json");
        let cpp_class = cpp_file.borrow_mut().add_class("BarClass");
        cpp_class.borrow_mut().add_class("FooClassA");
        cpp_class.borrow_mut().add_class("FooClassB");
        cpp_class.borrow_mut().add_class("FooClassC");

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "multiple_simple_cpp_class_expected_db.json"
        ));

        assert_ne!(sqlite_content, json_content);
    }

    #[test]
    fn test_no_equality_with_multiple_classes_wrong_class_name() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("simple_cpp_class.json");
        let cpp_class = cpp_file.borrow_mut().add_class("BarClass");
        cpp_class.borrow_mut().add_class("FooClassA");
        cpp_class.borrow_mut().add_class("FooClassB");
        cpp_class.borrow_mut().add_class("FooClassC");
        cpp_class.borrow_mut().add_class("FooClassX");

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "multiple_simple_cpp_class_expected_db.json"
        ));

        assert_ne!(sqlite_content, json_content);
    }

    #[test]
    fn test_no_equality_with_wrong_class_name() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("simple_cpp_class.json");
        let cpp_class = cpp_file.borrow_mut().add_class("BarClass");
        cpp_class.borrow_mut().add_class("BarClass");

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "simple_cpp_class_expected_db.json"
        ));

        assert_ne!(sqlite_content, json_content);
    }

    #[test]
    fn test_no_equality_with_added_to_wrong_cpp_file_instead() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("simple_cpp_class2.json");
        let cpp_class = cpp_file.borrow_mut().add_class("BarClass");
        cpp_class.borrow_mut().add_class("FooClass");

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "simple_cpp_class_expected_db.json"
        ));

        assert_ne!(sqlite_content, json_content);
    }

    #[test]
    fn test_removed_all_database_content() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("simple_cpp_class.json");
        let cpp_class = cpp_file.borrow_mut().add_class("BarClass");
        cpp_class.borrow_mut().add_class("FooClass");

        database_sqlite.remove_cpp_file_and_depending_content(cpp_file.borrow().get_name());

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content =
            DatabaseContent::load_from_file(&file_in_directory!("../../empty_expected_db.json"));

        assert_eq!(sqlite_content, json_content);
    }
}
