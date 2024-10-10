use serde::Deserialize;
use serde::Serialize;

use super::cpp_class::CppClass;
use super::func_decl::FuncDecl;
use super::func_impl::FuncImpl;
use super::virtual_func_impl::VirtualFuncImpl;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct HppFile {
    pub name: String,
    pub last_analyzed: u64,
    pub classes: Vec<CppClass>,
    pub func_decls: Vec<FuncDecl>,
    pub func_impls: Vec<FuncImpl>,
    pub virtual_func_impls: Vec<VirtualFuncImpl>,
    pub referenced_from_files: Vec<String>,
}
