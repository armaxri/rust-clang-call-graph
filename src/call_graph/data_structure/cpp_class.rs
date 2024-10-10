use serde::Deserialize;
use serde::Serialize;

use super::func_decl::FuncDecl;
use super::func_impl::FuncImpl;
use super::virtual_func_decl::VirtualFuncDecl;
use super::virtual_func_impl::VirtualFuncImpl;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct CppClass {
    pub name: String,
    pub parent_classes: Vec<String>,
    pub classes: Vec<CppClass>,
    pub func_decls: Vec<FuncDecl>,
    pub func_impls: Vec<FuncImpl>,
    pub virtual_func_decls: Vec<VirtualFuncDecl>,
    pub virtual_func_impls: Vec<VirtualFuncImpl>,
}
