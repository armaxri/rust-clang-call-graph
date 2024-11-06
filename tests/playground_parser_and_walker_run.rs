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
            run_ast_parser_test("./tests/playground/complexCases/simpleGtest", vec!["file"]);
        }
    }

    #[cfg(test)]
    mod c_style_tests {
        use super::*;

        #[test]
        fn decl_in_header_and_2_cpps_test() {
            run_ast_parser_test(
                "./tests/playground/cStyleTests/declInHeaderAndTwoCpps",
                vec!["impl", "main"],
            );
        }

        #[test]
        fn func_call_in_func_call_test() {
            run_ast_parser_test(
                "./tests/playground/cStyleTests/funcCallInFuncCall",
                vec!["file"],
            );
        }

        #[test]
        fn funcs_test() {
            run_ast_parser_test("./tests/playground/cStyleTests/funcs", vec!["file"]);
        }

        #[test]
        fn funcs_with_headers_test() {
            run_ast_parser_test(
                "./tests/playground/cStyleTests/funcsWithHeaders",
                vec!["file"],
            );
        }

        #[test]
        fn multiline_func_call_in_func_call_test() {
            run_ast_parser_test(
                "./tests/playground/cStyleTests/multilineFuncCallInFuncCall",
                vec!["file"],
            );
        }

        #[test]
        fn only_decl_and_impl_without_calls_test() {
            run_ast_parser_test(
                "./tests/playground/cStyleTests/onlyDeclAndImplWithoutCalls",
                vec!["file"],
            );
        }

        #[test]
        fn printf_test() {
            run_ast_parser_test("./tests/playground/cStyleTests/printf", vec!["file"]);
        }

        #[test]
        fn simple_decl_in_header_impl_in_header_and_one_cpp_test() {
            run_ast_parser_test(
                "./tests/playground/cStyleTests/simpleDeclInHeaderImplInHeaderAndOneCpp",
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
                "./tests/playground/simpleCppClasses/classCallFromLambda",
                vec!["file"],
            );
        }

        #[test]
        fn class_in_class_test() {
            run_ast_parser_test(
                "./tests/playground/simpleCppClasses/classInClass",
                vec!["file"],
            );
        }

        #[test]
        fn class_raw_pointer_call_test() {
            run_ast_parser_test(
                "./tests/playground/simpleCppClasses/classRawPointerCall",
                vec!["file"],
            );
        }

        #[test]
        #[ignore]
        fn class_unique_pointer_call_test() {
            run_ast_parser_test(
                "./tests/playground/simpleCppClasses/classUniquePointerCall",
                vec!["file"],
            );
        }

        #[test]
        fn final_method_test() {
            run_ast_parser_test(
                "./tests/playground/simpleCppClasses/finalMethod",
                vec!["file"],
            );
        }

        #[test]
        fn inheritance_chain_test() {
            run_ast_parser_test(
                "./tests/playground/simpleCppClasses/inheritanceChain",
                vec!["file"],
            );
        }

        #[test]
        fn inherited_virtual_method_test() {
            run_ast_parser_test(
                "./tests/playground/simpleCppClasses/inheritedVirtualMethod",
                vec!["file"],
            );
        }

        #[test]
        fn method_test() {
            run_ast_parser_test("./tests/playground/simpleCppClasses/method", vec!["file"]);
        }

        #[test]
        fn parent_class_in_namespace_test() {
            run_ast_parser_test(
                "./tests/playground/simpleCppClasses/parentClassInNamespace",
                vec!["file"],
            );
        }

        #[test]
        fn static_method_test() {
            run_ast_parser_test(
                "./tests/playground/simpleCppClasses/staticMethod",
                vec!["file"],
            );
        }

        #[test]
        fn struct_method_test() {
            run_ast_parser_test(
                "./tests/playground/simpleCppClasses/structMethod",
                vec!["file"],
            );
        }

        #[test]
        fn two_parent_classes_test() {
            run_ast_parser_test(
                "./tests/playground/simpleCppClasses/twoParentClasses",
                vec!["file"],
            );
        }

        #[test]
        fn virtual_method_test() {
            run_ast_parser_test(
                "./tests/playground/simpleCppClasses/virtualMethod",
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
                "./tests/playground/simpleTemplates/doubleTemplateClass",
                vec!["file"],
            );
        }

        #[test]
        fn simple_template_class_test() {
            run_ast_parser_test(
                "./tests/playground/simpleTemplates/simpleTemplateClass",
                vec!["file"],
            );
        }

        #[test]
        fn simple_template_class_virtual_func_test() {
            run_ast_parser_test(
                "./tests/playground/simpleTemplates/simpleTemplateClassVirtualFunc",
                vec!["file"],
            );
        }

        #[test]
        #[ignore]
        fn simple_template_function_with_class_test() {
            run_ast_parser_test(
                "./tests/playground/simpleTemplates/simpleTemplateFunctionWithClass",
                vec!["file"],
            );
        }

        #[test]
        fn simple_template_with_two_classes_test() {
            run_ast_parser_test(
                "./tests/playground/simpleTemplates/simpleTemplateWithTwoClasses",
                vec!["file"],
            );
        }
    }
}
