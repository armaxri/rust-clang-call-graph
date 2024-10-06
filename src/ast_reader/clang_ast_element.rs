use std::collections::VecDeque;

use super::super::location::range::Range;
use super::clang_ast_element_type::ClangAstElementType;

pub struct ClangAstElement {
    pub element_type: ClangAstElementType,
    pub element_id: u64,
    pub file: Box<String>,
    pub range: Range,
    pub inner: Box<VecDeque<Box<ClangAstElement>>>,
    pub attributes: String,
}

impl ClangAstElement {
    pub fn new<'a>(
        element_type: ClangAstElementType,
        element_id: u64,
        file: Box<String>,
        range: Range,
        attributes: String,
    ) -> ClangAstElement {
        ClangAstElement {
            element_type,
            element_id,
            file,
            range,
            inner: Box::new(VecDeque::new()),
            attributes,
        }
    }
}
