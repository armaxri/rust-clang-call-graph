use crate::location::range::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtualFuncCreationArgs {
    pub name: String,
    pub qualified_name: String,
    pub base_qualified_name: String,
    pub qualified_type: String,
    pub range: Range,
}

impl VirtualFuncCreationArgs {
    pub fn new(
        name: &str,
        qualified_name: &str,
        base_qualified_name: &str,
        qualified_type: &str,
        range: Range,
    ) -> Self {
        Self {
            name: name.to_string(),
            qualified_name: qualified_name.to_string(),
            base_qualified_name: base_qualified_name.to_string(),
            qualified_type: qualified_type.to_string(),
            range,
        }
    }
}
