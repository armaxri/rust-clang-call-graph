use std::rc::Rc;

use rusqlite::Connection;

#[derive(Debug, Clone)]
pub struct DatabaseSqliteInternal {
    pub db: Rc<Connection>,
}

impl PartialEq for DatabaseSqliteInternal {
    fn eq(&self, _: &Self) -> bool {
        return true;
    }
}

impl Eq for DatabaseSqliteInternal {}

impl DatabaseSqliteInternal {
    pub fn new(db: Rc<Connection>) -> Self {
        DatabaseSqliteInternal { db }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_sqlite_internal() {
        let db_connection = Rc::new(Connection::open_in_memory().unwrap());
        let db = DatabaseSqliteInternal {
            db: db_connection.clone(),
        };

        assert_eq!(db, db);

        let db2_instance = db.clone();
        assert_eq!(db, db2_instance);
    }
}
