use std::cell::RefCell;
use std::rc::Rc;

use rusqlite::params;
use serde::Deserialize;
use serde::Serialize;

use crate::location::position::Position;

use super::super::database::database_sqlite_internal::DatabaseSqliteInternal;
use super::func_structure::FuncStructure;
use super::helper::virtual_func_creation_args::VirtualFuncCreationArgs;
use super::FuncBasics;
use super::MainDeclPosition;
use super::MatchingFuncs;
use super::VirtualFuncBasics;

#[derive(Deserialize, Serialize, Debug, Clone, Eq)]
pub struct CppClass {
    id: u64,
    #[serde(skip)]
    db_connection: Option<DatabaseSqliteInternal>,

    name: String,
    parent_classes: Vec<String>,
    classes: Vec<Rc<RefCell<CppClass>>>,
    func_decls: Vec<Rc<RefCell<FuncStructure>>>,
    func_impls: Vec<Rc<RefCell<FuncStructure>>>,
    virtual_func_decls: Vec<Rc<RefCell<FuncStructure>>>,
    virtual_func_impls: Vec<Rc<RefCell<FuncStructure>>>,
}

impl PartialEq for CppClass {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
            && self.name == other.name
            && self.parent_classes == other.parent_classes
            && self.classes == other.classes
            && self.func_decls == other.func_decls
            && self.func_impls == other.func_impls
            && self.virtual_func_decls == other.virtual_func_decls
            && self.virtual_func_impls == other.virtual_func_impls;
    }
}

impl MatchingFuncs for CppClass {
    fn get_matching_funcs(
        &self,
        position: &Position,
        results: &mut Vec<Rc<RefCell<FuncStructure>>>,
    ) {
        for cpp_class in self.classes.iter() {
            cpp_class.borrow().get_matching_funcs(position, results);
        }
        for func_decl in self.func_decls.iter() {
            if func_decl.borrow().matches_position(position) {
                results.push(func_decl.clone());
            }
        }
        for func_impl in self.func_impls.iter() {
            if func_impl.borrow().matches_position(position) {
                results.push(func_impl.clone());
            }
            func_impl.borrow().get_matching_funcs(position, results);
        }
        for virtual_func_decl in self.virtual_func_decls.iter() {
            if virtual_func_decl.borrow().matches_position(position) {
                results.push(virtual_func_decl.clone());
            }
        }
        for virtual_func_impl in self.virtual_func_impls.iter() {
            if virtual_func_impl.borrow().matches_position(position) {
                results.push(virtual_func_impl.clone());
            }
            virtual_func_impl
                .borrow()
                .get_matching_funcs(position, results);
        }
    }
}

impl MainDeclPosition for CppClass {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_db_connection(&self) -> Option<DatabaseSqliteInternal> {
        self.db_connection.clone()
    }

    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_main_decl_position_id(&self) -> (Option<u64>, Option<u64>, Option<u64>) {
        (None, None, Some(self.id))
    }

    fn get_classes(&mut self) -> &mut Vec<Rc<RefCell<CppClass>>> {
        &mut self.classes
    }

    fn get_func_decls(&mut self) -> &mut Vec<Rc<RefCell<FuncStructure>>> {
        &mut self.func_decls
    }

    fn get_func_impls(&mut self) -> &mut Vec<Rc<RefCell<FuncStructure>>> {
        &mut self.func_impls
    }

    fn get_virtual_func_impls(&mut self) -> &mut Vec<Rc<RefCell<FuncStructure>>> {
        &mut self.virtual_func_impls
    }
}

impl CppClass {
    pub fn new(id: u64, db_connection: Option<DatabaseSqliteInternal>, name: String) -> Self {
        let mut new_class = Self {
            id,
            db_connection,
            name,
            parent_classes: Vec::new(),
            classes: Vec::new(),
            func_decls: Vec::new(),
            func_impls: Vec::new(),
            virtual_func_decls: Vec::new(),
            virtual_func_impls: Vec::new(),
        };

        if new_class.db_connection.is_some() {
            new_class.read_parent_classes();

            new_class.classes = CppClass::get_cpp_classes(
                &new_class.db_connection.as_ref().unwrap(),
                (None, None, Some(id)),
            );

            new_class.func_decls = FuncStructure::get_func_decls(
                new_class.db_connection.as_ref().unwrap(),
                (None, None, Some(new_class.id)),
            );
            new_class.func_impls = FuncStructure::get_func_impls(
                new_class.db_connection.as_ref().unwrap(),
                (None, None, Some(new_class.id)),
            );
            new_class.virtual_func_decls = FuncStructure::get_virtual_func_decls(
                new_class.db_connection.as_ref().unwrap(),
                (None, None, Some(new_class.id)),
            );
            new_class.virtual_func_impls = FuncStructure::get_virtual_func_impls(
                new_class.db_connection.as_ref().unwrap(),
                (None, None, Some(new_class.id)),
            );
        }

        new_class
    }

    pub fn create_cpp_class(
        db_connection: &DatabaseSqliteInternal,
        class_name: &str,
        parent_id: (Option<u64>, Option<u64>, Option<u64>),
    ) -> Self {
        let mut stmt = db_connection
            .db
            .prepare(
                "
            INSERT INTO cpp_classes (class_name, cpp_file_id, hpp_file_id, cpp_class_id)
            VALUES (?, ?, ?, ?)",
            )
            .unwrap();
        let result = stmt.insert(params![class_name, parent_id.0, parent_id.1, parent_id.2]);

        Self::new(
            result.unwrap() as u64,
            Some(db_connection.clone()),
            class_name.to_string(),
        )
    }

    pub fn get_cpp_classes(
        db_connection: &DatabaseSqliteInternal,
        parent_id: (Option<u64>, Option<u64>, Option<u64>),
    ) -> Vec<Rc<RefCell<CppClass>>> {
        let mut stmt = db_connection
            .db
            .prepare(
                "
            SELECT id, class_name
            FROM cpp_classes
            WHERE cpp_file_id = ? OR hpp_file_id = ? OR cpp_class_id = ?",
            )
            .unwrap();
        let cpp_classes_iter = stmt
            .query_map(params![parent_id.0, parent_id.1, parent_id.2], |row| {
                Ok(Rc::new(RefCell::new(CppClass::new(
                    row.get(0).unwrap(),
                    Some(db_connection.clone()),
                    row.get(1).unwrap(),
                ))))
            })
            .unwrap();

        cpp_classes_iter
            .map(|cpp_class| cpp_class.unwrap())
            .collect()
    }

    fn read_parent_classes(&mut self) {
        let mut stmt = self
            .db_connection
            .as_ref()
            .unwrap()
            .db
            .prepare(
                "
            SELECT parent_class_id
            FROM cpp_classes_2_cpp_classes
            WHERE child_class_id = ?",
            )
            .unwrap();
        let parent_classes_iter = stmt
            .query_map(params![self.id], |row| {
                Ok(row.get::<_, usize>(0).unwrap() as usize)
            })
            .unwrap();

        for parent_id in parent_classes_iter.map(|parent_class_id| parent_class_id.unwrap()) {
            let parent_class =
                Self::get_class_from_id(self.db_connection.as_ref().unwrap(), parent_id as u64);
            match parent_class {
                Some(parent_class) => self
                    .parent_classes
                    .push(parent_class.borrow().get_name().to_string()),
                None => panic!("Parent class not found"),
            }
        }
    }

    fn get_class_from_id(
        db_connection: &DatabaseSqliteInternal,
        id: u64,
    ) -> Option<Rc<RefCell<Self>>> {
        let mut stmt = db_connection
            .db
            .prepare(
                "
            SELECT class_name
            FROM cpp_classes
            WHERE id = ?",
            )
            .unwrap();
        let mut rows = stmt.query(params![id]).unwrap();

        if let Some(row) = rows.next().unwrap() {
            Some(Rc::new(RefCell::new(Self::new(
                id,
                Some(db_connection.clone()),
                row.get(0).unwrap(),
            ))))
        } else {
            None
        }
    }

    pub fn get_virtual_func_decls(&mut self) -> &mut Vec<Rc<RefCell<FuncStructure>>> {
        &mut self.virtual_func_decls
    }

    pub fn add_virtual_func_decl(
        &mut self,
        creation_args: VirtualFuncCreationArgs,
    ) -> Rc<RefCell<FuncStructure>> {
        let new_virtual_func_decl = Rc::new(RefCell::new(FuncStructure::create_virtual_func_decl(
            &self.get_db_connection().unwrap(),
            &creation_args,
            self.get_main_decl_position_id(),
        )));
        self.get_virtual_func_decls().push(new_virtual_func_decl);
        self.get_virtual_func_decls().last().unwrap().clone()
    }

    pub fn get_or_add_virtual_func_decl(
        &mut self,
        creation_args: VirtualFuncCreationArgs,
    ) -> Rc<RefCell<FuncStructure>> {
        if self
            .get_virtual_func_decls()
            .iter()
            .any(|c| c.borrow().equals_virtual_func_creation_args(&creation_args))
        {
            self.get_virtual_func_decls()
                .iter()
                .find(|c| c.borrow().equals_virtual_func_creation_args(&creation_args))
                .unwrap()
                .clone()
        } else {
            self.add_virtual_func_decl(creation_args)
        }
    }

    pub fn get_parent_classes(&self) -> Vec<Rc<RefCell<CppClass>>> {
        let mut parent_classes = Vec::new();
        let mut stmt = self
            .db_connection
            .as_ref()
            .unwrap()
            .db
            .prepare(
                "
            SELECT parent_class_id
            FROM cpp_classes_2_cpp_classes
            WHERE child_class_id = ?",
            )
            .unwrap();
        let parent_classes_iter = stmt
            .query_map(params![self.id], |row| {
                Ok(row.get::<_, usize>(0).unwrap() as usize)
            })
            .unwrap();

        for parent_id in parent_classes_iter.map(|parent_class_id| parent_class_id.unwrap()) {
            let parent_class =
                Self::get_class_from_id(self.db_connection.as_ref().unwrap(), parent_id as u64);
            match parent_class {
                Some(parent_class) => parent_classes.push(parent_class),
                None => panic!("Parent class not found"),
            }
        }
        parent_classes
    }

    pub fn get_parent_classes_names(&self) -> Vec<String> {
        self.parent_classes.clone()
    }

    pub fn add_parent_class(&mut self, parent_class: &Rc<RefCell<CppClass>>) {
        if self
            .parent_classes
            .iter()
            .any(|parent_class_name| parent_class_name == &parent_class.borrow().name)
        {
            return;
        }

        let mut stmt = self
            .db_connection
            .as_ref()
            .unwrap()
            .db
            .prepare(
                "
            INSERT INTO cpp_classes_2_cpp_classes (parent_class_id, child_class_id)
            VALUES (?, ?)",
            )
            .unwrap();
        stmt.insert(params![parent_class.borrow().id, self.id])
            .unwrap();
        self.parent_classes.push(parent_class.borrow().name.clone());
    }
}

pub const CPP_CLASS_SQL_CREATE_TABLE: &str = "
CREATE TABLE cpp_classes (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    class_name   TEXT NOT NULL,

    cpp_file_id  INTEGER NULL,
    hpp_file_id  INTEGER NULL,
    cpp_class_id INTEGER NULL,

    FOREIGN KEY (cpp_file_id) REFERENCES cpp_files(id) ON DELETE CASCADE,
    FOREIGN KEY (hpp_file_id) REFERENCES hpp_files(id) ON DELETE CASCADE,
    FOREIGN KEY (cpp_class_id) REFERENCES cpp_classes(id) ON DELETE CASCADE
)
";

pub const CPP_CLASS_2_CLASS_SQL_CREATE_TABLE: &str = "
CREATE TABLE cpp_classes_2_cpp_classes (
    parent_class_id INTEGER,
    child_class_id  INTEGER,

    PRIMARY KEY (parent_class_id, child_class_id),
    FOREIGN KEY (parent_class_id) REFERENCES cpp_classes(id) ON DELETE CASCADE,
    FOREIGN KEY (child_class_id) REFERENCES cpp_classes(id) ON DELETE CASCADE
)
";

pub fn create_database_tables(db_connection: &DatabaseSqliteInternal) {
    let _ = db_connection.db.execute_batch(CPP_CLASS_SQL_CREATE_TABLE);
    let _ = db_connection
        .db
        .execute_batch(CPP_CLASS_2_CLASS_SQL_CREATE_TABLE);
}
