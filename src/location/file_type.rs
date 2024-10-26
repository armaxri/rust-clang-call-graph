#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuncTypeEnum {
    SourceFile,
    HeaderFile,
}

pub fn get_file_type(file_name: &String) -> FuncTypeEnum {
    let splitted_file_name: Vec<&str> = file_name.split('.').collect();

    if splitted_file_name.len() <= 1 {
        return FuncTypeEnum::HeaderFile;
    }

    let file_extension = splitted_file_name[splitted_file_name.len() - 1].to_lowercase();

    match file_extension.as_str() {
        "c" | "cpp" | "cc" | "cxx" | "c++" | "cp" => FuncTypeEnum::SourceFile,
        _ => FuncTypeEnum::HeaderFile,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_file_type() {
        assert_eq!(
            get_file_type(&String::from("test.c")),
            FuncTypeEnum::SourceFile
        );
        assert_eq!(
            get_file_type(&String::from("test.cpp")),
            FuncTypeEnum::SourceFile
        );
        assert_eq!(
            get_file_type(&String::from("test.cc")),
            FuncTypeEnum::SourceFile
        );
        assert_eq!(
            get_file_type(&String::from("test.cxx")),
            FuncTypeEnum::SourceFile
        );
        assert_eq!(
            get_file_type(&String::from("test.c++")),
            FuncTypeEnum::SourceFile
        );
        assert_eq!(
            get_file_type(&String::from("test.cp")),
            FuncTypeEnum::SourceFile
        );
        assert_eq!(
            get_file_type(&String::from("test.h")),
            FuncTypeEnum::HeaderFile
        );
        assert_eq!(
            get_file_type(&String::from("test.hpp")),
            FuncTypeEnum::HeaderFile
        );
        assert_eq!(
            get_file_type(&String::from("test.hh")),
            FuncTypeEnum::HeaderFile
        );
        assert_eq!(
            get_file_type(&String::from("test.hxx")),
            FuncTypeEnum::HeaderFile
        );
        assert_eq!(
            get_file_type(&String::from("test.h++")),
            FuncTypeEnum::HeaderFile
        );
        assert_eq!(
            get_file_type(&String::from("test.hp")),
            FuncTypeEnum::HeaderFile
        );
        assert_eq!(
            get_file_type(&String::from("test")),
            FuncTypeEnum::HeaderFile
        );
        assert_eq!(
            get_file_type(&String::from("foo.cpp")),
            FuncTypeEnum::SourceFile
        );
        assert_eq!(
            get_file_type(&String::from("foo.CPP")),
            FuncTypeEnum::SourceFile
        );
        assert_eq!(
            get_file_type(&String::from("foo.cxx")),
            FuncTypeEnum::SourceFile
        );
        assert_eq!(
            get_file_type(&String::from("foo.CXX")),
            FuncTypeEnum::SourceFile
        );
        assert_eq!(
            get_file_type(&String::from("foo.c++")),
            FuncTypeEnum::SourceFile
        );
        assert_eq!(
            get_file_type(&String::from("foo.C++")),
            FuncTypeEnum::SourceFile
        );
        assert_eq!(
            get_file_type(&String::from("foo.cp")),
            FuncTypeEnum::SourceFile
        );
        assert_eq!(
            get_file_type(&String::from("foo.CP")),
            FuncTypeEnum::SourceFile
        );
        assert_eq!(
            get_file_type(&String::from("foo.cc")),
            FuncTypeEnum::SourceFile
        );
        assert_eq!(
            get_file_type(&String::from("foo.CC")),
            FuncTypeEnum::SourceFile
        );
        assert_eq!(
            get_file_type(&String::from("foo.c")),
            FuncTypeEnum::SourceFile
        );
        assert_eq!(
            get_file_type(&String::from("foo.C")),
            FuncTypeEnum::SourceFile
        );
        assert_eq!(
            get_file_type(&String::from("foo.hpp")),
            FuncTypeEnum::HeaderFile
        );
        assert_eq!(
            get_file_type(&String::from("foo.HPP")),
            FuncTypeEnum::HeaderFile
        );
        assert_eq!(
            get_file_type(&String::from("foo.h")),
            FuncTypeEnum::HeaderFile
        );
        assert_eq!(
            get_file_type(&String::from("foo.H")),
            FuncTypeEnum::HeaderFile
        );
        assert_eq!(
            get_file_type(&String::from("foo")),
            FuncTypeEnum::HeaderFile
        );
        assert_eq!(
            get_file_type(&String::from("foo.cppp")),
            FuncTypeEnum::HeaderFile
        );
    }
}
