use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::{
    ast_reader::clang_ast_element::ClangAstElement,
    call_graph::{
        data_structure::{file_structure::FileStructure, MainDeclPosition},
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
    }
}
