//! AST (Abstract Syntax Tree) definitions for the Aura language.

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub name: Option<String>,
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Function(FunctionDecl),
    TypeAlias(TypeAliasDecl),
    SumType(SumTypeDecl),
    Interface(InterfaceDecl),
    Import(ImportDecl),
    Extern(ExternDecl),
    Statement(Statement),
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceDecl {
    pub name: String,
    pub is_exported: bool,
    pub type_params: Vec<String>,
    pub methods: Vec<InterfaceMethod>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceMethod {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Receiver {
    pub name: String,
    pub target_type: Type,
}

impl std::fmt::Display for Receiver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.target_type)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecl {
    pub name: String,
    pub receiver: Option<Receiver>,
    pub is_async: bool,
    pub is_exported: bool,
    pub type_params: Vec<String>,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub body: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub name: String,
    pub type_annotation: Option<Type>,
    pub default_value: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAliasDecl {
    pub name: String,
    pub is_exported: bool,
    pub is_packed: bool,
    pub type_params: Vec<String>,
    pub target: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SumTypeDecl {
    pub name: String,
    pub is_exported: bool,
    pub type_params: Vec<String>,
    pub variants: Vec<Variant>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variant {
    pub name: String,
    pub fields: VariantFields,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariantFields {
    Unit,
    Tuple(Vec<Type>),
    Record(Vec<(String, Type)>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportDecl {
    pub source: String,
    pub items: Vec<ImportItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportItem {
    pub name: String,
    pub alias: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExternDecl {
    pub module_name: String,
    pub functions: Vec<ExternFunction>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExternFunction {
    pub name: String,
    pub type_params: Vec<String>,
    pub params: Vec<Param>,
    pub return_type: Type,
}

// -----------------------------------------------------------------------------
// TYPES
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Named {
        name: String,
        type_args: Vec<Type>,
    },
    Record(Vec<(String, Type)>),
    Tuple(Vec<Type>),
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },
    Pointer(Box<Type>),
    SendChannel(Box<Type>),
    RecvChannel(Box<Type>),
    Unit,
    TypeVar(String),
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Named { name, type_args } => {
                if type_args.is_empty() {
                    write!(f, "{}", name)
                } else {
                    let args: Vec<String> = type_args.iter().map(|t| format!("{}", t)).collect();
                    write!(f, "{}<{}>", name, args.join(", "))
                }
            }
            Type::Record(fields) => {
                let fs: Vec<String> = fields
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect();
                write!(f, "{{ {} }}", fs.join(", "))
            }
            Type::Tuple(elems) => {
                let es: Vec<String> = elems.iter().map(|e| format!("{}", e)).collect();
                write!(f, "({})", es.join(", "))
            }
            Type::Function {
                params,
                return_type,
            } => {
                let ps: Vec<String> = params.iter().map(|p| format!("{}", p)).collect();
                write!(f, "fn({}) -> {}", ps.join(", "), return_type)
            }
            Type::Pointer(inner) => write!(f, "*{}", inner),
            Type::SendChannel(inner) => write!(f, "chan<- {}", inner),
            Type::RecvChannel(inner) => write!(f, "<-chan {}", inner),
            Type::Unit => write!(f, "()"),
            Type::TypeVar(tv) => write!(f, "{}", tv),
        }
    }
}

// -----------------------------------------------------------------------------
// STATEMENTS
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let {
        name: String,
        is_mut: bool,
        type_annotation: Option<Type>,
        value: Expr,
    },
    LetPattern {
        pattern: Pattern,
        type_annotation: Option<Type>,
        value: Expr,
    },
    Assign {
        target: Expr,
        value: Expr,
    },
    Defer(Box<Expr>),
    ErrDefer(Box<Expr>),
    Expr(Expr),
    Return(Option<Expr>),
}

// -----------------------------------------------------------------------------
// EXPRESSIONS
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Identifier(String),
    Binary {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Unary {
        op: UnOp,
        expr: Box<Expr>,
    },
    Pipeline {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    FunctionCall {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    MemberAccess {
        object: Box<Expr>,
        member: String,
    },
    IndexAccess {
        object: Box<Expr>,
        index: Box<Expr>,
    },
    SliceAccess {
        object: Box<Expr>,
        low: Option<Box<Expr>>,
        high: Option<Box<Expr>>,
        max: Option<Box<Expr>>,
    },
    RecordLiteral {
        fields: Vec<(String, Expr)>,
        spread: Option<Box<Expr>>,
    },
    TupleLiteral(Vec<Expr>),
    ListLiteral(Vec<Expr>),
    Lambda {
        params: Vec<Param>,
        return_type: Option<Type>,
        body: Box<Expr>,
    },
    Match {
        subject: Box<Expr>,
        arms: Vec<MatchArm>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
    },
    Block(Vec<Statement>),
    Async(Box<Expr>),
    Await(Box<Expr>),
    Spawn(Box<Expr>),
    ChanSend {
        channel: Box<Expr>,
        value: Box<Expr>,
    },
    ChanRecv(Box<Expr>),
    Select {
        arms: Vec<SelectArm>,
        default: Option<Box<Expr>>,
    },
    ConstructorCall {
        name: String,
        args: Vec<Expr>,
    },
    Try(Box<Expr>),
    While {
        label: Option<String>,
        condition: Box<Expr>,
        body: Box<Expr>,
    },
    ForIn {
        label: Option<String>,
        index_name: Option<String>,
        var_name: String,
        iterable: Box<Expr>,
        body: Box<Expr>,
    },
    Break(Option<String>),
    Continue(Option<String>),
    Embed {
        path: String,
        is_binary: bool,
    },
    AddressOf(Box<Expr>),
    Deref(Box<Expr>),
    Panic(Box<Expr>),
    Recover,
    MapLiteral(Vec<(Expr, Expr)>),
    SetLiteral(Vec<Expr>),
    Placeholder,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    TemplateString(Vec<TemplateSegment>),
    Bool(bool),
    Unit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TemplateSegment {
    Text(String),
    Expr(Expr),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Equal,
    NotEqual,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    And,
    Or,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnOp {
    Negate,
    Not,
}

// -----------------------------------------------------------------------------
// CONCURRENCY (CSP)
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct SelectArm {
    pub kind: SelectArmKind,
    pub body: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SelectArmKind {
    Recv {
        binding: Option<String>,
        channel: Expr,
    },
    Send {
        channel: Expr,
        value: Expr,
    },
    Timeout(Expr),
}

// -----------------------------------------------------------------------------
// PATTERN MATCHING
// -----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Expr>,
    pub body: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Wildcard,
    Literal(Literal),
    Variable(String),
    Constructor {
        name: String,
        patterns: Vec<Pattern>,
    },
    Record {
        type_name: Option<String>,
        fields: Vec<(String, Pattern)>,
        has_rest: bool,
    },
    Tuple(Vec<Pattern>),
    List {
        items: Vec<Pattern>,
        rest: Option<Box<Pattern>>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_module_and_items_creation() {
        let module = Module {
            name: Some("test_mod".to_string()),
            items: vec![
                Item::TypeAlias(TypeAliasDecl {
                    name: "Id".to_string(),
                    is_exported: true,
                    is_packed: false,
                    type_params: vec![],
                    target: Type::Named {
                        name: "Int".to_string(),
                        type_args: vec![],
                    },
                }),
                Item::SumType(SumTypeDecl {
                    name: "Result".to_string(),
                    is_exported: true,
                    type_params: vec!["T".to_string(), "E".to_string()],
                    variants: vec![
                        Variant {
                            name: "Ok".to_string(),
                            fields: VariantFields::Tuple(vec![Type::TypeVar("T".to_string())]),
                        },
                        Variant {
                            name: "Err".to_string(),
                            fields: VariantFields::Tuple(vec![Type::TypeVar("E".to_string())]),
                        },
                    ],
                }),
            ],
        };

        assert_eq!(module.name, Some("test_mod".to_string()));
        assert_eq!(module.items.len(), 2);
    }

    #[test]
    fn test_ast_expressions_and_statements() {
        let let_stmt = Statement::Let {
            name: "x".to_string(),
            is_mut: false,
            type_annotation: Some(Type::Named {
                name: "Int".to_string(),
                type_args: vec![],
            }),
            value: Expr::Literal(Literal::Int(42)),
        };

        let binary_expr = Expr::Binary {
            op: BinOp::Add,
            left: Box::new(Expr::Identifier("x".to_string())),
            right: Box::new(Expr::Literal(Literal::Int(1))),
        };

        assert_eq!(
            let_stmt,
            Statement::Let {
                name: "x".to_string(),
                is_mut: false,
                type_annotation: Some(Type::Named {
                    name: "Int".to_string(),
                    type_args: vec![],
                }),
                value: Expr::Literal(Literal::Int(42)),
            }
        );

        if let Expr::Binary { op, .. } = binary_expr {
            assert_eq!(op, BinOp::Add);
        } else {
            panic!("Expected binary expr");
        }
    }

    #[test]
    fn test_ast_pattern_structures() {
        let pat_wildcard = Pattern::Wildcard;
        let pat_tuple = Pattern::Tuple(vec![
            Pattern::Literal(Literal::Int(1)),
            Pattern::Variable("y".to_string()),
        ]);
        let pat_record = Pattern::Record {
            type_name: Some("User".to_string()),
            fields: vec![("name".to_string(), Pattern::Variable("n".to_string()))],
            has_rest: true,
        };

        assert_eq!(pat_wildcard, Pattern::Wildcard);
        if let Pattern::Tuple(items) = pat_tuple {
            assert_eq!(items.len(), 2);
        }
        if let Pattern::Record { has_rest, .. } = pat_record {
            assert!(has_rest);
        }
    }
}
