use std::collections::VecDeque;

use super::clang_ast_element::ClangAstElement;

pub mod clang_ast_parser_impl;

pub trait ClangAstParser {
    fn parse_ast(&mut self) -> bool;

    fn get_ast(&self) -> &Box<VecDeque<Box<ClangAstElement>>>;
}
