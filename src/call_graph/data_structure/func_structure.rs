use std::cell::RefCell;
use std::rc::Rc;

use serde::Deserialize;
use serde::Serialize;

use crate::call_graph::database::database_sqlite_internal::DatabaseSqliteInternal;
use crate::location::position::Position;
use crate::location::range::Range;

use super::helper::func_creation_args::FuncCreationArgs;
use super::helper::virtual_func_creation_args::VirtualFuncCreationArgs;
use super::FuncBasics;
use super::FuncImplBasics;
use super::MatchingFuncs;
use super::VirtualFuncBasics;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuncMentionType {
    FuncDecl = 0,
    FuncImpl = 1,
    FuncCall = 2,
    VirtualFuncDecl = 3,
    VirtualFuncImpl = 4,
    VirtualFuncCall = 5,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq)]
pub struct FuncStructure {
    id: u64,
    #[serde(skip)]
    db_connection: Option<DatabaseSqliteInternal>,

    name: String,
    qualified_name: String,
    base_qualified_name: Option<String>,
    qual_type: String,
    range: Range,
    func_calls: Vec<Rc<RefCell<FuncStructure>>>,
    virtual_func_calls: Vec<Rc<RefCell<FuncStructure>>>,

    #[serde(skip)]
    func_type: Option<FuncMentionType>,
}

impl PartialEq for FuncStructure {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id
            && self.name == other.name
            && self.qualified_name == other.qualified_name
            && self.base_qualified_name == other.base_qualified_name
            && self.qual_type == other.qual_type
            && self.range == other.range
            && self.func_calls == other.func_calls
            && self.virtual_func_calls == other.virtual_func_calls;
    }
}

impl FuncStructure {
    pub fn new(
        id: u64,
        db_connection: Option<DatabaseSqliteInternal>,
        name: String,
        qualified_name: String,
        base_qualified_name: Option<String>,
        qual_type: String,
        range: Range,
        func_type: Option<FuncMentionType>,
    ) -> Self {
        let mut new_func = Self {
            id,
            db_connection,
            name,
            qualified_name,
            base_qualified_name,
            qual_type,
            range,
            func_calls: Vec::new(),
            virtual_func_calls: Vec::new(),
            func_type,
        };

        if new_func.db_connection.is_some()
            && (new_func.func_type == Some(FuncMentionType::FuncImpl)
                || new_func.func_type == Some(FuncMentionType::VirtualFuncImpl))
        {
            new_func.func_calls = Self::get_func_calls_from_id(
                &new_func.db_connection.as_ref().unwrap(),
                new_func.get_id_func_impls(),
            );
            new_func.virtual_func_calls = Self::get_virtual_func_calls_from_id(
                &new_func.db_connection.as_ref().unwrap(),
                new_func.get_id_func_impls(),
            );
        }

        new_func
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    fn get_id_func_impls(&self) -> (Option<u64>, Option<u64>) {
        if self.func_type == Some(FuncMentionType::FuncImpl) {
            (Some(self.id), None)
        } else if self.func_type == Some(FuncMentionType::VirtualFuncImpl) {
            (None, Some(self.id))
        } else {
            (None, None)
        }
    }

    pub fn get_database_connection(&self) -> Option<DatabaseSqliteInternal> {
        self.db_connection.clone()
    }
}

impl FuncBasics for FuncStructure {
    fn convert_func2func_creation_args4call(&self, call_range: &Range) -> FuncCreationArgs {
        FuncCreationArgs {
            name: self.get_name().to_string(),
            qualified_name: self.get_qualified_name().to_string(),
            qualified_type: self.get_qual_type().to_string(),
            range: call_range.clone(),
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_qualified_name(&self) -> &str {
        &self.qualified_name
    }

    fn get_qual_type(&self) -> &str {
        &self.qual_type
    }

    fn get_range(&self) -> &Range {
        &self.range
    }

    fn get_func_type(&self) -> Option<FuncMentionType> {
        self.func_type.clone()
    }

    fn matches_position(&self, position: &Position) -> bool {
        self.get_range().is_position_within_range(&position)
    }

    fn equals_func_creation_args(&self, func_creation_args: &FuncCreationArgs) -> bool {
        self.get_name() == func_creation_args.name
            && self.get_qualified_name() == func_creation_args.qualified_name
            && self.get_qual_type() == func_creation_args.qualified_type
            && self.get_range() == &func_creation_args.range
    }
}

impl VirtualFuncBasics for FuncStructure {
    fn convert_virtual_func2virtual_func_creation_args4call(
        &self,
        call_range: &Range,
    ) -> VirtualFuncCreationArgs {
        VirtualFuncCreationArgs {
            name: self.get_name().to_string(),
            qualified_name: self.get_qualified_name().to_string(),
            base_qualified_name: self.get_base_qualified_name().to_string(),
            qualified_type: self.get_qual_type().to_string(),
            range: call_range.clone(),
        }
    }

    fn get_base_qualified_name(&self) -> &str {
        self.base_qualified_name.as_ref().unwrap()
    }

    fn equals_virtual_func_creation_args(
        &self,
        func_creation_args: &VirtualFuncCreationArgs,
    ) -> bool {
        self.get_name() == func_creation_args.name
            && self.get_qualified_name() == func_creation_args.qualified_name
            && self.get_base_qualified_name() == func_creation_args.base_qualified_name
            && self.get_qual_type() == func_creation_args.qualified_type
            && self.get_range() == &func_creation_args.range
    }
}

impl FuncImplBasics for FuncStructure {
    fn get_func_calls(&mut self) -> &mut Vec<Rc<RefCell<FuncStructure>>> {
        &mut self.func_calls
    }
    fn add_func_call(&mut self, func_call: &FuncCreationArgs) -> Rc<RefCell<FuncStructure>> {
        let new_func_call = Rc::new(RefCell::new(FuncStructure::create_func_call(
            self.db_connection.as_ref().unwrap(),
            &func_call,
            self.get_id_func_impls(),
        )));

        self.get_func_calls().push(new_func_call);

        self.get_func_calls().last().unwrap().clone()
    }
    fn get_or_add_func_call(&mut self, func_call: &FuncCreationArgs) -> Rc<RefCell<FuncStructure>> {
        if self
            .get_func_calls()
            .iter()
            .any(|c| c.borrow().equals_func_creation_args(&func_call))
        {
            self.get_func_calls()
                .iter()
                .find(|c| c.borrow().equals_func_creation_args(&func_call))
                .unwrap()
                .clone()
        } else {
            self.add_func_call(func_call)
        }
    }

    fn get_virtual_func_calls(&mut self) -> &mut Vec<Rc<RefCell<FuncStructure>>> {
        &mut self.virtual_func_calls
    }
    fn add_virtual_func_call(
        &mut self,
        virtual_func_call: &VirtualFuncCreationArgs,
    ) -> Rc<RefCell<FuncStructure>> {
        let new_virtual_func_call = Rc::new(RefCell::new(FuncStructure::create_virtual_func_call(
            self.db_connection.as_ref().unwrap(),
            &virtual_func_call,
            self.get_id_func_impls(),
        )));

        self.get_virtual_func_calls().push(new_virtual_func_call);

        self.get_virtual_func_calls().last().unwrap().clone()
    }
    fn get_or_add_virtual_func_call(
        &mut self,
        virtual_func_call: &VirtualFuncCreationArgs,
    ) -> Rc<RefCell<FuncStructure>> {
        if self.get_virtual_func_calls().iter().any(|c| {
            c.borrow()
                .equals_virtual_func_creation_args(&virtual_func_call)
        }) {
            self.get_virtual_func_calls()
                .iter()
                .find(|c| {
                    c.borrow()
                        .equals_virtual_func_creation_args(&virtual_func_call)
                })
                .unwrap()
                .clone()
        } else {
            self.add_virtual_func_call(virtual_func_call)
        }
    }
}

impl MatchingFuncs for FuncStructure {
    fn get_matching_funcs(
        &self,
        position: &Position,
        results: &mut Vec<Rc<RefCell<FuncStructure>>>,
    ) {
        for func_call in self.func_calls.iter() {
            if func_call.borrow().matches_position(position) {
                results.push(func_call.clone());
            }
        }
        for virtual_func_call in self.virtual_func_calls.iter() {
            if virtual_func_call.borrow().matches_position(position) {
                results.push(virtual_func_call.clone());
            }
        }
    }
}
