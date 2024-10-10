use serde::Deserialize;
use serde::Serialize;

use super::location::Location;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Range {
    pub start: Location,
    pub end: Location,
}
