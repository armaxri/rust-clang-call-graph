use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum ClangAstElementType {
    CallExpr,
    ClassTemplateDecl,
    ClassTemplateSpecializationDecl,
    CompoundStmt,
    CXXMemberCallExpr,
    CXXMethodDecl,
    CXXRecordDecl,
    DeclRefExpr,
    FunctionDecl,
    FunctionTemplateDecl,
    MemberExpr,
    NamespaceDecl,
    Overrides,
    #[allow(non_camel_case_types)]
    private,
    #[allow(non_camel_case_types)]
    protected,
    #[allow(non_camel_case_types)]
    public,
    TemplateArgument,
    TypedefDecl,
}

impl FromStr for ClangAstElementType {
    type Err = ();

    fn from_str(input: &str) -> Result<ClangAstElementType, Self::Err> {
        match input {
            "CallExpr" => Ok(ClangAstElementType::CallExpr),
            "ClassTemplateDecl" => Ok(ClangAstElementType::ClassTemplateDecl),
            "ClassTemplateSpecializationDecl" => {
                Ok(ClangAstElementType::ClassTemplateSpecializationDecl)
            }
            "CompoundStmt" => Ok(ClangAstElementType::CompoundStmt),
            "CXXMemberCallExpr" => Ok(ClangAstElementType::CXXMemberCallExpr),
            "CXXMethodDecl" => Ok(ClangAstElementType::CXXMethodDecl),
            "CXXRecordDecl" => Ok(ClangAstElementType::CXXRecordDecl),
            "DeclRefExpr" => Ok(ClangAstElementType::DeclRefExpr),
            "FunctionDecl" => Ok(ClangAstElementType::FunctionDecl),
            "FunctionTemplateDecl" => Ok(ClangAstElementType::FunctionTemplateDecl),
            "MemberExpr" => Ok(ClangAstElementType::MemberExpr),
            "NamespaceDecl" => Ok(ClangAstElementType::NamespaceDecl),
            "Overrides" => Ok(ClangAstElementType::Overrides),
            "private" => Ok(ClangAstElementType::private),
            "protected" => Ok(ClangAstElementType::protected),
            "public" => Ok(ClangAstElementType::public),
            "TemplateArgument" => Ok(ClangAstElementType::TemplateArgument),
            "TypedefDecl" => Ok(ClangAstElementType::TypedefDecl),
            _ => Err(()),
        }
    }
}
