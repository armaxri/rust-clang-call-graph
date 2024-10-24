#[cfg(test)]
mod tests {
    use crate::{
        call_graph::{
            data_structure::{
                helper::virtual_func_creation_args::VirtualFuncCreationArgs, MainDeclPosition,
            },
            database::{database_content::DatabaseContent, database_sqlite::DatabaseSqlite},
        },
        file_in_directory, func_file_in_directory,
        location::range::Range,
    };

    #[test]
    fn test_simple_equality_with_one_function() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("simple_virtual_func_impl.json");
        cpp_file
            .borrow_mut()
            .add_virtual_func_impl(VirtualFuncCreationArgs {
                name: "add".to_string(),
                qualified_name: "__ZN3foo3addEii".to_string(),
                base_qualified_name: "__ZN3foo3addEii".to_string(),
                qualified_type: "int (int, int)".to_string(),
                range: Range::create(11, 5, 11, 8),
            });

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "simple_virtual_func_impl_expected_db.json"
        ));

        assert_eq!(sqlite_content, json_content);
    }

    #[test]
    fn test_simple_get_or_add_with_one_function() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("simple_virtual_func_impl.json");
        cpp_file
            .borrow_mut()
            .get_or_add_virtual_func_impl(VirtualFuncCreationArgs {
                name: "add".to_string(),
                qualified_name: "__ZN3foo3addEii".to_string(),
                base_qualified_name: "__ZN3foo3addEii".to_string(),
                qualified_type: "int (int, int)".to_string(),
                range: Range::create(11, 5, 11, 8),
            });
        cpp_file
            .borrow_mut()
            .get_or_add_virtual_func_impl(VirtualFuncCreationArgs {
                name: "add".to_string(),
                qualified_name: "__ZN3foo3addEii".to_string(),
                base_qualified_name: "__ZN3foo3addEii".to_string(),
                qualified_type: "int (int, int)".to_string(),
                range: Range::create(11, 5, 11, 8),
            });

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "simple_virtual_func_impl_expected_db.json"
        ));

        assert_eq!(sqlite_content, json_content);
    }

    #[test]
    fn test_equality_with_multiple_functions() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file =
            database_sqlite.get_or_add_cpp_file("multiple_simple_virtual_func_impl.json");
        cpp_file
            .borrow_mut()
            .add_virtual_func_impl(VirtualFuncCreationArgs {
                name: "add".to_string(),
                qualified_name: "__ZN3foo3addEii".to_string(),
                base_qualified_name: "__ZN3foo3addEii".to_string(),
                qualified_type: "int (int, int)".to_string(),
                range: Range::create(11, 5, 11, 8),
            });
        cpp_file
            .borrow_mut()
            .add_virtual_func_impl(VirtualFuncCreationArgs {
                name: "sub".to_string(),
                qualified_name: "__ZN3foo3subEii".to_string(),
                base_qualified_name: "__ZN3foo3subEii".to_string(),
                qualified_type: "int (int, int)".to_string(),
                range: Range::create(12, 5, 12, 8),
            });
        cpp_file
            .borrow_mut()
            .add_virtual_func_impl(VirtualFuncCreationArgs {
                name: "multiply".to_string(),
                qualified_name: "__ZN3foo8multiplyEii".to_string(),
                base_qualified_name: "__ZN3foo8multiplyEii".to_string(),
                qualified_type: "int (int, int)".to_string(),
                range: Range::create(13, 5, 13, 13),
            });
        cpp_file
            .borrow_mut()
            .add_virtual_func_impl(VirtualFuncCreationArgs {
                name: "divide".to_string(),
                qualified_name: "__ZN3foo6divideEii".to_string(),
                base_qualified_name: "__ZN3foo6divideEii".to_string(),
                qualified_type: "int (int, int)".to_string(),
                range: Range::create(14, 5, 14, 11),
            });

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "multiple_simple_virtual_func_impl_expected_db.json"
        ));

        assert_eq!(sqlite_content, json_content);
    }

    #[test]
    fn test_no_equality_with_multiple_functions_missing_implementation() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file =
            database_sqlite.get_or_add_cpp_file("multiple_simple_virtual_func_impl.json");
        cpp_file
            .borrow_mut()
            .add_virtual_func_impl(VirtualFuncCreationArgs {
                name: "add".to_string(),
                qualified_name: "__ZN3foo3addEii".to_string(),
                base_qualified_name: "__ZN3foo3addEii".to_string(),
                qualified_type: "int (int, int)".to_string(),
                range: Range::create(11, 5, 11, 8),
            });
        cpp_file
            .borrow_mut()
            .add_virtual_func_impl(VirtualFuncCreationArgs {
                name: "sub".to_string(),
                qualified_name: "__ZN3foo3subEii".to_string(),
                base_qualified_name: "__ZN3foo3subEii".to_string(),
                qualified_type: "int (int, int)".to_string(),
                range: Range::create(12, 5, 12, 8),
            });
        cpp_file
            .borrow_mut()
            .add_virtual_func_impl(VirtualFuncCreationArgs {
                name: "multiply".to_string(),
                qualified_name: "__ZN3foo8multiplyEii".to_string(),
                base_qualified_name: "__ZN3foo8multiplyEii".to_string(),
                qualified_type: "int (int, int)".to_string(),
                range: Range::create(13, 5, 13, 13),
            });

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "multiple_simple_virtual_func_impl_expected_db.json"
        ));

        assert_ne!(sqlite_content, json_content);
    }

    #[test]
    fn test_no_equality_with_wrong_function_name() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("simple_virtual_func_impl.json");
        cpp_file
            .borrow_mut()
            .add_virtual_func_impl(VirtualFuncCreationArgs {
                name: "multiply".to_string(),
                qualified_name: "__ZN3foo3addEii".to_string(),
                base_qualified_name: "__ZN3foo3addEii".to_string(),
                qualified_type: "int (int, int)".to_string(),
                range: Range::create(11, 5, 11, 8),
            });

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "simple_virtual_func_impl_expected_db.json"
        ));

        assert_ne!(sqlite_content, json_content);
    }

    #[test]
    fn test_no_equality_with_wrong_base_function_name() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("simple_virtual_func_impl.json");
        cpp_file
            .borrow_mut()
            .add_virtual_func_impl(VirtualFuncCreationArgs {
                name: "add".to_string(),
                qualified_name: "__ZN3foo3addEii".to_string(),
                base_qualified_name: "multi".to_string(),
                qualified_type: "int (int, int)".to_string(),
                range: Range::create(11, 5, 11, 8),
            });

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "simple_virtual_func_impl_expected_db.json"
        ));

        assert_ne!(sqlite_content, json_content);
    }

    #[test]
    fn test_no_equality_with_wrong_position() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("simple_virtual_func_impl.json");
        cpp_file
            .borrow_mut()
            .add_virtual_func_impl(VirtualFuncCreationArgs {
                name: "add".to_string(),
                qualified_name: "__ZN3foo3addEii".to_string(),
                base_qualified_name: "__ZN3foo3addEii".to_string(),
                qualified_type: "int (int, int)".to_string(),
                range: Range::create(11, 6, 11, 8),
            });

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "simple_virtual_func_impl_expected_db.json"
        ));

        assert_ne!(sqlite_content, json_content);
    }

    #[test]
    fn test_removed_all_database_content() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("simple_virtual_func_impl.json");
        cpp_file
            .borrow_mut()
            .add_virtual_func_impl(VirtualFuncCreationArgs {
                name: "add".to_string(),
                qualified_name: "__ZN3foo3addEii".to_string(),
                base_qualified_name: "__ZN3foo3addEii".to_string(),
                qualified_type: "int (int, int)".to_string(),
                range: Range::create(11, 5, 11, 8),
            });

        database_sqlite.remove_cpp_file_and_depending_content(cpp_file.borrow().get_name());

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content =
            DatabaseContent::load_from_file(&file_in_directory!("../../empty_expected_db.json"));

        assert_eq!(sqlite_content, json_content);
    }
}
