use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
};

use crate::{
    ast_reader::clang_ast_element::ClangAstElement,
    call_graph::{
        data_structure::{
            cpp_class::CppClass, file_structure::FileStructure, func_structure::FuncStructure,
            helper::func_creation_args::FuncCreationArgs, FuncBasics, FuncImplBasics,
            MainDeclPosition, VirtualFuncBasics,
        },
        database::database_sqlite::DatabaseSqlite,
    },
    location::range::Range,
};

struct ClangAstWalkerInternal {
    pub db: Rc<RefCell<DatabaseSqlite>>,
    pub file_path: String,
    pub current_file: Rc<RefCell<FileStructure>>,
    pub known_func_decls_and_impls: HashMap<usize, Rc<RefCell<FuncStructure>>>,
    pub known_classes: HashMap<String, Rc<RefCell<CppClass>>>,
    pub current_class_stack: Vec<Rc<RefCell<CppClass>>>,
    pub open_func_call_connections: HashMap<usize, Vec<(Range, Rc<RefCell<FuncStructure>>)>>,
    pub current_func_impl_ast_id: usize,
}

pub fn walk_ast_2_func_call_db(
    file_path: &str,
    parsed_ast: VecDeque<ClangAstElement>,
    db: Rc<RefCell<DatabaseSqlite>>,
) {
    // Make sure that the file is in the database, so that we can reference it.
    let main_file = db.borrow().get_or_add_cpp_file(&file_path);
    let mut current_file_name_str = main_file.borrow().get_name().to_string();

    let mut walker = ClangAstWalkerInternal {
        db: db,
        file_path: file_path.to_string(),
        current_file: main_file.clone(),
        known_func_decls_and_impls: HashMap::new(),
        known_classes: HashMap::new(),
        current_class_stack: Vec::new(),
        open_func_call_connections: HashMap::new(),
        current_func_impl_ast_id: 0,
    };

    for ast_element in parsed_ast {
        if *ast_element.file == "" {
            continue;
        }

        if *ast_element.file != current_file_name_str {
            if *ast_element.file == walker.file_path {
                walker.current_file = main_file.clone();
            } else {
                walker.current_file = walker.db.borrow().get_or_add_hpp_file(&ast_element.file);
                walker
                    .current_file
                    .borrow_mut()
                    .add_referenced_from_source_file(&main_file);
            }
            current_file_name_str = walker.current_file.borrow().get_name().to_string();
        }

        handle_ast_element(&ast_element, &mut walker, "");
    }
}

fn handle_ast_element(
    ast_element: &ClangAstElement,
    walker: &mut ClangAstWalkerInternal,
    name_prefix: &str,
) {
    match ast_element.element_type.as_str() {
        "FunctionDecl" => {
            handle_function_decl(ast_element, walker, name_prefix, None);
        }
        "CXXMethodDecl" => {
            handle_function_decl(ast_element, walker, name_prefix, None);
        }
        "NamespaceDecl" => {
            handle_namespace_decl(ast_element, walker, name_prefix);
        }
        "CXXRecordDecl" => {
            handle_cxx_record_decl(ast_element, walker, name_prefix);
        }
        "ClassTemplateDecl" => {
            handle_class_template_decl(ast_element, walker, name_prefix);
        }
        "FunctionTemplateDecl" => {
            handle_function_template_decl(ast_element, walker, name_prefix);
        }
        _ => {
            for inner_element in &ast_element.inner {
                handle_ast_element(inner_element, walker, name_prefix);
            }
        }
    }
}

fn handle_function_template_decl(
    ast_element: &ClangAstElement,
    walker: &mut ClangAstWalkerInternal,
    name_prefix: &str,
) {
    let template_func_name = &ast_element.attributes;

    'func_decls_loop: for inner_element in &ast_element.inner {
        if inner_element.element_type.as_str() == "FunctionDecl" {
            for inner_inner_element in &inner_element.inner {
                if inner_inner_element.element_type.as_str() == "TemplateArgument" {
                    handle_function_decl(
                        inner_element,
                        walker,
                        name_prefix,
                        Some(template_func_name),
                    );

                    continue 'func_decls_loop;
                }
            }
        }
    }
}

fn handle_class_template_decl(
    ast_element: &ClangAstElement,
    walker: &mut ClangAstWalkerInternal,
    name_prefix: &str,
) {
    let template_class_name = &ast_element.attributes;

    let new_class = if walker.current_class_stack.len() > 0 {
        walker
            .current_class_stack
            .last()
            .unwrap()
            .borrow_mut()
            .get_or_add_class(template_class_name)
    } else {
        walker
            .current_file
            .borrow_mut()
            .get_or_add_class(template_class_name)
    };

    walker.current_class_stack.push(new_class.clone());
    walker
        .known_classes
        .insert(template_class_name.clone(), new_class.clone());

    let new_name_prefix = if name_prefix == "" {
        format!("{}::", template_class_name)
    } else {
        format!("{}{}::", name_prefix, template_class_name)
    };

    for inner_element in &ast_element.inner {
        match inner_element.element_type.as_str() {
            "CXXRecordDecl" => {
                if inner_element
                    .attributes
                    .ends_with(format!("class {} definition", template_class_name).as_str())
                {
                    continue;
                }
                handle_cxx_record_decl(inner_element, walker, &new_name_prefix);
            }
            "ClassTemplateSpecializationDecl" => {
                handle_class_template_specialization_decl(inner_element, walker, &new_name_prefix);
            }
            _ => {
                handle_ast_element(inner_element, walker, &new_name_prefix);
            }
        }
    }

    walker.current_class_stack.pop();
}

fn collect_template_specialization(ast_element: &ClangAstElement) -> Vec<&str> {
    let mut templates: Vec<&str> = Vec::new();

    for inner_element in &ast_element.inner {
        if inner_element.element_type.as_str() == "TemplateArgument"
            && inner_element.attributes.starts_with("type '")
        {
            templates.push(
                &inner_element.attributes["type '".len()..inner_element.attributes.len() - 1],
            );
        }
    }

    templates
}

fn handle_class_template_specialization_decl(
    ast_element: &ClangAstElement,
    walker: &mut ClangAstWalkerInternal,
    name_prefix: &str,
) {
    let templates: Vec<&str> = collect_template_specialization(ast_element);

    let template_string = templates.join(", ");
    let new_name_prefix = format!(
        "{}<{}>::",
        &name_prefix[..name_prefix.len() - 2].to_string(),
        template_string
    );

    for inner_element in &ast_element.inner {
        handle_ast_element(inner_element, walker, &new_name_prefix);
    }
}

fn handle_cxx_record_decl(
    ast_element: &ClangAstElement,
    walker: &mut ClangAstWalkerInternal,
    name_prefix: &str,
) {
    if ast_element.attributes.starts_with("implicit ") {
        return;
    }

    let splitted_args: Vec<&str> = ast_element.attributes.split(" ").collect();

    // This is maybe not the best solution, but it works for now.
    let mut func_keyword_index = splitted_args.iter().position(|&attr| attr == "class");
    if func_keyword_index.is_none() {
        func_keyword_index = splitted_args.iter().position(|&attr| attr == "struct");
    }

    match func_keyword_index {
        Some(index) => {
            let class_name = splitted_args[index + 1];
            let new_name_prefix = if name_prefix == "" {
                format!("{}::", class_name)
            } else {
                format!("{}{}::", name_prefix, class_name)
            };

            let used_name = format!("{}{}", name_prefix, class_name);
            let class = if walker.current_class_stack.len() > 0 {
                walker
                    .current_class_stack
                    .last()
                    .unwrap()
                    .borrow_mut()
                    .get_or_add_class(&used_name)
            } else {
                walker
                    .current_file
                    .borrow_mut()
                    .get_or_add_class(&used_name)
            };

            walker.current_class_stack.push(class.clone());
            walker.known_classes.insert(used_name, class.clone());

            for inner_element in &ast_element.inner {
                match inner_element.element_type.as_str() {
                    "public" | "protected" | "private" => {
                        let splitted_modifiers: Vec<&str> =
                            inner_element.attributes.split("':'").collect();
                        let parent_name = if splitted_modifiers.len() > 1 {
                            splitted_modifiers[1][..splitted_modifiers[1].len() - 1].to_string()
                        } else {
                            "".to_string()
                        };

                        if parent_name != "" {
                            let parent_class = walker.known_classes.get(&parent_name);
                            if parent_class.is_some() {
                                class.borrow_mut().add_parent_class(&parent_class.unwrap());
                            } else {
                                println!("Could not find parent class with name {}", parent_name);
                            }
                        }
                    }
                    _ => {
                        handle_ast_element(inner_element, walker, &new_name_prefix);
                    }
                }
            }

            walker.current_class_stack.pop();
        }
        None => {}
    }
}

fn handle_namespace_decl(
    ast_element: &ClangAstElement,
    walker: &mut ClangAstWalkerInternal,
    name_prefix: &str,
) {
    let namespace_str = ast_element.attributes.split(" ").last().unwrap();
    let new_name_prefix = if name_prefix == "" {
        format!("{}::", namespace_str)
    } else {
        format!("{}{}::", name_prefix, namespace_str)
    };

    for inner_element in &ast_element.inner {
        handle_ast_element(inner_element, walker, &new_name_prefix);
    }
}

fn handle_function_decl(
    ast_element: &ClangAstElement,
    walker: &mut ClangAstWalkerInternal,
    name_prefix: &str,
    template_func_name: Option<&str>,
) {
    if ast_element
        .attributes
        .starts_with("<<invalid sloc>> <invalid sloc> implicit")
        || ast_element.attributes.starts_with("implicit ")
    {
        return;
    }

    let compound_stmt = get_compound_stmt(ast_element);
    let mut func_creation_args = if ast_element.prev_element_id != 0 {
        walker
            .known_func_decls_and_impls
            .get(&ast_element.prev_element_id)
            .unwrap()
            .borrow()
            .convert_func2func_creation_args4call(&ast_element.range)
    } else {
        ast_element.create_func_creation_args(Some(walker), name_prefix)
    };

    if template_func_name.is_some() {
        let used_template_func_name = template_func_name.unwrap().to_string();
        let templates: Vec<&str> = collect_template_specialization(ast_element);
        func_creation_args.set_new_qualified_name(format!(
            "{}{}<{}>{}",
            name_prefix,
            used_template_func_name,
            templates.join(", "),
            func_creation_args.qualified_name[used_template_func_name.len()..].to_string()
        ));
    }

    match compound_stmt {
        Some(_compound_stmt) => {
            let func_impl = if walker.current_class_stack.len() > 0 {
                walker
                    .current_class_stack
                    .last()
                    .unwrap()
                    .borrow_mut()
                    .get_or_add_func_impl(func_creation_args)
            } else {
                walker
                    .current_file
                    .borrow_mut()
                    .get_or_add_func_impl(func_creation_args)
            };
            walker
                .known_func_decls_and_impls
                .insert(ast_element.element_id, func_impl.clone());

            if walker
                .open_func_call_connections
                .contains_key(&ast_element.element_id)
            {
                for missing_connection in walker
                    .open_func_call_connections
                    .get(&ast_element.element_id)
                    .unwrap()
                {
                    missing_connection.1.borrow_mut().get_or_add_func_call(
                        &func_impl
                            .borrow()
                            .convert_func2func_creation_args4call(&missing_connection.0),
                    );
                }
                walker
                    .open_func_call_connections
                    .remove(&ast_element.element_id);
            }

            walker.current_func_impl_ast_id = ast_element.element_id;
            for inner_element in &ast_element.inner {
                walk_func_impl_inner(inner_element, &func_impl, walker, &ast_element.range);
            }
            walker.current_func_impl_ast_id = 0;
        }
        None => {
            if func_creation_args.is_virtual() {
                walker.known_func_decls_and_impls.insert(
                    ast_element.element_id,
                    walker
                        .current_class_stack
                        .last()
                        .unwrap()
                        .borrow_mut()
                        .get_or_add_virtual_func_decl(func_creation_args),
                );
            } else {
                walker.known_func_decls_and_impls.insert(
                    ast_element.element_id,
                    if walker.current_class_stack.len() > 0 {
                        walker
                            .current_class_stack
                            .last()
                            .unwrap()
                            .borrow_mut()
                            .get_or_add_func_decl(func_creation_args)
                    } else {
                        walker
                            .current_file
                            .borrow_mut()
                            .get_or_add_func_decl(func_creation_args)
                    },
                );
            }
        }
    }
}

fn walk_func_impl_inner(
    ast_element: &ClangAstElement,
    func_impl: &Rc<RefCell<FuncStructure>>,
    walker: &mut ClangAstWalkerInternal,
    current_range: &Range,
) {
    let mut used_current_range = current_range;
    match ast_element.element_type.as_str() {
        "DeclRefExpr" => {
            let splitted_attributes: Vec<&str> = ast_element.attributes.split(" ").collect();
            let mut func_keyword_index = splitted_attributes
                .iter()
                .position(|&attr| attr == "Function");
            if func_keyword_index.is_none() {
                func_keyword_index = splitted_attributes
                    .iter()
                    .position(|&attr| attr == "CXXMethod");
            }
            match func_keyword_index {
                Some(index) => {
                    if splitted_attributes.len() >= index + 1
                        && splitted_attributes[index + 1].starts_with("0x")
                    {
                        if let Ok(hex_value) =
                            usize::from_str_radix(&splitted_attributes[index + 1][2..], 16)
                        {
                            let func_decl_id = hex_value as usize;

                            if func_decl_id == walker.current_func_impl_ast_id {
                                let creation_args = func_impl
                                    .borrow()
                                    .convert_func2func_creation_args4call(used_current_range);
                                func_impl.borrow_mut().get_or_add_func_call(&creation_args);
                                return;
                            }

                            let func_decl = walker.known_func_decls_and_impls.get(&func_decl_id);

                            if func_decl.is_some() {
                                func_impl.borrow_mut().get_or_add_func_call(
                                    &func_decl
                                        .unwrap()
                                        .borrow()
                                        .convert_func2func_creation_args4call(used_current_range),
                                );
                            } else {
                                match walker.open_func_call_connections.contains_key(&hex_value) {
                                    true => {
                                        walker
                                            .open_func_call_connections
                                            .get_mut(&hex_value)
                                            .unwrap()
                                            .push((used_current_range.clone(), func_impl.clone()));
                                    }
                                    false => {
                                        walker.open_func_call_connections.insert(
                                            hex_value,
                                            vec![(used_current_range.clone(), func_impl.clone())],
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
                None => {}
            }
        }
        "MemberExpr" => {
            let splitted_attributes: Vec<&str> = ast_element.attributes.split(" ").collect();

            if splitted_attributes.last().unwrap().starts_with("0x") {
                if let Ok(hex_value) =
                    usize::from_str_radix(&splitted_attributes.last().unwrap()[2..], 16)
                {
                    let func_decl_id = hex_value as usize;
                    let func_decl = walker.known_func_decls_and_impls.get(&func_decl_id);

                    if func_decl.is_some() {
                        func_impl.borrow_mut().get_or_add_func_call(
                            &func_decl
                                .unwrap()
                                .borrow()
                                .convert_func2func_creation_args4call(used_current_range),
                        );
                    } else {
                        match walker.open_func_call_connections.contains_key(&hex_value) {
                            true => {
                                walker
                                    .open_func_call_connections
                                    .get_mut(&hex_value)
                                    .unwrap()
                                    .push((used_current_range.clone(), func_impl.clone()));
                            }
                            false => {
                                walker.open_func_call_connections.insert(
                                    hex_value,
                                    vec![(used_current_range.clone(), func_impl.clone())],
                                );
                            }
                        }
                    }
                }
            }
        }
        "CallExpr" | "CXXMemberCallExpr" => {
            used_current_range = &ast_element.range;
        }
        _ => {}
    }

    for inner_element in &ast_element.inner {
        walk_func_impl_inner(inner_element, func_impl, walker, used_current_range);
    }
}

fn get_compound_stmt(ast_element: &ClangAstElement) -> Option<&ClangAstElement> {
    for child in &ast_element.inner {
        if child.element_type == "CompoundStmt" {
            return Some(child);
        }
    }
    None
}

impl ClangAstElement {
    fn create_func_creation_args(
        &self,
        walker: Option<&ClangAstWalkerInternal>,
        name_prefix: &str,
    ) -> FuncCreationArgs {
        let splitted_attributes: Vec<&str> = self.attributes.split(" ").collect();
        let start_index = get_in_function_qual_type_start_index(&splitted_attributes);
        let end_index = get_in_function_qual_type_end_index(&splitted_attributes);
        let binding = splitted_attributes[start_index..end_index + 1].join(" ");
        let qualified_type = binding.as_str();
        let qualified_name = &format!(
            "{}{}",
            name_prefix,
            splitted_attributes[start_index - 1..end_index + 1]
                .join(" ")
                .as_str()
        );
        let base_qualified_name: Option<String> = if splitted_attributes.len() >= end_index + 2
            && splitted_attributes[end_index + 1] == "virtual"
        {
            Some(qualified_name.clone())
        } else {
            self.get_base_qualified_name_from_override(walker)
        };

        FuncCreationArgs::new(
            splitted_attributes[start_index - 1],
            qualified_name,
            base_qualified_name,
            qualified_type[1..qualified_type.len() - 1]
                .to_string()
                .as_str(),
            self.range.clone(),
        )
    }

    fn get_base_qualified_name_from_override(
        &self,
        walker: Option<&ClangAstWalkerInternal>,
    ) -> Option<String> {
        for inner_element in &self.inner {
            if inner_element.element_type == "Overrides" {
                let splitted_attributes: Vec<&str> = inner_element.attributes.split(" ").collect();
                if splitted_attributes.len() >= 1 && splitted_attributes[0].starts_with("0x") {
                    if let Ok(hex_value) = usize::from_str_radix(&splitted_attributes[0][2..], 16) {
                        let func_decl_id = hex_value as usize;
                        let func_decl = walker
                            .unwrap()
                            .known_func_decls_and_impls
                            .get(&func_decl_id);

                        if func_decl.is_some() {
                            return Some(
                                func_decl
                                    .unwrap()
                                    .borrow()
                                    .get_base_qualified_name()
                                    .to_string(),
                            );
                        } else {
                            println!("Could not find function decl with id 0x{:x}", func_decl_id);
                        }
                    }

                    return Some(splitted_attributes[1..].join(" "));
                }
            }
        }

        None
    }
}

fn get_in_function_qual_type_start_index(current_vec: &Vec<&str>) -> usize {
    for (i, elem) in current_vec.iter().enumerate() {
        if elem.starts_with("'") {
            return i;
        }
    }
    panic!("No name start found in func name: {:?}", current_vec);
}

fn get_in_function_qual_type_end_index(current_vec: &Vec<&str>) -> usize {
    for (i, elem) in current_vec.iter().enumerate().rev() {
        if elem.ends_with("'") {
            return i;
        }
    }
    panic!("No name start found in func name: {:?}", current_vec);
}

#[cfg(test)]
mod tests {
    use crate::location::range::Range;

    use super::*;

    #[test]
    fn get_in_function_qual_type_start_index_test() {
        assert_eq!(
            get_in_function_qual_type_start_index(&vec!["add", "'int", "(int,", "int)'", "extern"]),
            1
        );
        assert_eq!(
            get_in_function_qual_type_start_index(&vec![
                "blub", "add", "'int", "(int,", "int)'", "extern"
            ]),
            2
        );
    }

    #[test]
    fn get_in_function_qual_type_end_index_test() {
        assert_eq!(
            get_in_function_qual_type_end_index(&vec!["add", "'int", "(int,", "int)'", "extern"]),
            3
        );
        assert_eq!(
            get_in_function_qual_type_end_index(&vec!["blub", "add", "'int", "(int,", "int)'"]),
            4
        );
    }

    #[test]
    fn create_func_creation_args_test() {
        let input = ClangAstElement {
            element_type: "FunctionDecl".to_string(),
            element_id: 0x123011160,
            parent_element_id: 0,
            prev_element_id: 0,
            file: Rc::new("test.cpp".to_string()),
            range: Range::create(1, 2, 3, 4),
            inner: VecDeque::new(),
            attributes: "add 'int (int, int)'".to_string(),
        };
        let converted_args = input.create_func_creation_args(None, "");

        let expected_args = FuncCreationArgs {
            name: "add".to_string(),
            qualified_name: "add 'int (int, int)'".to_string(),
            base_qualified_name: None,
            qualified_type: "int (int, int)".to_string(),
            range: Range::create(1, 2, 3, 4),
        };

        assert_eq!(converted_args, expected_args);
    }

    #[test]
    fn create_func_creation_args_test_with_used() {
        let input = ClangAstElement {
            element_type: "FunctionDecl".to_string(),
            element_id: 0x123011160,
            parent_element_id: 0,
            prev_element_id: 0,
            file: Rc::new("test.cpp".to_string()),
            range: Range::create(1, 2, 3, 4),
            inner: VecDeque::new(),
            attributes: "used add 'int (int, int)'".to_string(),
        };
        let converted_args = input.create_func_creation_args(None, "");

        let expected_args = FuncCreationArgs {
            name: "add".to_string(),
            qualified_name: "add 'int (int, int)'".to_string(),
            base_qualified_name: None,
            qualified_type: "int (int, int)".to_string(),
            range: Range::create(1, 2, 3, 4),
        };

        assert_eq!(converted_args, expected_args);
    }

    #[test]
    fn create_func_creation_args_test_with_extern() {
        let input = ClangAstElement {
            element_type: "FunctionDecl".to_string(),
            element_id: 0x123011160,
            parent_element_id: 0,
            prev_element_id: 0,
            file: Rc::new("test.cpp".to_string()),
            range: Range::create(1, 2, 3, 4),
            inner: VecDeque::new(),
            attributes: "add 'int (int, int)' extern".to_string(),
        };
        let converted_args = input.create_func_creation_args(None, "");

        let expected_args = FuncCreationArgs {
            name: "add".to_string(),
            qualified_name: "add 'int (int, int)'".to_string(),
            base_qualified_name: None,
            qualified_type: "int (int, int)".to_string(),
            range: Range::create(1, 2, 3, 4),
        };

        assert_eq!(converted_args, expected_args);
    }
}
