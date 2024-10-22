#[cfg(test)]
mod tests {
    use crate::{
        call_graph::{
            data_structure::{
                helper::{func_creation_args::FuncCreationArgs, range::Range},
                FuncBasics, FuncImplBasics, MainDeclLocation,
            },
            database::{database_content::DatabaseContent, database_sqlite::DatabaseSqlite},
        },
        file_in_directory, func_file_in_directory,
    };

    #[test]
    fn test_simple_equality_with_one_call() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("simple_func_call.json");
        let func_decl = cpp_file.borrow_mut().add_func_decl(FuncCreationArgs {
            name: "add".to_string(),
            qualified_name: "__ZN3foo3addEii".to_string(),
            qualified_type: "int (int, int)".to_string(),
            range: Range::create(11, 5, 11, 8),
        });
        let func_impl = cpp_file.borrow_mut().add_func_impl(FuncCreationArgs {
            name: "main".to_string(),
            qualified_name: "_main".to_string(),
            qualified_type: "int (int, char **)".to_string(),
            range: Range::create(5, 4, 5, 9),
        });
        func_impl.borrow_mut().add_func_call(
            &func_decl
                .borrow()
                .convert_func2func_creation_args4call(&Range::create(20, 6, 20, 10)),
        );

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "simple_func_call_expected_db.json"
        ));

        assert_eq!(sqlite_content, json_content);
    }

    #[test]
    fn test_simple_get_or_add_with_one_call() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("simple_func_call.json");
        let func_decl = cpp_file.borrow_mut().add_func_decl(FuncCreationArgs {
            name: "add".to_string(),
            qualified_name: "__ZN3foo3addEii".to_string(),
            qualified_type: "int (int, int)".to_string(),
            range: Range::create(11, 5, 11, 8),
        });
        let func_impl = cpp_file.borrow_mut().add_func_impl(FuncCreationArgs {
            name: "main".to_string(),
            qualified_name: "_main".to_string(),
            qualified_type: "int (int, char **)".to_string(),
            range: Range::create(5, 4, 5, 9),
        });
        func_impl.borrow_mut().get_or_add_func_call(
            &func_decl
                .borrow()
                .convert_func2func_creation_args4call(&Range::create(20, 6, 20, 10)),
        );
        func_impl.borrow_mut().get_or_add_func_call(
            &func_decl
                .borrow()
                .convert_func2func_creation_args4call(&Range::create(20, 6, 20, 10)),
        );

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "simple_func_call_expected_db.json"
        ));

        assert_eq!(sqlite_content, json_content);
    }

    #[test]
    fn test_equality_with_multiple_calls() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("multiple_simple_func_call.json");
        let func_decl_add = cpp_file.borrow_mut().add_func_decl(FuncCreationArgs {
            name: "add".to_string(),
            qualified_name: "__ZN3foo3addEii".to_string(),
            qualified_type: "int (int, int)".to_string(),
            range: Range::create(11, 5, 11, 8),
        });
        let func_decl_sub = cpp_file.borrow_mut().add_func_decl(FuncCreationArgs {
            name: "sub".to_string(),
            qualified_name: "__ZN3foo3subEii".to_string(),
            qualified_type: "int (int, int)".to_string(),
            range: Range::create(12, 5, 12, 8),
        });
        let func_decl_multiply = cpp_file.borrow_mut().add_func_decl(FuncCreationArgs {
            name: "multiply".to_string(),
            qualified_name: "__ZN3foo3multiplyEii".to_string(),
            qualified_type: "int (int, int)".to_string(),
            range: Range::create(13, 5, 13, 13),
        });
        let func_decl_divide = cpp_file.borrow_mut().add_func_decl(FuncCreationArgs {
            name: "divide".to_string(),
            qualified_name: "__ZN3foo3divideEii".to_string(),
            qualified_type: "int (int, int)".to_string(),
            range: Range::create(14, 5, 14, 11),
        });
        let func_impl = cpp_file.borrow_mut().add_func_impl(FuncCreationArgs {
            name: "main".to_string(),
            qualified_name: "_main".to_string(),
            qualified_type: "int (int, char **)".to_string(),
            range: Range::create(5, 4, 5, 9),
        });
        func_impl.borrow_mut().add_func_call(
            &func_decl_add
                .borrow()
                .convert_func2func_creation_args4call(&func_decl_add.borrow().get_range()),
        );
        func_impl.borrow_mut().add_func_call(
            &func_decl_sub
                .borrow()
                .convert_func2func_creation_args4call(&func_decl_sub.borrow().get_range()),
        );
        func_impl.borrow_mut().add_func_call(
            &func_decl_multiply
                .borrow()
                .convert_func2func_creation_args4call(&func_decl_multiply.borrow().get_range()),
        );
        func_impl.borrow_mut().add_func_call(
            &func_decl_divide
                .borrow()
                .convert_func2func_creation_args4call(&func_decl_divide.borrow().get_range()),
        );

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "multiple_simple_func_call_expected_db.json"
        ));

        assert_eq!(sqlite_content, json_content);
    }

    #[test]
    fn test_no_equality_with_multiple_calls_missing_call() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("multiple_simple_func_call.json");
        let func_decl_add = cpp_file.borrow_mut().add_func_decl(FuncCreationArgs {
            name: "add".to_string(),
            qualified_name: "__ZN3foo3addEii".to_string(),
            qualified_type: "int (int, int)".to_string(),
            range: Range::create(11, 5, 11, 8),
        });
        let func_decl_sub = cpp_file.borrow_mut().add_func_decl(FuncCreationArgs {
            name: "sub".to_string(),
            qualified_name: "__ZN3foo3subEii".to_string(),
            qualified_type: "int (int, int)".to_string(),
            range: Range::create(12, 5, 12, 8),
        });
        let func_decl_multiply = cpp_file.borrow_mut().add_func_decl(FuncCreationArgs {
            name: "multiply".to_string(),
            qualified_name: "__ZN3foo3multiplyEii".to_string(),
            qualified_type: "int (int, int)".to_string(),
            range: Range::create(13, 5, 13, 13),
        });
        cpp_file.borrow_mut().add_func_decl(FuncCreationArgs {
            name: "divide".to_string(),
            qualified_name: "__ZN3foo3divideEii".to_string(),
            qualified_type: "int (int, int)".to_string(),
            range: Range::create(14, 5, 14, 11),
        });
        let func_impl = cpp_file.borrow_mut().add_func_impl(FuncCreationArgs {
            name: "main".to_string(),
            qualified_name: "_main".to_string(),
            qualified_type: "int (int, char **)".to_string(),
            range: Range::create(5, 4, 5, 9),
        });
        func_impl.borrow_mut().add_func_call(
            &func_decl_add
                .borrow()
                .convert_func2func_creation_args4call(&func_decl_add.borrow().get_range()),
        );
        func_impl.borrow_mut().add_func_call(
            &func_decl_sub
                .borrow()
                .convert_func2func_creation_args4call(&func_decl_sub.borrow().get_range()),
        );
        func_impl.borrow_mut().add_func_call(
            &func_decl_multiply
                .borrow()
                .convert_func2func_creation_args4call(&func_decl_multiply.borrow().get_range()),
        );

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "multiple_simple_func_call_expected_db.json"
        ));

        assert_ne!(sqlite_content, json_content);
    }

    #[test]
    fn test_no_equality_with_wrong_call_name() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file: std::rc::Rc<
            std::cell::RefCell<crate::call_graph::data_structure::cpp_file::CppFile>,
        > = database_sqlite.get_or_add_cpp_file("simple_func_call.json");
        let func_decl = cpp_file.borrow_mut().add_func_decl(FuncCreationArgs {
            name: "multiply".to_string(),
            qualified_name: "__ZN3foo3addEii".to_string(),
            qualified_type: "int (int, int)".to_string(),
            range: Range::create(11, 5, 11, 8),
        });
        let func_impl = cpp_file.borrow_mut().add_func_impl(FuncCreationArgs {
            name: "main".to_string(),
            qualified_name: "_main".to_string(),
            qualified_type: "int (int, char **)".to_string(),
            range: Range::create(5, 4, 5, 9),
        });
        func_impl.borrow_mut().add_func_call(
            &func_decl
                .borrow()
                .convert_func2func_creation_args4call(&Range::create(20, 6, 20, 10)),
        );

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "simple_func_call_expected_db.json"
        ));

        assert_ne!(sqlite_content, json_content);
    }

    #[test]
    fn test_no_equality_with_wrong_location() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("simple_func_call.json");
        let func_decl = cpp_file.borrow_mut().add_func_decl(FuncCreationArgs {
            name: "add".to_string(),
            qualified_name: "__ZN3foo3addEii".to_string(),
            qualified_type: "int (int, int)".to_string(),
            range: Range::create(11, 5, 11, 8),
        });
        let func_impl = cpp_file.borrow_mut().add_func_impl(FuncCreationArgs {
            name: "main".to_string(),
            qualified_name: "_main".to_string(),
            qualified_type: "int (int, char **)".to_string(),
            range: Range::create(5, 4, 5, 9),
        });
        func_impl.borrow_mut().add_func_call(
            &func_decl
                .borrow()
                .convert_func2func_creation_args4call(&Range::create(20, 6, 30, 10)),
        );

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "simple_func_call_expected_db.json"
        ));

        assert_ne!(sqlite_content, json_content);
    }

    #[test]
    fn test_removed_all_database_content() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("simple_func_call.json");
        let func_decl = cpp_file.borrow_mut().add_func_decl(FuncCreationArgs {
            name: "add".to_string(),
            qualified_name: "__ZN3foo3addEii".to_string(),
            qualified_type: "int (int, int)".to_string(),
            range: Range::create(11, 5, 11, 8),
        });
        let func_impl = cpp_file.borrow_mut().add_func_impl(FuncCreationArgs {
            name: "main".to_string(),
            qualified_name: "_main".to_string(),
            qualified_type: "int (int, char **)".to_string(),
            range: Range::create(5, 4, 5, 9),
        });
        func_impl.borrow_mut().add_func_call(
            &func_decl
                .borrow()
                .convert_func2func_creation_args4call(&Range::create(20, 6, 20, 10)),
        );

        database_sqlite.remove_cpp_file_and_depending_content(cpp_file.borrow().get_name());

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content =
            DatabaseContent::load_from_file(&file_in_directory!("../../empty_expected_db.json"));

        assert_eq!(sqlite_content, json_content);
    }
}
