use std::collections::VecDeque;

use super::super::super::location::position::Position;
use super::super::super::location::range::Range;
use super::super::super::process::Process;
use super::super::clang_ast_element::ClangAstElement;
use super::super::clang_ast_element_type::ClangAstElementType;

pub struct ClangAstParserImpl {
    process: Box<dyn Process>,
    ast: Box<VecDeque<Box<ClangAstElement>>>,
    files: Vec<Box<String>>,
    last_seen_line: u32,
}

impl<'a> ClangAstParserImpl {
    pub fn new(process: Box<dyn Process>) -> Self {
        let mut parser_impl = ClangAstParserImpl {
            process,
            ast: Box::new(VecDeque::new()),
            files: Vec::new(),
            last_seen_line: 0,
        };

        parser_impl.files.push(Box::new("".to_string()));

        parser_impl
    }
}

impl<'a> super::ClangAstParser for ClangAstParserImpl {
    fn parse_ast(&mut self) -> bool {
        if self.process.process() == false
            || self.process.has_next_line() == false
            || !self
                .process
                .get_next_line()
                .starts_with("TranslationUnitDecl")
        {
            // TODO: Add error handling here.
            return false;
        }

        if self.process.has_next_line() {
            let mut ast = std::mem::take(&mut self.ast);
            self.parse_ast_line(1, &mut ast);
            self.ast = ast;
        }

        return true;
    }

    fn get_ast(
        &self,
    ) -> &Box<VecDeque<Box<crate::ast_reader::clang_ast_element::ClangAstElement>>> {
        return &self.ast;
    }
}

impl<'a> ClangAstParserImpl {
    fn parse_ast_line(
        &mut self,
        current_parse_depth: usize,
        parent_vec: &mut Box<VecDeque<Box<ClangAstElement>>>,
    ) {
        while self.process.has_next_line() {
            let line = self.process.fetch_next_line();
            let parsing_start_depth = get_string_element_start(&line);

            if parsing_start_depth < current_parse_depth {
                return;
            } else if parsing_start_depth == current_parse_depth {
                self.process.get_next_line();
                let (_, ast_element) = self.get_ast_element_with_depth(&line);
                if let Some(element) = ast_element {
                    parent_vec.push_back(element);
                }
            } else {
                let last_element = parent_vec.back_mut().unwrap();
                let mut last_element_inner = &mut last_element.inner;
                self.parse_ast_line(current_parse_depth + 2, &mut last_element_inner);
            }
        }
    }

    fn parse_ast_element(&mut self, line: &'a str) -> Option<Box<ClangAstElement>> {
        let mut parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 3 {
            return None;
        }

        // This is a very rare case only seen with Overrides so far.
        if parts[0].ends_with(":") {
            let decl_type =
                ClangAstElementType::from_str(&parts[0][..parts[0].len() - 1].to_string());
            if parts.len() > 1 {
                parts.remove(0);
                parts.remove(0);
            }
            parts.pop();

            let remaining_parts = parts.join(" ");

            let file = self.files.last().unwrap();
            return Some(Box::new(ClangAstElement::new(
                decl_type,
                0,
                file.clone(),
                Range::new(0, 0, 0, 0),
                remaining_parts,
            )));
        }

        let decl_type = ClangAstElementType::from_str(&parts[0].to_string());
        parts.remove(0);

        let id = if parts[0].starts_with("0x") {
            if let Ok(hex_value) = u64::from_str_radix(&parts[0][2..], 16) {
                parts.remove(0);
                hex_value
            } else {
                0
            }
        } else {
            0
        };

        let range = self.get_range(&mut parts);
        let remaining_parts = parts.join(" ");

        let file = self.files.last().unwrap();
        return Some(Box::new(ClangAstElement::new(
            decl_type,
            id,
            file.clone(),
            range,
            remaining_parts,
        )));
    }

    fn get_ast_element_with_depth(
        &mut self,
        line: &String,
    ) -> (usize, Option<Box<ClangAstElement>>) {
        let ast_element_depth = get_string_element_start(&line);
        let ast_element = self.parse_ast_element(&line[(ast_element_depth + 1)..]);

        (ast_element_depth, ast_element)
    }

    fn get_range(&mut self, elements: &mut Vec<&str>) -> Range {
        if elements.len() < 1 || elements[0].starts_with("<<") {
            return Range::new(0, 0, 0, 0);
        }

        if elements[0].starts_with("<col:") && elements[0].ends_with(">") {
            if let Some(col_str) = elements[0]
                .strip_prefix("<col:")
                .and_then(|s| s.strip_suffix('>'))
            {
                if let Ok(col) = col_str.parse::<u32>() {
                    elements.remove(0);
                    return Range::new(self.last_seen_line, col, self.last_seen_line, col);
                }
            }
        }

        if elements.len() < 2 {
            return Range::new(0, 0, 0, 0);
        }

        if elements[0].starts_with("<") && elements[0].ends_with(",") && elements[1].ends_with(">")
        {
            let start = self.get_first_range_element(&elements[0]);
            let end = self.get_second_range_element(&elements[1]);
            elements.remove(0);
            elements.remove(0);
            return Range::new(start.line, start.column, end.line, end.column);
        }

        Range::new(0, 0, 0, 0)
    }

    fn get_first_range_element(&mut self, element: &str) -> Position {
        if element.starts_with("<col:") && element.ends_with(",") {
            if let Some(col_str) = element
                .strip_prefix("<col:")
                .and_then(|s| s.strip_suffix(','))
            {
                if let Ok(col) = col_str.parse::<u32>() {
                    return Position::new(self.last_seen_line, col);
                }
            }
        }

        if element.starts_with("<line:") && element.ends_with(",") {
            let parts: Vec<&str> = element[6..element.len() - 1].split(':').collect();
            if parts.len() == 2 {
                if let Ok(line) = parts[0].parse::<u32>() {
                    if let Ok(col) = parts[1].parse::<u32>() {
                        self.last_seen_line = line;
                        return Position::new(line, col);
                    }
                }
            }
        }

        if element.starts_with("<") && element.ends_with(",") {
            let parts: Vec<&str> = element[1..element.len() - 1].split(':').collect();
            if parts.len() == 3 {
                self.files.push(Box::new(parts[0].to_string()));

                if let Ok(line) = parts[1].parse::<u32>() {
                    if let Ok(col) = parts[2].parse::<u32>() {
                        self.last_seen_line = line;
                        return Position::new(line, col);
                    }
                }
            }
        }

        Position::new(0, 0)
    }

    fn get_second_range_element(&mut self, element: &str) -> Position {
        if element.starts_with("line:") && element.ends_with(">") {
            let parts: Vec<&str> = element[0..element.len() - 1].split(':').collect();
            if parts.len() == 3 {
                if let Ok(line) = parts[1].parse::<u32>() {
                    // The second part should not store the line number for reuse.
                    // self.last_seen_line = line;
                    if let Ok(col) = parts[2].parse::<u32>() {
                        return Position::new(line, col);
                    }
                }
            }
        }

        if element.starts_with("col:") && element.ends_with(">") {
            if let Some(col_str) = element
                .strip_prefix("col:")
                .and_then(|s| s.strip_suffix('>'))
            {
                if let Ok(col) = col_str.parse::<u32>() {
                    return Position::new(self.last_seen_line, col);
                }
            }
        }

        Position::new(0, 0)
    }
}

fn get_string_element_start(line: &String) -> usize {
    match line.find('-') {
        Some(index) => return index,
        None => return 0,
    };
}

#[cfg(test)]
mod tests {
    use crate::ast_reader::clang_ast_parser::ClangAstParser;

    use super::*;
    use crate::process::dummy_process::DummyProcess;

    #[test]
    fn test_parse_ast_line() {
        let mut process = DummyProcess::new();
        process.add_line(
            "TranslationUnitDecl 0x11d848e08 <<invalid sloc>> <invalid sloc>".to_string(),
        );
        process.add_line(
            "|-TypedefDecl 0x11d849cf0 <<invalid sloc>> <invalid sloc> implicit __int128_t '__int128'"
                .to_string(),
        );
        process.add_line("| `-BuiltinType 0x11d8493d0 '__int128'".to_string());
        process.add_line(
            "`-TypedefDecl 0x11d849d60 <<invalid sloc>> <invalid sloc> implicit __uint128_t 'unsigned __int128''"
                .to_string(),
        );
        process.add_line("  `-BuiltinType 0x11d8493f0 'unsigned __int128'".to_string());
        let mut parser = ClangAstParserImpl::new(Box::new(process));
        assert_eq!(parser.parse_ast(), true);
        assert_eq!(parser.get_ast().len(), 2);
        assert_eq!(parser.get_ast()[0].inner.len(), 1);
        assert_eq!(parser.get_ast()[1].inner.len(), 1);
    }

    #[test]
    fn test_get_second_range_element_line() {
        let process = DummyProcess::new();
        let mut parser = ClangAstParserImpl::new(Box::new(process));
        parser.last_seen_line = 5;

        let element = parser.get_second_range_element("line:4:3>");
        assert_eq!(element.line, 4);
        assert_eq!(element.column, 3);
        assert_eq!(parser.files.len(), 1);
        assert_eq!(parser.files.last().unwrap().as_ref(), "");
        assert_eq!(parser.last_seen_line, 5);
    }

    #[test]
    fn test_get_second_range_element_col() {
        let process = DummyProcess::new();
        let mut parser = ClangAstParserImpl::new(Box::new(process));
        parser.last_seen_line = 5;

        let element = parser.get_second_range_element("col:3>");
        assert_eq!(element.line, 5);
        assert_eq!(element.column, 3);
        assert_eq!(parser.files.len(), 1);
        assert_eq!(parser.files.last().unwrap().as_ref(), "");
    }

    #[test]
    fn test_get_first_range_element_file() {
        let process = DummyProcess::new();
        let mut parser = ClangAstParserImpl::new(Box::new(process));
        parser.last_seen_line = 5;

        let element = parser.get_first_range_element("</home/user/foo/bar.c:7:8,");
        assert_eq!(element.line, 7);
        assert_eq!(element.column, 8);
        assert_eq!(parser.files.len(), 2);
        assert_eq!(
            parser.files.last().unwrap().as_ref(),
            "/home/user/foo/bar.c"
        );
        assert_eq!(parser.last_seen_line, 7);
    }

    #[test]
    fn test_get_first_range_element_line() {
        let process = DummyProcess::new();
        let mut parser = ClangAstParserImpl::new(Box::new(process));
        parser.last_seen_line = 5;

        let element = parser.get_first_range_element("<line:4:3,");
        assert_eq!(element.line, 4);
        assert_eq!(element.column, 3);
        assert_eq!(parser.files.len(), 1);
        assert_eq!(parser.files.last().unwrap().as_ref(), "");
        assert_eq!(parser.last_seen_line, 4);
    }

    #[test]
    fn test_get_first_range_element_col() {
        let process = DummyProcess::new();
        let mut parser = ClangAstParserImpl::new(Box::new(process));
        parser.last_seen_line = 5;

        let element = parser.get_first_range_element("<col:3,");
        assert_eq!(element.line, 5);
        assert_eq!(element.column, 3);
        assert_eq!(parser.files.len(), 1);
        assert_eq!(parser.files.last().unwrap().as_ref(), "");
    }

    #[test]
    fn parse_ast_element_simple_structure() {
        let process = DummyProcess::new();
        let mut parser = ClangAstParserImpl::new(Box::new(process));

        let mut element = parser
            .parse_ast_element("TranslationUnitDecl 0x7f8b1b0b3e00 <<invalid sloc>> <invalid sloc>")
            .unwrap();
        assert_eq!(
            element.element_type,
            ClangAstElementType::TranslationUnitDecl
        );
        assert_eq!(element.element_id, 0x7f8b1b0b3e00);
        assert_eq!(element.file.as_ref(), "");
        assert_eq!(element.range.start.line, 0);
        assert_eq!(element.range.start.column, 0);
        assert_eq!(element.range.end.line, 0);
        assert_eq!(element.range.end.column, 0);
        assert_eq!(element.inner.len(), 0);
        assert_eq!(element.attributes, "<<invalid sloc>> <invalid sloc>");

        element = parser
            .parse_ast_element("BuiltinType 0x11d849790 '__clang_svint32x2_t'")
            .unwrap();
        assert_eq!(element.element_type, ClangAstElementType::BuiltinType);
        assert_eq!(element.element_id, 0x11d849790);
        assert_eq!(element.file.as_ref(), "");
        assert_eq!(element.range.start.line, 0);
        assert_eq!(element.range.start.column, 0);
        assert_eq!(element.range.end.line, 0);
        assert_eq!(element.range.end.column, 0);
        assert_eq!(element.inner.len(), 0);
        assert_eq!(element.attributes, "'__clang_svint32x2_t'");
    }

    #[test]
    fn parse_ast_element_structures_with_range() {
        let process = DummyProcess::new();
        let mut parser = ClangAstParserImpl::new(Box::new(process));

        let mut element = parser
            .parse_ast_element("TranslationUnitDecl 0x7f8b1b0b3e00 <<invalid sloc>> <invalid sloc>")
            .unwrap();
        assert_eq!(
            element.element_type,
            ClangAstElementType::TranslationUnitDecl
        );
        assert_eq!(element.element_id, 0x7f8b1b0b3e00);
        assert_eq!(element.file.as_ref(), "");
        assert_eq!(element.range.start.line, 0);
        assert_eq!(element.range.start.column, 0);
        assert_eq!(element.range.end.line, 0);
        assert_eq!(element.range.end.column, 0);
        assert_eq!(element.inner.len(), 0);
        assert_eq!(element.attributes, "<<invalid sloc>> <invalid sloc>");

        element = parser
            .parse_ast_element("FunctionDecl 0x11d905360 </Users/xxx/git/vscode-clang-call-graph/src/test/backendSuite/walkerTests/actualTests/cStyleTests/declInHeaderAndTwoCpps/header.h:1:1, col:27> col:5 used add 'int (int, int)'")
            .unwrap();
        assert_eq!(element.element_type, ClangAstElementType::FunctionDecl);
        assert_eq!(element.element_id, 0x11d905360);
        assert_eq!(element.file.as_ref(), "/Users/xxx/git/vscode-clang-call-graph/src/test/backendSuite/walkerTests/actualTests/cStyleTests/declInHeaderAndTwoCpps/header.h");
        assert_eq!(element.range.start.line, 1);
        assert_eq!(element.range.start.column, 1);
        assert_eq!(element.range.end.line, 1);
        assert_eq!(element.range.end.column, 27);
        assert_eq!(element.inner.len(), 0);
        assert_eq!(element.attributes, "col:5 used add 'int (int, int)'");

        element = parser
            .parse_ast_element("ParmVarDecl 0x11d905208 <col:9, col:13> col:13 val1 'int'")
            .unwrap();
        assert_eq!(element.element_type, ClangAstElementType::ParmVarDecl);
        assert_eq!(element.element_id, 0x11d905208);
        assert_eq!(element.file.as_ref(), "/Users/xxx/git/vscode-clang-call-graph/src/test/backendSuite/walkerTests/actualTests/cStyleTests/declInHeaderAndTwoCpps/header.h");
        assert_eq!(element.range.start.line, 1);
        assert_eq!(element.range.start.column, 9);
        assert_eq!(element.range.end.line, 1);
        assert_eq!(element.range.end.column, 13);
        assert_eq!(element.inner.len(), 0);
        assert_eq!(element.attributes, "col:13 val1 'int'");

        element = parser
            .parse_ast_element("FunctionDecl 0x11d9056c0 </Users/xxx/git/vscode-clang-call-graph/src/test/backendSuite/walkerTests/actualTests/cStyleTests/declInHeaderAndTwoCpps/main.cpp:3:1, line:6:1> line:3:5 main 'int (int, char **)'")
            .unwrap();
        assert_eq!(element.element_type, ClangAstElementType::FunctionDecl);
        assert_eq!(element.element_id, 0x11d9056c0);
        assert_eq!(element.file.as_ref(), "/Users/xxx/git/vscode-clang-call-graph/src/test/backendSuite/walkerTests/actualTests/cStyleTests/declInHeaderAndTwoCpps/main.cpp");
        assert_eq!(element.range.start.line, 3);
        assert_eq!(element.range.start.column, 1);
        assert_eq!(element.range.end.line, 6);
        assert_eq!(element.range.end.column, 1);
        assert_eq!(element.inner.len(), 0);
        assert_eq!(element.attributes, "line:3:5 main 'int (int, char **)'");

        element = parser
            .parse_ast_element("ParmVarDecl 0x11d905478 <col:10, col:14> col:14 argc 'int'")
            .unwrap();
        assert_eq!(element.element_type, ClangAstElementType::ParmVarDecl);
        assert_eq!(element.element_id, 0x11d905478);
        assert_eq!(element.file.as_ref(), "/Users/xxx/git/vscode-clang-call-graph/src/test/backendSuite/walkerTests/actualTests/cStyleTests/declInHeaderAndTwoCpps/main.cpp");
        assert_eq!(element.range.start.line, 3);
        assert_eq!(element.range.start.column, 10);
        assert_eq!(element.range.end.line, 3);
        assert_eq!(element.range.end.column, 14);
        assert_eq!(element.inner.len(), 0);
        assert_eq!(element.attributes, "col:14 argc 'int'");
    }

    #[test]
    fn parse_ast_element_structures_without_id() {
        let process = DummyProcess::new();
        let mut parser = ClangAstParserImpl::new(Box::new(process));

        let  element = parser
            .parse_ast_element("CopyAssignment non_trivial has_const_param needs_overload_resolution implicit_has_const_param")
            .unwrap();
        assert_eq!(element.element_type, ClangAstElementType::CopyAssignment);
        assert_eq!(element.element_id, 0x0);
        assert_eq!(element.file.as_ref(), "");
        assert_eq!(element.range.start.line, 0);
        assert_eq!(element.range.start.column, 0);
        assert_eq!(element.range.end.line, 0);
        assert_eq!(element.range.end.column, 0);
        assert_eq!(element.inner.len(), 0);
        assert_eq!(
            element.attributes,
            "non_trivial has_const_param needs_overload_resolution implicit_has_const_param"
        );
    }

    #[test]
    fn parse_ast_element_packed_structures() {
        let process = DummyProcess::new();
        let mut parser = ClangAstParserImpl::new(Box::new(process));

        let element = parser
            .parse_ast_element(
                "Overrides: [ 0x14bf3dce8 __shared_count::~__shared_count 'void () noexcept' ]",
            )
            .unwrap();
        assert_eq!(element.element_type, ClangAstElementType::Overrides);
        assert_eq!(element.element_id, 0x0);
        assert_eq!(element.file.as_ref(), "");
        assert_eq!(element.range.start.line, 0);
        assert_eq!(element.range.start.column, 0);
        assert_eq!(element.range.end.line, 0);
        assert_eq!(element.range.end.column, 0);
        assert_eq!(element.inner.len(), 0);
        assert_eq!(
            element.attributes,
            "0x14bf3dce8 __shared_count::~__shared_count 'void () noexcept'"
        );
    }

    #[test]
    fn empty_structure() {
        let mut process = DummyProcess::new();
        process.add_line("TranslationUnitDecl".to_string());
        let mut parser = ClangAstParserImpl::new(Box::new(process));
        assert_eq!(parser.parse_ast(), true);
        assert_eq!(parser.get_ast().len(), 0);
    }

    #[test]
    fn wrong_file_output() {
        let mut process = DummyProcess::new();
        process.add_line("test".to_string());
        let mut parser = ClangAstParserImpl::new(Box::new(process));
        assert_eq!(parser.parse_ast(), false);
    }
}
