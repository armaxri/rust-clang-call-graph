use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use crate::{
    ast_reader::clang_ast_element::ClangAstElement,
    call_graph::database::database_sqlite::DatabaseSqlite,
};

struct ClangAstWalkerInternal {
    pub db: Rc<RefCell<DatabaseSqlite>>,
}

pub fn walk_ast_2_func_call_db(
    _file_path: &str,
    _parsed_ast: VecDeque<ClangAstElement>,
    _db: Rc<RefCell<DatabaseSqlite>>,
) {
    let walker = ClangAstWalkerInternal { db: _db };

    for _ast_element in _parsed_ast {
        // Do something with _ast_element
    }
}
