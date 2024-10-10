use serde::Deserialize;
use serde::Serialize;

use super::func_call::FuncCall;
use super::range::Range;
use super::virtual_func_call::VirtualFuncCall;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct VirtualFuncImpl {
    pub name: String,
    pub qualified_name: String,
    pub base_qualified_name: String,
    pub qual_type: String,
    pub range: Range,
    pub func_calls: Vec<FuncCall>,
    pub virtual_func_calls: Vec<VirtualFuncCall>,
}
