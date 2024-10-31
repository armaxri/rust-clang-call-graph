use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::{
    ast_reader::clang_ast_element::ClangAstElement,
    call_graph::{
        data_structure::{
            file_structure::FileStructure, helper::func_creation_args::FuncCreationArgs,
            MainDeclPosition,
        },
        database::database_sqlite::DatabaseSqlite,
    },
};

struct ClangAstWalkerInternal {
    pub db: Rc<RefCell<DatabaseSqlite>>,
    pub file_path: String,
    pub current_file: Rc<RefCell<FileStructure>>,
}

pub fn walk_ast_2_func_call_db(
    file_path: &str,
    parsed_ast: VecDeque<ClangAstElement>,
    db: Rc<RefCell<DatabaseSqlite>>,
) {
    // Make sure that the file is in the database, so that we can reference it.
    let main_file = db.borrow().get_or_add_cpp_file(&file_path);
    let mut current_file_name_str = main_file.borrow().get_name().to_string();

    let mut walker = ClangAstWalkerInternal {
        db: db,
        file_path: file_path.to_string(),
        current_file: main_file.clone(),
    };

    for ast_element in parsed_ast {
        if *ast_element.file == "" {
            continue;
        }

        if *ast_element.file != current_file_name_str {
            if *ast_element.file == walker.file_path {
                walker.current_file = main_file.clone();
            } else {
                walker.current_file = walker.db.borrow().get_or_add_hpp_file(&ast_element.file);
                walker
                    .current_file
                    .borrow_mut()
                    .add_referenced_from_source_file(&main_file);
            }
            current_file_name_str = walker.current_file.borrow().get_name().to_string();
        }

        handle_ast_element(&ast_element, &mut walker);
    }
}

fn handle_ast_element(ast_element: &ClangAstElement, walker: &mut ClangAstWalkerInternal) {
    match ast_element.element_type.as_str() {
        "FunctionDecl" => {
            handle_function_decl(ast_element, walker);
        }
        _ => {}
    }
}

fn handle_function_decl(ast_element: &ClangAstElement, walker: &mut ClangAstWalkerInternal) {
    let compound_stmt = get_compound_stmt(ast_element);
    let func_creation_args = ast_element.create_func_creation_args();

    match compound_stmt {
        Some(_compound_stmt) => {
            let mut _func_impl = walker
                .current_file
                .borrow_mut()
                .get_or_add_func_impl(func_creation_args);
        }
        None => {
            walker
                .current_file
                .borrow_mut()
                .get_or_add_func_decl(func_creation_args);
        }
    }
}

fn get_compound_stmt(ast_element: &ClangAstElement) -> Option<&ClangAstElement> {
    for child in &ast_element.inner {
        if child.element_type == "CompoundStmt" {
            return Some(child);
        }
    }
    None
}

impl ClangAstElement {
    fn create_func_creation_args(&self) -> FuncCreationArgs {
        let splitted_attributes: Vec<&str> = self.attributes.split(" ").collect();
        let start_index = get_in_function_qual_type_start_index(&splitted_attributes);
        let end_index = get_in_function_qual_type_end_index(&splitted_attributes);
        let binding = splitted_attributes[start_index..end_index + 1].join(" ");
        let qualified_type = binding.as_str();

        FuncCreationArgs::new(
            splitted_attributes[start_index - 1],
            splitted_attributes[start_index - 1..end_index + 1]
                .join(" ")
                .as_str(),
            qualified_type[1..qualified_type.len() - 1]
                .to_string()
                .as_str(),
            self.range.clone(),
        )
    }
}

fn get_in_function_qual_type_start_index(current_vec: &Vec<&str>) -> usize {
    for (i, elem) in current_vec.iter().enumerate() {
        if elem.starts_with("'") {
            return i;
        }
    }
    panic!("No name start found in func name: {:?}", current_vec);
}

fn get_in_function_qual_type_end_index(current_vec: &Vec<&str>) -> usize {
    for (i, elem) in current_vec.iter().enumerate().rev() {
        if elem.ends_with("'") {
            return i;
        }
    }
    panic!("No name start found in func name: {:?}", current_vec);
}

#[cfg(test)]
mod tests {
    use crate::location::range::Range;

    use super::*;

    #[test]
    fn get_in_function_qual_type_start_index_test() {
        assert_eq!(
            get_in_function_qual_type_start_index(&vec!["add", "'int", "(int,", "int)'", "extern"]),
            1
        );
        assert_eq!(
            get_in_function_qual_type_start_index(&vec![
                "blub", "add", "'int", "(int,", "int)'", "extern"
            ]),
            2
        );
    }

    #[test]
    fn get_in_function_qual_type_end_index_test() {
        assert_eq!(
            get_in_function_qual_type_end_index(&vec!["add", "'int", "(int,", "int)'", "extern"]),
            3
        );
        assert_eq!(
            get_in_function_qual_type_end_index(&vec!["blub", "add", "'int", "(int,", "int)'"]),
            4
        );
    }

    #[test]
    fn create_func_creation_args_test() {
        let input = ClangAstElement {
            element_type: "FunctionDecl".to_string(),
            element_id: 0x123011160,
            parent_element_id: 0,
            prev_element_id: 0,
            file: Rc::new("test.cpp".to_string()),
            range: Range::create(1, 2, 3, 4),
            inner: VecDeque::new(),
            attributes: "add 'int (int, int)'".to_string(),
        };
        let converted_args = input.create_func_creation_args();

        let expected_args = FuncCreationArgs {
            name: "add".to_string(),
            qualified_name: "add 'int (int, int)'".to_string(),
            qualified_type: "int (int, int)".to_string(),
            range: Range::create(1, 2, 3, 4),
        };

        assert_eq!(converted_args, expected_args);
    }

    #[test]
    fn create_func_creation_args_test_with_used() {
        let input = ClangAstElement {
            element_type: "FunctionDecl".to_string(),
            element_id: 0x123011160,
            parent_element_id: 0,
            prev_element_id: 0,
            file: Rc::new("test.cpp".to_string()),
            range: Range::create(1, 2, 3, 4),
            inner: VecDeque::new(),
            attributes: "used add 'int (int, int)'".to_string(),
        };
        let converted_args = input.create_func_creation_args();

        let expected_args = FuncCreationArgs {
            name: "add".to_string(),
            qualified_name: "add 'int (int, int)'".to_string(),
            qualified_type: "int (int, int)".to_string(),
            range: Range::create(1, 2, 3, 4),
        };

        assert_eq!(converted_args, expected_args);
    }

    #[test]
    fn create_func_creation_args_test_with_extern() {
        let input = ClangAstElement {
            element_type: "FunctionDecl".to_string(),
            element_id: 0x123011160,
            parent_element_id: 0,
            prev_element_id: 0,
            file: Rc::new("test.cpp".to_string()),
            range: Range::create(1, 2, 3, 4),
            inner: VecDeque::new(),
            attributes: "add 'int (int, int)' extern".to_string(),
        };
        let converted_args = input.create_func_creation_args();

        let expected_args = FuncCreationArgs {
            name: "add".to_string(),
            qualified_name: "add 'int (int, int)'".to_string(),
            qualified_type: "int (int, int)".to_string(),
            range: Range::create(1, 2, 3, 4),
        };

        assert_eq!(converted_args, expected_args);
    }
}
