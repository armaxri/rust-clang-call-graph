#[cfg(test)]
mod tests {
    use crate::{
        call_graph::{
            data_structure::MainDeclLocation,
            database::{database_content::DatabaseContent, database_sqlite::DatabaseSqlite},
        },
        file_in_directory, func_file_in_directory,
    };

    #[test]
    fn test_equality_with_simple_parent_class() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("simple_parent_cpp_class.json");
        let parent_class = cpp_file.borrow_mut().add_class("ParentClass");
        let child_class = cpp_file.borrow_mut().add_class("ChildClass");

        child_class.borrow_mut().add_parent_class(&parent_class);

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "simple_parent_cpp_class_expected_db.json"
        ));

        assert_eq!(sqlite_content, json_content);
    }

    #[test]
    fn test_double_add_with_simple_parent_class() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("simple_parent_cpp_class.json");
        let parent_class = cpp_file.borrow_mut().add_class("ParentClass");
        let child_class = cpp_file.borrow_mut().add_class("ChildClass");

        child_class.borrow_mut().add_parent_class(&parent_class);
        child_class.borrow_mut().add_parent_class(&parent_class);

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "simple_parent_cpp_class_expected_db.json"
        ));

        assert_eq!(sqlite_content, json_content);
    }

    #[test]
    fn test_equality_with_multiple_simple_parent_class() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("multiple_simple_parent_cpp_class.json");
        let parent_class1 = cpp_file.borrow_mut().add_class("ParentClass1");
        let parent_class2 = cpp_file.borrow_mut().add_class("ParentClass2");
        let parent_class3 = cpp_file.borrow_mut().add_class("ParentClass3");
        let parent_class4 = cpp_file.borrow_mut().add_class("ParentClass4");
        let child_class = cpp_file.borrow_mut().add_class("ChildClass");

        child_class.borrow_mut().add_parent_class(&parent_class1);
        child_class.borrow_mut().add_parent_class(&parent_class2);
        child_class.borrow_mut().add_parent_class(&parent_class3);
        child_class.borrow_mut().add_parent_class(&parent_class4);

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "multiple_simple_parent_cpp_class_expected_db.json"
        ));

        assert_eq!(sqlite_content, json_content);
    }

    #[test]
    fn test_no_equality_with_multiple_simple_parent_class_missing_parent() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("multiple_simple_parent_cpp_class.json");
        let parent_class1 = cpp_file.borrow_mut().add_class("ParentClass1");
        let parent_class2 = cpp_file.borrow_mut().add_class("ParentClass2");
        let parent_class3 = cpp_file.borrow_mut().add_class("ParentClass3");
        cpp_file.borrow_mut().add_class("ParentClass4");
        let child_class = cpp_file.borrow_mut().add_class("ChildClass");

        child_class.borrow_mut().add_parent_class(&parent_class1);
        child_class.borrow_mut().add_parent_class(&parent_class2);
        child_class.borrow_mut().add_parent_class(&parent_class3);

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "multiple_simple_parent_cpp_class_expected_db.json"
        ));

        assert_ne!(sqlite_content, json_content);
    }

    #[test]
    fn test_equality_with_multiple_chained_parent_classes() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("parent_cpp_class_chain.json");
        let grand_parent_class = cpp_file.borrow_mut().add_class("GrandParentClass");
        let parent_class = cpp_file.borrow_mut().add_class("ParentClass");
        let child_class = cpp_file.borrow_mut().add_class("ChildClass");

        parent_class
            .borrow_mut()
            .add_parent_class(&grand_parent_class);
        child_class.borrow_mut().add_parent_class(&parent_class);

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "parent_cpp_class_chain_expected_db.json"
        ));

        assert_eq!(sqlite_content, json_content);
    }

    #[test]
    fn test_no_equality_with_multiple_chained_parent_class_missing_middle_class() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("parent_cpp_class_chain.json");
        let grand_parent_class = cpp_file.borrow_mut().add_class("GrandParentClass");
        let parent_class = cpp_file.borrow_mut().add_class("ParentClass");
        let child_class = cpp_file.borrow_mut().add_class("ChildClass");

        parent_class
            .borrow_mut()
            .add_parent_class(&grand_parent_class);
        child_class
            .borrow_mut()
            .add_parent_class(&grand_parent_class);

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "parent_cpp_class_chain_expected_db.json"
        ));

        assert_ne!(sqlite_content, json_content);
    }

    #[test]
    fn test_get_parent_classes() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("simple_parent_cpp_class.json");
        let parent_class = cpp_file.borrow_mut().add_class("ParentClass");
        let child_class = cpp_file.borrow_mut().add_class("ChildClass");

        child_class.borrow_mut().add_parent_class(&parent_class);

        let parent_classes = child_class.borrow().get_parent_classes();

        assert_eq!(parent_classes.len(), 1);
        assert_eq!(parent_classes[0].borrow().get_name(), "ParentClass");
        assert_eq!(parent_classes[0], parent_class);
    }

    #[test]
    fn test_equality_with_simple_parent_class_in_different_files() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let hpp_file = database_sqlite.get_or_add_hpp_file("ParentClass.h");
        let parent_class = hpp_file.borrow_mut().add_class("ParentClass");
        let cpp_file = database_sqlite.get_or_add_cpp_file("ChildClass.cpp");
        let child_class = cpp_file.borrow_mut().add_class("ChildClass");

        child_class.borrow_mut().add_parent_class(&parent_class);

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "parent_cpp_class_in_hpp_expected_db.json"
        ));

        assert_eq!(sqlite_content, json_content);
    }

    #[test]
    fn test_removed_all_database_content() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let cpp_file = database_sqlite.get_or_add_cpp_file("simple_parent_cpp_class.json");
        let parent_class = cpp_file.borrow_mut().add_class("ParentClass");
        let child_class = cpp_file.borrow_mut().add_class("ChildClass");

        child_class.borrow_mut().add_parent_class(&parent_class);

        database_sqlite.remove_cpp_file_and_depending_content(cpp_file.borrow().get_name());

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content =
            DatabaseContent::load_from_file(&file_in_directory!("../../empty_expected_db.json"));

        assert_eq!(sqlite_content, json_content);
    }
}
