#[cfg(test)]
mod tests {
    use crate::{
        call_graph::{
            data_structure::{
                helper::{func_creation_args::FuncCreationArgs, range::Range},
                MainDeclLocation,
            },
            database::{database_content::DatabaseContent, database_sqlite::DatabaseSqlite},
        },
        file_in_directory, func_file_in_directory,
    };

    #[test]
    fn test_simple_match() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("simple_cpp_class_base.json");
        let cpp_class = cpp_file.borrow_mut().add_class("FooClass");
        cpp_class.borrow_mut().add_func_decl(FuncCreationArgs {
            name: "add".to_string(),
            qualified_name: "__ZN3foo3addEii".to_string(),
            qualified_type: "int (int, int)".to_string(),
            range: Range::create(11, 5, 11, 8),
        });

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "simple_cpp_class_base_expected_db.json"
        ));

        assert_eq!(sqlite_content, json_content);
    }

    #[test]
    fn test_no_equality_based_on_empty_class() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("simple_cpp_class_base.json");
        cpp_file.borrow_mut().add_class("FooClass");

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "simple_cpp_class_base_expected_db.json"
        ));

        assert_ne!(sqlite_content, json_content);
    }

    #[test]
    fn test_wrong_class_name() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("simple_cpp_class_base.json");
        let cpp_class = cpp_file.borrow_mut().add_class("NotFooClass");
        cpp_class.borrow_mut().add_func_decl(FuncCreationArgs {
            name: "add".to_string(),
            qualified_name: "__ZN3foo3addEii".to_string(),
            qualified_type: "int (int, int)".to_string(),
            range: Range::create(11, 5, 11, 8),
        });

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "simple_cpp_class_base_expected_db.json"
        ));

        assert_ne!(sqlite_content, json_content);
    }
}
