use serde::Deserialize;
use serde::Serialize;

use super::range::Range;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct VirtualFuncDecl {
    pub name: String,
    pub qualified_name: String,
    pub base_qualified_name: String,
    pub qual_type: String,
    pub range: Range,
}
