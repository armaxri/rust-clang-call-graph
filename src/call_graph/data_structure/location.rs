use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}
