#[macro_export]
macro_rules! directory {
    () => {{
        let file_path = file!();
        let dir_path = std::path::Path::new(file_path).parent().unwrap();
        dir_path.to_str().unwrap()
    }};
}

#[macro_export]
macro_rules! file_in_directory {
    ($file_name:expr) => {{
        let file_path = file!();
        let dir_path = std::path::Path::new(file_path).parent().unwrap();
        let file_path = dir_path.join($file_name);
        file_path.to_str().unwrap().to_string()
    }};
}

#[macro_export]
macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let mut name = type_name_of(f);
        name = &name[..name.len() - 3]; // Remove the trailing "::f"
        let parts: Vec<&str> = name.split("::").collect();
        parts.last().unwrap().to_string()
    }};
}

#[macro_export]
macro_rules! func_file_in_directory {
    ($file_extension:expr) => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let mut name = type_name_of(f);
        name = &name[..name.len() - 3]; // Remove the trailing "::f"
        let parts: Vec<&str> = name.split("::").collect();
        let func_name = parts.last().unwrap().to_string();

        let file_path = file!();
        let dir_path = std::path::Path::new(file_path).parent().unwrap();
        let file_path =
            dir_path.join([func_name, ".".to_string(), $file_extension.to_string()].concat());
        file_path.to_str().unwrap().to_string()
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_directory() {
        let dir = directory!();
        assert_eq!(dir, "src");
    }

    #[test]
    fn test_file_in_directory() {
        let file = file_in_directory!("macros.rs");
        assert_eq!(file, "src/macros.rs");
    }

    #[test]
    fn test_function_name() {
        let name = function_name!();
        assert_eq!(name, "test_function_name");
    }

    #[test]
    fn test_func_file_in_directory() {
        let file = func_file_in_directory!("rs");
        assert_eq!(file, "src/test_func_file_in_directory.rs");
    }
}
