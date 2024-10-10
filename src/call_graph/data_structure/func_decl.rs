use serde::Deserialize;
use serde::Serialize;

use super::range::Range;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct FuncDecl {
    pub name: String,
    pub qualified_name: String,
    pub qual_type: String,
    pub range: Range,
}
