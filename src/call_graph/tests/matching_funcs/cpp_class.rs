#[cfg(test)]
mod tests {
    use crate::{
        call_graph::{
            data_structure::{
                cpp_file::CppFile,
                helper::{
                    location::Location, range::Range,
                    virtual_func_creation_args::VirtualFuncCreationArgs,
                },
                FuncImplBasics, MainDeclLocation, VirtualFuncBasics,
            },
            database::database_sqlite::create_in_memory_database,
        },
        file_in_directory,
    };

    #[test]
    fn test_get_matching_funcs_call() {
        let db_connection = create_in_memory_database();

        let file = CppFile::create_cpp_file(&db_connection, "file.cpp", None);
        let cpp_class = file.borrow_mut().add_class("Foo");

        let func2call = cpp_class
            .borrow_mut()
            .add_virtual_func_decl(VirtualFuncCreationArgs {
                name: "func2call".to_string(),
                qualified_name: "func2call".to_string(),
                base_qualified_name: "func2call".to_string(),
                qualified_type: "int".to_string(),
                range: Range::new(Location::new(1, 2), Location::new(1, 10)),
            });

        let func_impl = cpp_class
            .borrow_mut()
            .add_virtual_func_impl(VirtualFuncCreationArgs {
                name: "func".to_string(),
                qualified_name: "func".to_string(),
                base_qualified_name: "func".to_string(),
                qualified_type: "int".to_string(),
                range: Range::new(Location::new(3, 2), Location::new(3, 10)),
            });

        let func_call_args = &func2call
            .borrow()
            .convert_virtual_func2virtual_func_creation_args4call(&Range::new(
                Location::new(2, 2),
                Location::new(2, 10),
            ));

        let func = func_impl
            .borrow_mut()
            .add_virtual_func_call(&func_call_args);

        // Drop the mutable borrow of cpp_class before the immutable borrow
        let _func = func.clone();

        print!("{:?}", file!());
        //print!("{:?}", directory!());
        print!("{:?}", self::file_in_directory!("keks"));
        println!("{:?}", serde_json::to_string_pretty(&file));

        /* TODO
        let matches = cpp_class.borrow().get_matching_funcs(Location::new(2, 5));

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].virtual_func_call, func);
        */
    }
}
