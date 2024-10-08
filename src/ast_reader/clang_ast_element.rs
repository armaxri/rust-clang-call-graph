use std::collections::VecDeque;
use std::rc::Rc;

use super::super::location::range::Range;

pub struct ClangAstElement {
    pub element_type: String,
    pub element_id: u64,
    pub file: Rc<String>,
    pub range: Range,
    pub inner: VecDeque<ClangAstElement>,
    pub attributes: String,
}

impl ClangAstElement {
    pub fn new<'a>(
        element_type: String,
        element_id: u64,
        file: Rc<String>,
        range: Range,
        attributes: String,
    ) -> ClangAstElement {
        ClangAstElement {
            element_type,
            element_id,
            file,
            range,
            inner: VecDeque::new(),
            attributes,
        }
    }
}
