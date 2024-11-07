use std::collections::VecDeque;
use std::rc::Rc;

use super::super::location::position::Position;
use super::super::location::range::Range;
use super::super::process::Process;
use super::clang_ast_element::ClangAstElement;

pub trait ClangAstParser {
    fn parse_ast(&mut self) -> Option<VecDeque<ClangAstElement>>;
}

pub struct ClangAstParserImpl {
    process: Box<dyn Process>,
    files: Vec<Rc<String>>,
    last_seen_line: usize,
}

impl ClangAstParserImpl {
    pub fn new(process: Box<dyn Process>) -> Self {
        let mut parser_impl = ClangAstParserImpl {
            process,
            files: Vec::new(),
            last_seen_line: 0,
        };

        parser_impl.files.push(Rc::new("".to_string()));

        parser_impl
    }
}

impl ClangAstParser for ClangAstParserImpl {
    fn parse_ast(&mut self) -> Option<VecDeque<ClangAstElement>> {
        if self.process.has_next_line() == false
            || !self
                .process
                .get_next_line()
                .starts_with("TranslationUnitDecl")
        {
            // TODO: Add error handling here.
            return None;
        }

        let mut ast: VecDeque<ClangAstElement> = VecDeque::new();

        self.last_seen_line = 0;
        self.files.clear();
        self.files.push(Rc::new("".to_string()));

        if self.process.has_next_line() {
            self.parse_ast_line(1, &mut ast);
        }

        return Some(ast);
    }
}

impl ClangAstParserImpl {
    fn parse_ast_line(
        &mut self,
        current_parse_depth: usize,
        parent_vec: &mut VecDeque<ClangAstElement>,
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

    fn parse_ast_element(&mut self, line: &str) -> Option<ClangAstElement> {
        let mut parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 0 {
            return None;
        }

        if parts.len() == 1 {
            let file = self.files.last().unwrap();
            return Some(ClangAstElement::new(
                parts[0].to_string(),
                0,
                0,
                0,
                Rc::clone(&file),
                Range::create(0, 0, 0, 0),
                "".to_string(),
            ));
        }

        // This is a very rare case only seen with Overrides so far.
        if parts[0].ends_with(":") {
            let decl_type = &parts[0][..parts[0].len() - 1];
            if parts.len() > 1 {
                parts.remove(0);
                parts.remove(0);
            }
            parts.pop();

            let remaining_parts = parts.join(" ");

            let file = self.files.last().unwrap();
            return Some(ClangAstElement::new(
                decl_type.to_string(),
                0,
                0,
                0,
                Rc::clone(&file),
                Range::create(0, 0, 0, 0),
                remaining_parts,
            ));
        }

        let decl_type = &parts[0].to_string();
        parts.remove(0);

        let id: usize = if parts[0].starts_with("0x") {
            if let Ok(hex_value) = usize::from_str_radix(&parts[0][2..], 16) {
                parts.remove(0);
                hex_value
            } else {
                0
            }
        } else {
            0
        };

        let mut parent_element_id: usize = 0;
        if parts[0] == "parent" {
            parts.remove(0);
            if parts[0].starts_with("0x") {
                if let Ok(hex_value) = usize::from_str_radix(&parts[0][2..], 16) {
                    parts.remove(0);
                    parent_element_id = hex_value as usize;
                }
            }
        }

        let mut prev_element_id: usize = 0;
        if parts[0] == "prev" {
            parts.remove(0);
            if parts[0].starts_with("0x") {
                if let Ok(hex_value) = usize::from_str_radix(&parts[0][2..], 16) {
                    parts.remove(0);
                    prev_element_id = hex_value as usize;
                }
            }
        }

        let range = self.get_range(&mut parts);

        drop_location(&mut parts);

        let remaining_parts = parts.join(" ");
        let file = self.files.last().unwrap();
        return Some(ClangAstElement::new(
            decl_type.to_string(),
            id,
            parent_element_id,
            prev_element_id,
            Rc::clone(&file),
            range,
            remaining_parts,
        ));
    }

    fn get_ast_element_with_depth(&mut self, line: &str) -> (usize, Option<ClangAstElement>) {
        let ast_element_depth = get_string_element_start(&line);
        let ast_element = self.parse_ast_element(&line[(ast_element_depth + 1)..]);

        (ast_element_depth, ast_element)
    }

    fn get_range(&mut self, elements: &mut Vec<&str>) -> Range {
        if elements.len() < 1 || elements[0].starts_with("<<") {
            return Range::create(0, 0, 0, 0);
        }

        if elements[0].starts_with("<line:") && elements[0].ends_with(">") {
            let parts: Vec<&str> = elements[0][6..elements[0].len() - 1].split(':').collect();
            if parts.len() == 2 {
                if let Ok(line) = parts[0].parse::<usize>() {
                    if let Ok(col) = parts[1].parse::<usize>() {
                        self.last_seen_line = line;
                        elements.remove(0);
                        return Range::create(line, col, line, col);
                    }
                }
            }
        }

        if elements[0].starts_with("<col:") && elements[0].ends_with(">") {
            if let Some(col_str) = elements[0]
                .strip_prefix("<col:")
                .and_then(|s| s.strip_suffix('>'))
            {
                if let Ok(col) = col_str.parse::<usize>() {
                    elements.remove(0);
                    return Range::create(self.last_seen_line, col, self.last_seen_line, col + 1);
                }
            }
        }

        if elements.len() < 2 {
            return Range::create(0, 0, 0, 0);
        }

        if elements[0].starts_with("<") && elements[0].ends_with(",") && elements[1].ends_with(">")
        {
            let start = self.get_first_range_element(&elements[0]);
            let end = self.get_second_range_element(&elements[1]);
            elements.remove(0);
            elements.remove(0);
            return Range::create(start.line, start.column, end.line, end.column + 1);
        }

        Range::create(0, 0, 0, 0)
    }

    fn get_first_range_element(&mut self, element: &str) -> Position {
        if element.starts_with("<col:") && element.ends_with(",") {
            if let Some(col_str) = element
                .strip_prefix("<col:")
                .and_then(|s| s.strip_suffix(','))
            {
                if let Ok(col) = col_str.parse::<usize>() {
                    return Position::new(self.last_seen_line, col);
                }
            }
        }

        if element.starts_with("<line:") && element.ends_with(",") {
            let parts: Vec<&str> = element[6..element.len() - 1].split(':').collect();
            if parts.len() == 2 {
                if let Ok(line) = parts[0].parse::<usize>() {
                    if let Ok(col) = parts[1].parse::<usize>() {
                        self.last_seen_line = line;
                        return Position::new(line, col);
                    }
                }
            }
        }

        if element.starts_with("<") && element.ends_with(",") {
            let parts: Vec<&str> = element[1..element.len() - 1].split(':').collect();
            if parts.len() == 3 {
                self.files.push(Rc::new(parts[0].to_string()));

                if let Ok(line) = parts[1].parse::<usize>() {
                    if let Ok(col) = parts[2].parse::<usize>() {
                        self.last_seen_line = line;
                        return Position::new(line, col);
                    }
                }
            }
            if parts.len() == 4 {
                self.files.push(Rc::new([parts[0], parts[1]].join(":")));

                if let Ok(line) = parts[2].parse::<usize>() {
                    if let Ok(col) = parts[3].parse::<usize>() {
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
                if let Ok(line) = parts[1].parse::<usize>() {
                    // The second part should not store the line number for reuse.
                    // self.last_seen_line = line;
                    if let Ok(col) = parts[2].parse::<usize>() {
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
                if let Ok(col) = col_str.parse::<usize>() {
                    return Position::new(self.last_seen_line, col);
                }
            }
        }

        Position::new(0, 0)
    }
}

fn drop_location(parts: &mut Vec<&str>) {
    if parts.len() < 1 {
        return;
    }

    if parts[0].starts_with("col:") || parts[0].starts_with("line:") {
        parts.remove(0);
        return;
    }

    let candidate = parts[0].to_string();
    let splitted_candidate: Vec<&str> = candidate.split(':').collect();
    if (splitted_candidate.len() == 3 || splitted_candidate.len() == 4)
        && is_number(splitted_candidate[splitted_candidate.len() - 1])
        && is_number(splitted_candidate[splitted_candidate.len() - 2])
    {
        parts.remove(0);
    }
}

fn is_number(s: &str) -> bool {
    s.chars().all(|c| c.is_digit(10))
}

fn get_string_element_start(line: &str) -> usize {
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
        let ast = parser.parse_ast();
        assert_eq!(ast.is_some(), true);
        let ast = ast.unwrap();
        assert_eq!(ast.len(), 2);
        assert_eq!(ast[0].inner.len(), 1);
        assert_eq!(ast[1].inner.len(), 1);
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
    fn test_get_first_range_element_file_relative_path() {
        let process = DummyProcess::new();
        let mut parser = ClangAstParserImpl::new(Box::new(process));
        parser.last_seen_line = 5;

        let element = parser.get_first_range_element("<../foo/bar.c:7:8,");
        assert_eq!(element.line, 7);
        assert_eq!(element.column, 8);
        assert_eq!(parser.files.len(), 2);
        assert_eq!(parser.files.last().unwrap().as_ref(), "../foo/bar.c");
        assert_eq!(parser.last_seen_line, 7);
    }

    #[test]
    fn test_get_first_range_element_file_windows_path() {
        let process = DummyProcess::new();
        let mut parser = ClangAstParserImpl::new(Box::new(process));
        parser.last_seen_line = 5;

        let element = parser.get_first_range_element("<C:\\foo\\bar.c:7:8,");
        assert_eq!(element.line, 7);
        assert_eq!(element.column, 8);
        assert_eq!(parser.files.len(), 2);
        assert_eq!(parser.files.last().unwrap().as_ref(), "C:\\foo\\bar.c");
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
        assert_eq!(element.element_type, "TranslationUnitDecl");
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
        assert_eq!(element.element_type, "BuiltinType");
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
        assert_eq!(element.element_type, "TranslationUnitDecl");
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
        assert_eq!(element.element_type, "FunctionDecl");
        assert_eq!(element.element_id, 0x11d905360);
        assert_eq!(element.file.as_ref(), "/Users/xxx/git/vscode-clang-call-graph/src/test/backendSuite/walkerTests/actualTests/cStyleTests/declInHeaderAndTwoCpps/header.h");
        assert_eq!(element.range.start.line, 1);
        assert_eq!(element.range.start.column, 1);
        assert_eq!(element.range.end.line, 1);
        assert_eq!(element.range.end.column, 28);
        assert_eq!(element.inner.len(), 0);
        assert_eq!(element.attributes, "used add 'int (int, int)'");

        element = parser
            .parse_ast_element("ParmVarDecl 0x11d905208 <col:9, col:13> col:13 val1 'int'")
            .unwrap();
        assert_eq!(element.element_type, "ParmVarDecl");
        assert_eq!(element.element_id, 0x11d905208);
        assert_eq!(element.file.as_ref(), "/Users/xxx/git/vscode-clang-call-graph/src/test/backendSuite/walkerTests/actualTests/cStyleTests/declInHeaderAndTwoCpps/header.h");
        assert_eq!(element.range.start.line, 1);
        assert_eq!(element.range.start.column, 9);
        assert_eq!(element.range.end.line, 1);
        assert_eq!(element.range.end.column, 14);
        assert_eq!(element.inner.len(), 0);
        assert_eq!(element.attributes, "val1 'int'");

        element = parser
            .parse_ast_element("FunctionDecl 0x11d9056c0 </Users/xxx/git/vscode-clang-call-graph/src/test/backendSuite/walkerTests/actualTests/cStyleTests/declInHeaderAndTwoCpps/main.cpp:3:1, line:6:1> line:3:5 main 'int (int, char **)'")
            .unwrap();
        assert_eq!(element.element_type, "FunctionDecl");
        assert_eq!(element.element_id, 0x11d9056c0);
        assert_eq!(element.file.as_ref(), "/Users/xxx/git/vscode-clang-call-graph/src/test/backendSuite/walkerTests/actualTests/cStyleTests/declInHeaderAndTwoCpps/main.cpp");
        assert_eq!(element.range.start.line, 3);
        assert_eq!(element.range.start.column, 1);
        assert_eq!(element.range.end.line, 6);
        assert_eq!(element.range.end.column, 2);
        assert_eq!(element.inner.len(), 0);
        assert_eq!(element.attributes, "main 'int (int, char **)'");

        element = parser
            .parse_ast_element("ParmVarDecl 0x11d905478 <col:10, col:14> col:14 argc 'int'")
            .unwrap();
        assert_eq!(element.element_type, "ParmVarDecl");
        assert_eq!(element.element_id, 0x11d905478);
        assert_eq!(element.file.as_ref(), "/Users/xxx/git/vscode-clang-call-graph/src/test/backendSuite/walkerTests/actualTests/cStyleTests/declInHeaderAndTwoCpps/main.cpp");
        assert_eq!(element.range.start.line, 3);
        assert_eq!(element.range.start.column, 10);
        assert_eq!(element.range.end.line, 3);
        assert_eq!(element.range.end.column, 15);
        assert_eq!(element.inner.len(), 0);
        assert_eq!(element.attributes, "argc 'int'");

        element = parser
            .parse_ast_element("CXXMethodDecl 0x14b830080 <line:1:7> col:7 implicit constexpr operator= 'TestClass &(const TestClass &)' inline default noexcept-unevaluated 0x14b830080")
            .unwrap();
        assert_eq!(element.element_type, "CXXMethodDecl");
        assert_eq!(element.element_id, 0x14b830080);
        assert_eq!(element.file.as_ref(), "/Users/xxx/git/vscode-clang-call-graph/src/test/backendSuite/walkerTests/actualTests/cStyleTests/declInHeaderAndTwoCpps/main.cpp");
        assert_eq!(element.range.start.line, 1);
        assert_eq!(element.range.start.column, 7);
        assert_eq!(element.range.end.line, 1);
        assert_eq!(element.range.end.column, 7);
        assert_eq!(element.inner.len(), 0);
        assert_eq!(element.attributes, "implicit constexpr operator= 'TestClass &(const TestClass &)' inline default noexcept-unevaluated 0x14b830080");
    }

    #[test]
    fn parse_ast_element_structures_without_id() {
        let process = DummyProcess::new();
        let mut parser = ClangAstParserImpl::new(Box::new(process));

        let  element = parser
            .parse_ast_element("CopyAssignment non_trivial has_const_param needs_overload_resolution implicit_has_const_param")
            .unwrap();
        assert_eq!(element.element_type, "CopyAssignment");
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
    fn parse_ast_element_minimal_structure() {
        let process = DummyProcess::new();
        let mut parser = ClangAstParserImpl::new(Box::new(process));

        let element = parser.parse_ast_element("TemplateArgument expr").unwrap();
        assert_eq!(element.element_type, "TemplateArgument");
        assert_eq!(element.element_id, 0x0);
        assert_eq!(element.file.as_ref(), "");
        assert_eq!(element.range.start.line, 0);
        assert_eq!(element.range.start.column, 0);
        assert_eq!(element.range.end.line, 0);
        assert_eq!(element.range.end.column, 0);
        assert_eq!(element.inner.len(), 0);
        assert_eq!(element.attributes, "expr");
    }

    #[test]
    fn parse_ast_one_element_structure() {
        let process = DummyProcess::new();
        let mut parser = ClangAstParserImpl::new(Box::new(process));

        let element = parser.parse_ast_element("TemplateArgument").unwrap();
        assert_eq!(element.element_type, "TemplateArgument");
        assert_eq!(element.element_id, 0x0);
        assert_eq!(element.file.as_ref(), "");
        assert_eq!(element.range.start.line, 0);
        assert_eq!(element.range.start.column, 0);
        assert_eq!(element.range.end.line, 0);
        assert_eq!(element.range.end.column, 0);
        assert_eq!(element.inner.len(), 0);
        assert_eq!(element.attributes, "");
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
        assert_eq!(element.element_type, "Overrides");
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
        let ast = parser.parse_ast();
        assert_eq!(ast.is_some(), true);
        let ast = ast.unwrap();
        assert_eq!(ast.len(), 0);
    }

    #[test]
    fn wrong_file_output() {
        let mut process = DummyProcess::new();
        process.add_line("test".to_string());
        let mut parser = ClangAstParserImpl::new(Box::new(process));
        let ast = parser.parse_ast();
        assert_eq!(ast.is_some(), false);
    }

    #[test]
    fn parse_ast_element_with_func_prev() {
        let process = DummyProcess::new();
        let mut parser = ClangAstParserImpl::new(Box::new(process));

        let element = parser
            .parse_ast_element("FunctionDecl 0x15591de00 prev 0x155904b98 <line:5:1, line:7:1> line:5:5 mult 'int (int, int)'")
            .unwrap();
        assert_eq!(element.element_type, "FunctionDecl");
        assert_eq!(element.element_id, 0x15591de00);
        assert_eq!(element.prev_element_id, 0x155904b98);
        assert_eq!(element.file.as_ref(), "");
        assert_eq!(element.range.start.line, 5);
        assert_eq!(element.range.start.column, 1);
        assert_eq!(element.range.end.line, 7);
        assert_eq!(element.range.end.column, 2);
        assert_eq!(element.inner.len(), 0);
        assert_eq!(element.attributes, "mult 'int (int, int)'");
    }

    #[test]
    fn remove_trailing_location() {
        let process = DummyProcess::new();
        let mut parser = ClangAstParserImpl::new(Box::new(process));

        let mut element = parser
            .parse_ast_element("FunctionDecl 0x15591de00 prev 0x155904b98 <line:5:1, line:7:1> col:5 mult 'int (int, int)'")
            .unwrap();
        assert_eq!(element.element_type, "FunctionDecl");
        assert_eq!(element.element_id, 0x15591de00);
        assert_eq!(element.prev_element_id, 0x155904b98);
        assert_eq!(element.file.as_ref(), "");
        assert_eq!(element.range.start.line, 5);
        assert_eq!(element.range.start.column, 1);
        assert_eq!(element.range.end.line, 7);
        assert_eq!(element.range.end.column, 2);
        assert_eq!(element.inner.len(), 0);
        assert_eq!(element.attributes, "mult 'int (int, int)'");

        element = parser
            .parse_ast_element("FunctionDecl 0x15591de00 prev 0x155904b98 <line:5:1, line:7:1> line:5:5 mult 'int (int, int)'")
            .unwrap();
        assert_eq!(element.element_type, "FunctionDecl");
        assert_eq!(element.element_id, 0x15591de00);
        assert_eq!(element.prev_element_id, 0x155904b98);
        assert_eq!(element.file.as_ref(), "");
        assert_eq!(element.range.start.line, 5);
        assert_eq!(element.range.start.column, 1);
        assert_eq!(element.range.end.line, 7);
        assert_eq!(element.range.end.column, 2);
        assert_eq!(element.inner.len(), 0);
        assert_eq!(element.attributes, "mult 'int (int, int)'");

        element = parser
            .parse_ast_element("FunctionDecl 0x15591de00 prev 0x155904b98 <line:5:1, line:7:1> /foo/ba.cpp:5:5 mult 'int (int, int)'")
            .unwrap();
        assert_eq!(element.element_type, "FunctionDecl");
        assert_eq!(element.element_id, 0x15591de00);
        assert_eq!(element.prev_element_id, 0x155904b98);
        assert_eq!(element.file.as_ref(), "");
        assert_eq!(element.range.start.line, 5);
        assert_eq!(element.range.start.column, 1);
        assert_eq!(element.range.end.line, 7);
        assert_eq!(element.range.end.column, 2);
        assert_eq!(element.inner.len(), 0);
        assert_eq!(element.attributes, "mult 'int (int, int)'");

        element = parser
            .parse_ast_element("FunctionDecl 0x15591de00 prev 0x155904b98 <line:5:1, line:7:1> C:\\foo\\ba.cpp:5:5 mult 'int (int, int)'")
            .unwrap();
        assert_eq!(element.element_type, "FunctionDecl");
        assert_eq!(element.element_id, 0x15591de00);
        assert_eq!(element.prev_element_id, 0x155904b98);
        assert_eq!(element.file.as_ref(), "");
        assert_eq!(element.range.start.line, 5);
        assert_eq!(element.range.start.column, 1);
        assert_eq!(element.range.end.line, 7);
        assert_eq!(element.range.end.column, 2);
        assert_eq!(element.inner.len(), 0);
        assert_eq!(element.attributes, "mult 'int (int, int)'");
    }
}
