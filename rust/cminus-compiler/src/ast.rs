pub type Program = Vec<Stmt>;

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub identifier: String,
    pub sub_arrays: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Binary(Operator, Box<Expr>, Box<Expr>),
    Unary(Operator, Box<Expr>),
    Number(i32),
    Str(String),
    Identifier(String),
    Assignment(Box<Expr>, Box<Expr>),
    FunctionCall(String, Vec<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Block(Vec<Stmt>),
    If(Expr, Box<Stmt>),
    While(Expr, Box<Stmt>),
    For(Expr, Expr, Expr, Box<Stmt>),
    Return(Expr),
    Read(String),
    Write(Expr),
    Expr(Expr),
    Function(String, Vec<Parameter>, Box<Stmt>),
    Declaration(Vec<Parameter>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Ref,
    Deref,
    Positive,
    Negate,
    Not,
    PreIncr,
    PreDecr,
    PostIncr,
    PostDecr,
    ArrayAccess,
    Multiply,
    Divide,
    Add,
    Subtract,
    Mod,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Equal,
    NotEqual,
    And,
    Xor,
    Or,
    LogicalAnd,
    LogicalOr,
}


pub fn push_to_vec<T>(mut params: Vec<T>, param: T) -> Vec<T> {
    params.push(param);
    params
}
