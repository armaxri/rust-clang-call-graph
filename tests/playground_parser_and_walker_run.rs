#[cfg(test)]
mod tests {
    use std::{cell::RefCell, path::PathBuf, rc::Rc};

    use rust_clang_call_graph::{
        ast_reader::clang_ast_parser::{ClangAstParser, ClangAstParserImpl},
        call_graph::{
            ast_walker::clang_ast_walker::walk_ast_2_func_call_db,
            database::{database_content::DatabaseContent, database_sqlite::DatabaseSqlite},
        },
        process::dummy_process::DummyProcess,
    };

    fn run_ast_parser_test(test_dir_path: &str, ast2load_file_names: Vec<&str>) {
        let test_dir = PathBuf::from(test_dir_path);
        let db_path = test_dir.join("created_db.db");
        let database_sqlite = Rc::new(RefCell::new(DatabaseSqlite::create_database(
            &db_path, true,
        )));

        for ast2load_file_name in ast2load_file_names {
            let ast2load_file = test_dir.join([ast2load_file_name, ".ast2load"].join(""));
            let cpp_file = test_dir.join([ast2load_file_name, ".cpp"].join(""));
            let dummy_process = Box::new(DummyProcess::new_from_file(
                &ast2load_file.to_str().unwrap().to_string(),
            ));

            let mut parser = ClangAstParserImpl::new(dummy_process);
            let ast = parser.parse_ast();

            match ast {
                Some(ast) => {
                    walk_ast_2_func_call_db(
                        &cpp_file.to_str().unwrap(),
                        ast,
                        database_sqlite.clone(),
                    );
                }
                None => {
                    assert!(false);
                }
            }
        }

        let sqlite_content = database_sqlite.borrow().get_db_content();
        let db_json_file = test_dir.join("created_db.json");
        sqlite_content.save_to_file(&db_json_file.to_str().unwrap());

        let expected_db_content_json_file_name = test_dir.join("expected_db_content.json");
        let expected_db_content =
            DatabaseContent::load_from_file(&expected_db_content_json_file_name.to_str().unwrap());

        assert_eq!(sqlite_content, expected_db_content);
    }

    #[cfg(test)]
    mod complex_cases {
        use super::*;

        #[test]
        #[ignore]
        fn simple_gtest_test() {
            run_ast_parser_test(
                "./tests/playground/complex_cases/simple_gtest",
                vec!["file"],
            );
        }
    }

    #[cfg(test)]
    mod c_style_tests {
        use super::*;

        #[test]
        fn decl_in_header_and_2_cpps_test() {
            run_ast_parser_test(
                "./tests/playground/c_style_tests/decl_in_header_and_two_cpps",
                vec!["impl", "main"],
            );
        }

        #[test]
        fn func_call_in_func_call_test() {
            run_ast_parser_test(
                "./tests/playground/c_style_tests/func_call_in_func_call",
                vec!["file"],
            );
        }

        #[test]
        fn funcs_test() {
            run_ast_parser_test("./tests/playground/c_style_tests/funcs", vec!["file"]);
        }

        #[test]
        fn funcs_with_headers_test() {
            run_ast_parser_test(
                "./tests/playground/c_style_tests/funcs_with_headers",
                vec!["file"],
            );
        }

        #[test]
        fn multiline_func_call_in_func_call_test() {
            run_ast_parser_test(
                "./tests/playground/c_style_tests/multiline_func_call_in_func_call",
                vec!["file"],
            );
        }

        #[test]
        fn only_decl_and_impl_without_calls_test() {
            run_ast_parser_test(
                "./tests/playground/c_style_tests/only_decl_and_impl_without_calls",
                vec!["file"],
            );
        }

        #[test]
        fn printf_test() {
            run_ast_parser_test("./tests/playground/c_style_tests/printf", vec!["file"]);
        }

        #[test]
        fn simple_decl_in_header_impl_in_header_and_one_cpp_test() {
            run_ast_parser_test(
                "./tests/playground/c_style_tests/simple_decl_in_header_impl_in_header_and_one_cpp",
                vec!["main"],
            );
        }
    }

    #[cfg(test)]
    mod simple_cpp_classes {
        use super::*;

        #[test]
        fn class_call_from_lambda_test() {
            run_ast_parser_test(
                "./tests/playground/simple_cpp_classes/class_call_from_lambda",
                vec!["file"],
            );
        }

        #[test]
        fn class_in_class_test() {
            run_ast_parser_test(
                "./tests/playground/simple_cpp_classes/class_in_class",
                vec!["file"],
            );
        }

        #[test]
        fn class_raw_pointer_call_test() {
            run_ast_parser_test(
                "./tests/playground/simple_cpp_classes/class_raw_pointer_call",
                vec!["file"],
            );
        }

        #[test]
        #[ignore]
        fn class_unique_pointer_call_test() {
            run_ast_parser_test(
                "./tests/playground/simple_cpp_classes/class_unique_pointer_call",
                vec!["file"],
            );
        }

        #[test]
        fn final_method_test() {
            run_ast_parser_test(
                "./tests/playground/simple_cpp_classes/final_method",
                vec!["file"],
            );
        }

        #[test]
        fn inheritance_chain_test() {
            run_ast_parser_test(
                "./tests/playground/simple_cpp_classes/inheritance_chain",
                vec!["file"],
            );
        }

        #[test]
        fn inherited_virtual_method_test() {
            run_ast_parser_test(
                "./tests/playground/simple_cpp_classes/inherited_virtual_method",
                vec!["file"],
            );
        }

        #[test]
        fn method_test() {
            run_ast_parser_test("./tests/playground/simple_cpp_classes/method", vec!["file"]);
        }

        #[test]
        fn parent_class_in_namespace_test() {
            run_ast_parser_test(
                "./tests/playground/simple_cpp_classes/parent_class_in_namespace",
                vec!["file"],
            );
        }

        #[test]
        fn static_method_test() {
            run_ast_parser_test(
                "./tests/playground/simple_cpp_classes/static_method",
                vec!["file"],
            );
        }

        #[test]
        fn struct_method_test() {
            run_ast_parser_test(
                "./tests/playground/simple_cpp_classes/struct_method",
                vec!["file"],
            );
        }

        #[test]
        fn two_parent_classes_test() {
            run_ast_parser_test(
                "./tests/playground/simple_cpp_classes/two_parent_classes",
                vec!["file"],
            );
        }

        #[test]
        fn virtual_method_test() {
            run_ast_parser_test(
                "./tests/playground/simple_cpp_classes/virtual_method",
                vec!["file"],
            );
        }
    }

    #[cfg(test)]
    mod simple_templates {
        use super::*;

        #[test]
        fn double_template_class_test() {
            run_ast_parser_test(
                "./tests/playground/simple_templates/double_template_class",
                vec!["file"],
            );
        }

        #[test]
        fn simple_template_class_test() {
            run_ast_parser_test(
                "./tests/playground/simple_templates/simple_template_class",
                vec!["file"],
            );
        }

        #[test]
        fn simple_template_class_virtual_func_test() {
            run_ast_parser_test(
                "./tests/playground/simple_templates/simple_template_class_virtual_func",
                vec!["file"],
            );
        }

        #[test]
        #[ignore]
        fn simple_template_function_with_class_test() {
            run_ast_parser_test(
                "./tests/playground/simple_templates/simple_template_function_with_class",
                vec!["file"],
            );
        }

        #[test]
        fn simple_template_with_two_classes_test() {
            run_ast_parser_test(
                "./tests/playground/simple_templates/simple_template_with_two_classes",
                vec!["file"],
            );
        }
    }
}
