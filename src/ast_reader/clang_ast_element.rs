use std::collections::VecDeque;
use std::rc::Rc;

use super::super::location::range::Range;

pub struct ClangAstElement {
    pub element_type: String,
    pub element_id: usize,
    pub parent_element_id: usize,
    pub prev_element_id: usize,
    pub file: Rc<String>,
    pub range: Range,
    pub inner: VecDeque<ClangAstElement>,
    pub attributes: String,
}

impl ClangAstElement {
    pub fn new<'a>(
        element_type: String,
        element_id: usize,
        parent_element_id: usize,
        prev_element_id: usize,
        file: Rc<String>,
        range: Range,
        attributes: String,
    ) -> ClangAstElement {
        ClangAstElement {
            element_type,
            element_id,
            parent_element_id,
            prev_element_id,
            file,
            range,
            inner: VecDeque::new(),
            attributes,
        }
    }
}
