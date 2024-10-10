use serde::Deserialize;
use serde::Serialize;

use super::{cpp_file::CppFile, hpp_file::HppFile};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct DatabaseJson {
    pub cpp_files: Vec<CppFile>,
    pub hpp_files: Vec<HppFile>,
}
