use crate::location::range::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncCreationArgs {
    pub name: String,
    pub qualified_name: String,
    pub base_qualified_name: Option<String>,
    pub qualified_type: String,
    pub range: Range,
}

impl FuncCreationArgs {
    pub fn new(
        name: &str,
        qualified_name: &str,
        base_qualified_name: Option<String>,
        qualified_type: &str,
        range: Range,
    ) -> Self {
        Self {
            name: name.to_string(),
            qualified_name: qualified_name.to_string(),
            base_qualified_name: base_qualified_name,
            qualified_type: qualified_type.to_string(),
            range,
        }
    }

    pub fn is_virtual(&self) -> bool {
        self.base_qualified_name.is_some()
    }
}
