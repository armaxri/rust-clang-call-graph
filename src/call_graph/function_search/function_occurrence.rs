use std::{cell::RefCell, rc::Rc};

use crate::call_graph::{
    data_structure::{
        func_call::FuncCall, func_decl::FuncDecl, func_impl::FuncImpl,
        virtual_func_call::VirtualFuncCall, virtual_func_decl::VirtualFuncDecl,
        virtual_func_impl::VirtualFuncImpl,
    },
    database::database_sqlite_internal::DatabaseSqliteInternal,
};

pub struct FunctionOccurrence {
    pub file: String,
    pub db_connection: DatabaseSqliteInternal,
    pub func_decl: Rc<RefCell<FuncDecl>>,
    pub func_impl: Rc<RefCell<FuncImpl>>,
    pub func_call: Rc<RefCell<FuncCall>>,
    pub virtual_func_decl: Rc<RefCell<VirtualFuncDecl>>,
    pub virtual_func_impl: Rc<RefCell<VirtualFuncImpl>>,
    pub virtual_func_call: Rc<RefCell<VirtualFuncCall>>,
}
