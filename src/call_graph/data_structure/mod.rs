use std::{cell::RefCell, rc::Rc};

use cpp_class::CppClass;
use func_structure::{FuncMentionType, FuncStructure};
use helper::{
    func_creation_args::FuncCreationArgs, virtual_func_creation_args::VirtualFuncCreationArgs,
};

use crate::location::{position::Position, range::Range};

use super::database::database_sqlite_internal::DatabaseSqliteInternal;

pub mod cpp_class;
pub mod cpp_file;
pub mod file_structure;
pub mod func_call;
pub mod func_decl;
pub mod func_impl;
pub mod func_structure;
pub mod helper;
pub mod hpp_file;
pub mod virtual_func_call;
pub mod virtual_func_decl;
pub mod virtual_func_impl;

pub trait MatchingFuncs {
    fn get_matching_funcs(
        &self,
        position: &Position,
        results: &mut Vec<Rc<RefCell<FuncStructure>>>,
    );
}

pub trait FuncBasics {
    fn convert_func2func_creation_args4call(&self, call_range: &Range) -> FuncCreationArgs;

    fn get_name(&self) -> &str;
    fn get_qualified_name(&self) -> &str;
    fn get_qual_type(&self) -> &str;
    fn get_range(&self) -> &Range;

    fn get_func_type(&self) -> Option<FuncMentionType>;

    fn matches_position(&self, position: &Position) -> bool;

    fn equals_func_creation_args(&self, func_creation_args: &FuncCreationArgs) -> bool;
}

pub trait VirtualFuncBasics: FuncBasics {
    fn convert_virtual_func2virtual_func_creation_args4call(
        &self,
        call_range: &Range,
    ) -> VirtualFuncCreationArgs;

    fn get_base_qualified_name(&self) -> &str;

    fn equals_virtual_func_creation_args(
        &self,
        func_creation_args: &VirtualFuncCreationArgs,
    ) -> bool;
}

pub trait FuncImplBasics: FuncBasics + MatchingFuncs {
    fn get_func_calls(&mut self) -> &mut Vec<Rc<RefCell<FuncStructure>>>;
    fn add_func_call(&mut self, func_call: &FuncCreationArgs) -> Rc<RefCell<FuncStructure>>;
    fn get_or_add_func_call(&mut self, func_call: &FuncCreationArgs) -> Rc<RefCell<FuncStructure>>;

    fn get_virtual_func_calls(&mut self) -> &mut Vec<Rc<RefCell<FuncStructure>>>;
    fn add_virtual_func_call(
        &mut self,
        virtual_func_call: &VirtualFuncCreationArgs,
    ) -> Rc<RefCell<FuncStructure>>;
    fn get_or_add_virtual_func_call(
        &mut self,
        virtual_func_call: &VirtualFuncCreationArgs,
    ) -> Rc<RefCell<FuncStructure>>;
}

pub trait InFile {
    fn get_file_id(&self) -> Option<u64>;
}

pub trait MainDeclPosition: MatchingFuncs {
    fn get_name(&self) -> &str;

    fn get_db_connection(&self) -> Option<DatabaseSqliteInternal>;
    fn get_id(&self) -> u64;
    fn get_main_decl_position_id(&self) -> (Option<u64>, Option<u64>, Option<u64>);

    fn get_classes(&mut self) -> &mut Vec<Rc<RefCell<CppClass>>>;
    fn add_class(&mut self, class_name: &str) -> Rc<RefCell<CppClass>> {
        let new_class = Rc::new(RefCell::new(CppClass::create_cpp_class(
            &self.get_db_connection().unwrap(),
            class_name,
            self.get_main_decl_position_id(),
        )));
        self.get_classes().push(new_class);
        self.get_classes().last_mut().unwrap().clone()
    }
    fn get_or_add_class(&mut self, class_name: &str) -> Rc<RefCell<CppClass>> {
        // TODO: This double search is really necessary? How to deal with the ownership here?
        if self
            .get_classes()
            .iter()
            .any(|c| c.borrow().get_name() == class_name)
        {
            self.get_classes()
                .iter_mut()
                .find(|c| c.borrow().get_name() == class_name)
                .unwrap()
                .clone()
        } else {
            self.add_class(class_name)
        }
    }

    fn get_func_decls(&mut self) -> &mut Vec<Rc<RefCell<FuncStructure>>>;
    fn add_func_decl(&mut self, creation_args: FuncCreationArgs) -> Rc<RefCell<FuncStructure>> {
        let new_func_decl = Rc::new(RefCell::new(FuncStructure::create_func_decl(
            &self.get_db_connection().unwrap(),
            &creation_args,
            self.get_main_decl_position_id(),
        )));
        self.get_func_decls().push(new_func_decl);
        self.get_func_decls().last_mut().unwrap().clone()
    }
    fn get_or_add_func_decl(
        &mut self,
        creation_args: FuncCreationArgs,
    ) -> Rc<RefCell<FuncStructure>> {
        if self
            .get_func_decls()
            .iter()
            .any(|c| c.borrow().equals_func_creation_args(&creation_args))
        {
            self.get_func_decls()
                .iter_mut()
                .find(|c| c.borrow().equals_func_creation_args(&creation_args))
                .unwrap()
                .clone()
        } else {
            self.add_func_decl(creation_args)
        }
    }

    fn get_func_impls(&mut self) -> &mut Vec<Rc<RefCell<FuncStructure>>>;
    fn add_func_impl(&mut self, creation_args: FuncCreationArgs) -> Rc<RefCell<FuncStructure>> {
        let new_func_impl = Rc::new(RefCell::new(FuncStructure::create_func_impl(
            &self.get_db_connection().unwrap(),
            &creation_args,
            self.get_main_decl_position_id(),
        )));
        self.get_func_impls().push(new_func_impl);
        self.get_func_impls().last_mut().unwrap().clone()
    }
    fn get_or_add_func_impl(
        &mut self,
        creation_args: FuncCreationArgs,
    ) -> Rc<RefCell<FuncStructure>> {
        if self
            .get_func_impls()
            .iter()
            .any(|c| c.borrow().equals_func_creation_args(&creation_args))
        {
            self.get_func_impls()
                .iter_mut()
                .find(|c| c.borrow().equals_func_creation_args(&creation_args))
                .unwrap()
                .clone()
        } else {
            self.add_func_impl(creation_args)
        }
    }

    fn get_virtual_func_impls(&mut self) -> &mut Vec<Rc<RefCell<FuncStructure>>>;
    fn add_virtual_func_impl(
        &mut self,
        creation_args: VirtualFuncCreationArgs,
    ) -> Rc<RefCell<FuncStructure>> {
        let new_virtual_func_impl = Rc::new(RefCell::new(FuncStructure::create_virtual_func_impl(
            &self.get_db_connection().unwrap(),
            &creation_args,
            self.get_main_decl_position_id(),
        )));
        self.get_virtual_func_impls().push(new_virtual_func_impl);
        self.get_virtual_func_impls().last_mut().unwrap().clone()
    }
    fn get_or_add_virtual_func_impl(
        &mut self,
        creation_args: VirtualFuncCreationArgs,
    ) -> Rc<RefCell<FuncStructure>> {
        if self
            .get_virtual_func_impls()
            .iter()
            .any(|c| c.borrow().equals_virtual_func_creation_args(&creation_args))
        {
            self.get_virtual_func_impls()
                .iter_mut()
                .find(|c| c.borrow().equals_virtual_func_creation_args(&creation_args))
                .unwrap()
                .clone()
        } else {
            self.add_virtual_func_impl(creation_args)
        }
    }

    fn find_func_decl(&self, _func: &dyn FuncBasics) -> Option<Rc<RefCell<FuncStructure>>> {
        todo!()
    }
    fn find_virtual_func_impl(&self, _func: &dyn FuncBasics) -> Option<Rc<RefCell<FuncStructure>>> {
        todo!()
    }
}
