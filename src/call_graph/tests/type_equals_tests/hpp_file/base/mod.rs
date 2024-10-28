#[cfg(test)]
mod tests {
    use crate::{
        call_graph::{
            data_structure::{helper::func_creation_args::FuncCreationArgs, MainDeclPosition},
            database::{database_content::DatabaseContent, database_sqlite::DatabaseSqlite},
        },
        file_in_directory, func_file_in_directory,
        location::{position::Position, range::Range},
    };

    #[test]
    fn test_empty_database() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content =
            DatabaseContent::load_from_file(&file_in_directory!("../../empty_expected_db.json"));

        assert_eq!(sqlite_content, json_content);
    }

    #[test]
    fn test_empty_file() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        database_sqlite.get_or_add_hpp_file("empty.json");

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content =
            DatabaseContent::load_from_file(&file_in_directory!("empty_file_expected_db.json"));

        assert_eq!(sqlite_content, json_content);
    }

    #[test]
    fn test_not_equal_empty_vs_filled_database() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "simple_func_decl_expected_db.json"
        ));

        assert_ne!(sqlite_content, json_content);
    }

    #[test]
    fn test_wrong_file_name() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let hpp_file = database_sqlite.get_or_add_hpp_file("simple_func_decl_expected_db.json");
        hpp_file.borrow_mut().add_func_decl(FuncCreationArgs::new(
            "add",
            "__ZN3foo3addEii",
            "int (int, int)",
            Range::new(Position::new(11, 5), Position::new(11, 8)),
        ));

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "simple_func_decl_expected_db.json"
        ));

        assert_ne!(sqlite_content, json_content);
    }

    #[test]
    fn test_impl_file() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let hpp_file = database_sqlite.get_or_add_hpp_file("simple_func_decl.json");
        hpp_file.borrow_mut().add_func_decl(FuncCreationArgs::new(
            "add",
            "__ZN3foo3addEii",
            "int (int, int)",
            Range::new(Position::new(11, 5), Position::new(11, 8)),
        ));

        let sqlite_content = database_sqlite.get_db_content();
        sqlite_content.save_to_file(&func_file_in_directory!("json"));

        let json_content = DatabaseContent::load_from_file(&file_in_directory!(
            "simple_func_decl_expected_db.json"
        ));

        assert_eq!(sqlite_content, json_content);
    }

    #[test]
    fn test_just_analyzed() {
        let database_sqlite =
            DatabaseSqlite::create_database(&func_file_in_directory!("db").into(), true);

        let timestamp1 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        std::thread::sleep(std::time::Duration::from_secs(1));

        let hpp_file = database_sqlite.get_or_add_hpp_file("empty.json");
        std::thread::sleep(std::time::Duration::from_secs(1));
        let timestamp2 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        assert!(hpp_file.borrow().get_last_analyzed() > timestamp1);
        assert!(hpp_file.borrow().get_last_analyzed() < timestamp2);

        std::thread::sleep(std::time::Duration::from_secs(1));
        hpp_file.borrow_mut().just_analyzed();
        std::thread::sleep(std::time::Duration::from_secs(1));

        let timestamp3 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        assert!(hpp_file.borrow().get_last_analyzed() > timestamp2);
        assert!(hpp_file.borrow().get_last_analyzed() < timestamp3);
    }
}
