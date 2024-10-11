use super::range::Range;

pub struct VirtualFuncCreationArgs {
    pub name: String,
    pub qualified_name: String,
    pub base_qualified_name: String,
    pub qualified_type: String,
    pub range: Range,
}
