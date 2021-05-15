pub struct CodeModule {
    pub name: String,
    pub path: String,
    pub package: Vec<CodePackage>,
}

pub struct CodePackage {
    pub name: String,
    pub path: String,
    pub class: Vec<CodeClass>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeFile {
    pub name: String,
    pub path: String,
    pub imports: Vec<String>,
    pub classes: Vec<CodeClass>,
    pub functions: Vec<CodeFunction>,
}

impl Default for CodeFile {
    fn default() -> Self {
        CodeFile {
            name: "".to_string(),
            path: "".to_string(),
            imports: vec![],
            classes: vec![],
            functions: vec![],
        }
    }
}

pub struct CodeImport {
    pub name: String,
    pub import: String,
    pub source: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeClass {
    pub name: String,
    pub path: String,
    pub constant: Vec<ClassConstant>,
    pub functions: Vec<CodeFunction>,
    pub start: CodePoint,
    pub end: CodePoint
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodePoint {
    pub row: usize,
    pub column: usize
}

impl Default for CodePoint {
    fn default() -> Self {
        CodePoint {
            row: 0,
            column: 0
        }
    }
}

impl Default for CodeClass {
    fn default() -> Self {
        CodeClass {
            name: "".to_string(),
            path: "".to_string(),
            constant: vec![],
            functions: vec![],
            start: Default::default(),
            end: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClassConstant {
    pub name: String,
    pub typ: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeFunction {
    pub name: String,
    pub vars: Vec<String>,
}

impl Default for CodeFunction {
    fn default() -> Self {
        CodeFunction {
            name: "".to_string(),
            vars: vec![],
        }
    }
}

