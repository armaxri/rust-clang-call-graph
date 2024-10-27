use std::{cell::RefCell, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::{
    call_graph::database::database_sqlite_internal::DatabaseSqliteInternal,
    location::position::Position,
};

use super::{
    cpp_class::CppClass, func_structure::FuncStructure, File, MainDeclPosition, MatchingFuncs,
};

#[derive(Deserialize, Serialize, Debug, Clone, Eq)]
pub struct FileStructure {
    id: u64,
    #[serde(skip)]
    db_connection: Option<DatabaseSqliteInternal>,

    name: String,
    last_analyzed: i64,
    classes: Vec<Rc<RefCell<CppClass>>>,
    func_decls: Vec<Rc<RefCell<FuncStructure>>>,
    func_impls: Vec<Rc<RefCell<FuncStructure>>>,
    virtual_func_impls: Vec<Rc<RefCell<FuncStructure>>>,
    referenced_from_header_files: Vec<String>,
    referenced_from_source_files: Vec<String>,

    file_is_header: bool,
}

impl PartialEq for FileStructure {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
            && self.name == other.name
            && self.classes == other.classes
            && self.func_decls == other.func_decls
            && self.func_impls == other.func_impls
            && self.virtual_func_impls == other.virtual_func_impls
            && self.referenced_from_header_files == other.referenced_from_header_files
            && self.referenced_from_source_files == other.referenced_from_source_files
            && self.file_is_header == other.file_is_header;
    }
}

impl MatchingFuncs for FileStructure {
    fn get_matching_funcs(
        &self,
        _position: Position,
        results: &mut Vec<Rc<RefCell<FuncStructure>>>,
    ) {
        todo!()
    }
}

impl MainDeclPosition for FileStructure {
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
        if !self.file_is_header {
            (Some(self.id), None, None)
        } else {
            (None, Some(self.id), None)
        }
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

impl File for FileStructure {
    fn get_includes(&self) -> Vec<Rc<dyn File>> {
        todo!()
    }

    fn get_last_analyzed(&self) -> i64 {
        self.last_analyzed
    }

    fn set_last_analyzed(&mut self, last_analyzed: i64) {
        if self.file_is_header {
            self.set_last_analyzed_inner_hpp(last_analyzed);
        } else {
            self.set_last_analyzed_inner_cpp(last_analyzed);
        }
        self.last_analyzed = last_analyzed;
    }

    fn just_analyzed(&mut self) {
        self.set_last_analyzed(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        );
    }
}

impl FileStructure {
    pub fn new(
        id: u64,
        db_connection: Option<DatabaseSqliteInternal>,
        name: String,
        last_analyzed: i64,
        file_is_header: bool,
    ) -> Self {
        let mut file_structure = Self {
            id,
            db_connection,
            name,
            last_analyzed,
            classes: Vec::new(),
            func_decls: Vec::new(),
            func_impls: Vec::new(),
            virtual_func_impls: Vec::new(),
            referenced_from_header_files: Vec::new(),
            referenced_from_source_files: Vec::new(),
            file_is_header,
        };

        if file_structure.db_connection.is_some() {
            if file_structure.file_is_header {
                file_structure.referenced_from_header_files =
                    file_structure.read_referenced_from_header_files();
                file_structure.referenced_from_source_files =
                    file_structure.read_referenced_from_source_files();
            }

            file_structure.classes = CppClass::get_cpp_classes(
                &file_structure.db_connection.as_ref().unwrap(),
                file_structure.get_main_decl_position_id(),
            );

            file_structure.func_decls = FuncStructure::get_func_decls(
                file_structure.db_connection.as_ref().unwrap(),
                file_structure.get_main_decl_position_id(),
            );
            file_structure.func_impls = FuncStructure::get_func_impls(
                file_structure.db_connection.as_ref().unwrap(),
                file_structure.get_main_decl_position_id(),
            );
            file_structure.virtual_func_impls = FuncStructure::get_virtual_func_impls(
                file_structure.db_connection.as_ref().unwrap(),
                file_structure.get_main_decl_position_id(),
            );
        }

        file_structure
    }

    pub fn get_referenced_from_header_files(&self) -> &Vec<String> {
        &self.referenced_from_header_files
    }

    pub fn add_referenced_from_header_file(&mut self, file: &Rc<RefCell<FileStructure>>) {
        if self
            .referenced_from_header_files
            .contains(&file.borrow().name)
        {
            return;
        }

        self.add_referenced_from_header_file_inner(file);

        self.referenced_from_header_files
            .push(file.borrow().name.clone());
    }

    pub fn get_referenced_from_source_files(&self) -> Vec<String> {
        self.referenced_from_source_files.clone()
    }

    pub fn add_referenced_from_source_file(&mut self, file: &Rc<RefCell<FileStructure>>) {
        if self
            .referenced_from_source_files
            .contains(&file.borrow().get_name().to_string())
        {
            return;
        }

        self.add_referenced_from_source_file_inner(file);

        self.referenced_from_source_files
            .push(file.borrow().get_name().to_string());
    }

    pub fn get_referenced_from_header_files_mut(&mut self) -> &mut Vec<String> {
        &mut self.referenced_from_header_files
    }
}
