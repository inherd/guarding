use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GuardRule {
    pub origin: String,
    pub ty: RuleType,
    pub level: RuleLevel,
    pub scope: RuleScope,
    pub expr: Expr,
    pub ops: Vec<Operator>,
    pub assert: RuleAssert
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LayeredRule {
    Normal(NormalLayered),
    Onion(OnionArch)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NormalLayered {

}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OnionArch {

}


impl Default for GuardRule {
    fn default() -> Self {
        GuardRule {
            origin: "".to_string(),
            ty: RuleType::Normal,
            level: RuleLevel::Class,
            scope: RuleScope::All,
            expr: Expr::Identifier("".to_string()),
            ops: vec![],
            assert: RuleAssert::Empty
        }
    }
}

impl GuardRule {
    pub fn assert_sized(rule: &GuardRule) -> usize {
        let mut size = 0;
        match &rule.assert {
            RuleAssert::Sized(sized) => {
                size = *sized;
            }
            _ => {}
        }
        size
    }

    pub fn assert_string(rule: &GuardRule) -> String {
        let mut string = "".to_string();
        match &rule.assert {
            RuleAssert::Stringed(str) => {
                string = str.clone();
            }
            _ => {}
        }
        string
    }

    pub fn package_level(rule: &GuardRule) -> (bool, RuleLevel, String) {
        let mut string = "".to_string();
        let mut level = RuleLevel::Package;
        let mut has_capture = false;
        match &rule.assert {
            RuleAssert::Leveled(lv, package_ident) => {
                has_capture = true;
                level = lv.clone();
                string = package_ident.clone();
            }
            _ => {}
        }

        return (has_capture, level, string);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleType {
    Normal,
    Layer,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuleLevel {
    Package,
    Function,
    Class,
    Struct,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuleScope {
    All,
    PathDefine(String),
    Extend(String),
    Assignable(String),
    Implementation(String),
    MatchRegex(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expr {
    PropsCall(Vec<String>),
    Identifier(String)
}

/// A function call, can be a filter or a global function
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FunctionCall {
    /// The name of the function
    pub name: String,
    /// The args of the function: key -> value
    pub args: HashMap<String, Expr>,
}

impl FunctionCall {
    pub fn new(name: String) -> FunctionCall {
        FunctionCall {
            name,
            args: Default::default()
        }
    }
}

impl Default for FunctionCall {
    fn default() -> Self {
        FunctionCall {
            name: "".to_string(),
            args: Default::default()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operator {
    /// >
    Gt,
    /// >=
    Gte,
    /// <
    Lt,
    /// <=
    Lte,
    /// ==
    Eq,
    /// !=
    Ineq,
    /// and
    And,
    /// or
    Or,
    /// !
    /// not
    Not,

    // string assert operator
    StartsWith,
    Endswith,
    Contains,

    // package operators
    Inside,
    ResideIn,
    Accessed,
    DependBy
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuleAssert {
    Empty,
    Stringed(String),
    Leveled(RuleLevel, String),
    ArrayStringed(Vec<String>),
    Sized(usize),
}