use super::range::Range;

pub struct FuncCreationArgs {
    pub name: String,
    pub qualified_name: String,
    pub qualified_type: String,
    pub range: Range,
}

impl FuncCreationArgs {
    pub fn new(name: &str, qualified_name: &str, qualified_type: &str, range: Range) -> Self {
        Self {
            name: name.to_string(),
            qualified_name: qualified_name.to_string(),
            qualified_type: qualified_type.to_string(),
            range,
        }
    }
}
