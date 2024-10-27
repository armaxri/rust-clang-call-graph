#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        call_graph::{
            data_structure::{
                func_structure::FuncStructure,
                helper::{
                    func_creation_args::FuncCreationArgs,
                    virtual_func_creation_args::VirtualFuncCreationArgs,
                },
                FuncBasics, FuncImplBasics, MainDeclPosition, MatchingFuncs, VirtualFuncBasics,
            },
            database::database_sqlite::DatabaseSqlite,
        },
        location::{position::Position, range::Range},
    };

    #[test]
    fn test_get_matching_funcs_virtual_func_call() {
        let database_sqlite = DatabaseSqlite::create_in_memory_database();

        let cpp_file = database_sqlite.get_or_add_hpp_file("file.cpp");
        let cpp_class = cpp_file.borrow_mut().get_or_add_class("Foo");

        let func_decl = cpp_class
            .borrow_mut()
            .add_virtual_func_decl(VirtualFuncCreationArgs {
                name: "func_decl".to_string(),
                qualified_name: "func_decl".to_string(),
                base_qualified_name: "func_decl".to_string(),
                qualified_type: "int".to_string(),
                range: Range::new(Position::new(1, 2), Position::new(1, 10)),
            });

        let func_impl = cpp_class
            .borrow_mut()
            .add_virtual_func_impl(VirtualFuncCreationArgs {
                name: "func".to_string(),
                qualified_name: "func".to_string(),
                base_qualified_name: "func".to_string(),
                qualified_type: "int".to_string(),
                range: Range::new(Position::new(3, 2), Position::new(3, 10)),
            });

        let func_call_args = &func_decl
            .borrow()
            .convert_virtual_func2virtual_func_creation_args4call(&Range::new(
                Position::new(2, 2),
                Position::new(2, 10),
            ));

        let func = func_impl
            .borrow_mut()
            .get_or_add_virtual_func_call(&func_call_args);

        let mut matches: Vec<Rc<RefCell<FuncStructure>>> = Vec::new();
        cpp_file
            .borrow()
            .get_matching_funcs(&Position::new(2, 5), &mut matches);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0], func);
    }

    #[test]
    fn test_get_matching_funcs_virtual_func_decl() {
        let database_sqlite = DatabaseSqlite::create_in_memory_database();

        let cpp_file = database_sqlite.get_or_add_hpp_file("file.cpp");
        let cpp_class = cpp_file.borrow_mut().get_or_add_class("Foo");

        let func_decl = cpp_class
            .borrow_mut()
            .add_virtual_func_decl(VirtualFuncCreationArgs {
                name: "func_decl".to_string(),
                qualified_name: "func_decl".to_string(),
                base_qualified_name: "func_decl".to_string(),
                qualified_type: "int".to_string(),
                range: Range::new(Position::new(1, 2), Position::new(1, 10)),
            });

        let func_impl = cpp_class
            .borrow_mut()
            .add_virtual_func_impl(VirtualFuncCreationArgs {
                name: "func".to_string(),
                qualified_name: "func".to_string(),
                base_qualified_name: "func".to_string(),
                qualified_type: "int".to_string(),
                range: Range::new(Position::new(3, 2), Position::new(3, 10)),
            });

        let func_call_args = &func_decl
            .borrow()
            .convert_virtual_func2virtual_func_creation_args4call(&Range::new(
                Position::new(2, 2),
                Position::new(2, 10),
            ));

        func_impl
            .borrow_mut()
            .get_or_add_virtual_func_call(&func_call_args);

        let mut matches: Vec<Rc<RefCell<FuncStructure>>> = Vec::new();
        cpp_file
            .borrow()
            .get_matching_funcs(&Position::new(1, 5), &mut matches);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0], func_decl);
    }

    #[test]
    fn test_get_matching_funcs_virtual_func_impl() {
        let database_sqlite = DatabaseSqlite::create_in_memory_database();

        let cpp_file = database_sqlite.get_or_add_hpp_file("file.cpp");
        let cpp_class = cpp_file.borrow_mut().get_or_add_class("Foo");

        let func_decl = cpp_class
            .borrow_mut()
            .add_virtual_func_decl(VirtualFuncCreationArgs {
                name: "func_decl".to_string(),
                qualified_name: "func_decl".to_string(),
                base_qualified_name: "func_decl".to_string(),
                qualified_type: "int".to_string(),
                range: Range::new(Position::new(1, 2), Position::new(1, 10)),
            });

        let func_impl = cpp_class
            .borrow_mut()
            .add_virtual_func_impl(VirtualFuncCreationArgs {
                name: "func".to_string(),
                qualified_name: "func".to_string(),
                base_qualified_name: "func".to_string(),
                qualified_type: "int".to_string(),
                range: Range::new(Position::new(3, 2), Position::new(3, 10)),
            });

        let func_call_args = &func_decl
            .borrow()
            .convert_virtual_func2virtual_func_creation_args4call(&Range::new(
                Position::new(2, 2),
                Position::new(2, 10),
            ));

        func_impl
            .borrow_mut()
            .get_or_add_virtual_func_call(&func_call_args);

        let mut matches: Vec<Rc<RefCell<FuncStructure>>> = Vec::new();
        cpp_file
            .borrow()
            .get_matching_funcs(&Position::new(3, 5), &mut matches);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0], func_impl);
    }

    #[test]
    fn test_get_matching_funcs_no_matches() {
        let database_sqlite = DatabaseSqlite::create_in_memory_database();

        let cpp_file = database_sqlite.get_or_add_hpp_file("file.cpp");
        let cpp_class = cpp_file.borrow_mut().get_or_add_class("Foo");

        let func_decl = cpp_file.borrow_mut().add_func_decl(FuncCreationArgs {
            name: "func_decl".to_string(),
            qualified_name: "func_decl".to_string(),
            qualified_type: "int".to_string(),
            range: Range::new(Position::new(1, 2), Position::new(1, 10)),
        });

        let func_impl = cpp_class
            .borrow_mut()
            .add_virtual_func_impl(VirtualFuncCreationArgs {
                name: "func".to_string(),
                qualified_name: "func".to_string(),
                base_qualified_name: "func".to_string(),
                qualified_type: "int".to_string(),
                range: Range::new(Position::new(3, 2), Position::new(3, 10)),
            });

        let func_call_args = &func_decl
            .borrow()
            .convert_func2func_creation_args4call(&Range::new(
                Position::new(2, 2),
                Position::new(2, 10),
            ));

        func_impl.borrow_mut().get_or_add_func_call(&func_call_args);

        let mut matches: Vec<Rc<RefCell<FuncStructure>>> = Vec::new();
        cpp_file
            .borrow()
            .get_matching_funcs(&Position::new(1, 1), &mut matches);
        assert_eq!(matches.len(), 0);

        let mut matches: Vec<Rc<RefCell<FuncStructure>>> = Vec::new();
        cpp_file
            .borrow()
            .get_matching_funcs(&Position::new(2, 11), &mut matches);
        assert_eq!(matches.len(), 0);

        let mut matches: Vec<Rc<RefCell<FuncStructure>>> = Vec::new();
        cpp_file
            .borrow()
            .get_matching_funcs(&Position::new(3, 1), &mut matches);
        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_get_matching_funcs_func_decl() {
        let database_sqlite = DatabaseSqlite::create_in_memory_database();

        let cpp_file = database_sqlite.get_or_add_hpp_file("file.cpp");

        let func = cpp_file.borrow_mut().add_func_decl(FuncCreationArgs {
            name: "func".to_string(),
            qualified_name: "func".to_string(),
            qualified_type: "int".to_string(),
            range: Range::new(Position::new(2, 2), Position::new(2, 10)),
        });

        let mut matches: Vec<Rc<RefCell<FuncStructure>>> = Vec::new();
        cpp_file
            .borrow()
            .get_matching_funcs(&Position::new(2, 5), &mut matches);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0], func);
    }

    #[test]
    fn test_get_matching_funcs_func_impl() {
        let database_sqlite = DatabaseSqlite::create_in_memory_database();

        let cpp_file = database_sqlite.get_or_add_hpp_file("file.cpp");

        let func = cpp_file.borrow_mut().add_func_impl(FuncCreationArgs {
            name: "func".to_string(),
            qualified_name: "func".to_string(),
            qualified_type: "int".to_string(),
            range: Range::new(Position::new(2, 2), Position::new(2, 10)),
        });

        let mut matches: Vec<Rc<RefCell<FuncStructure>>> = Vec::new();
        cpp_file
            .borrow()
            .get_matching_funcs(&Position::new(2, 5), &mut matches);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0], func);
    }
}
