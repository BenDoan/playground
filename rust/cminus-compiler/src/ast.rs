pub type Program = Vec<Meta<Stmt>>;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Meta<T> {
    pub inside: T,
    pub byte_offset: usize,
    pub line_num: Option<u32>,
}

impl<T> Meta<T> {
    pub fn new(inside: T, byte_offset: usize) -> Meta<T> {
        Meta {
            inside: inside,
            byte_offset: byte_offset,
            line_num: None,
        }
    }

    pub fn get_line(&self) -> usize {
        self.byte_offset
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Parameter {
    pub identifier: String,
    pub sub_arrays: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Expr {
    Binary(Operator, Box<Meta<Expr>>, Box<Meta<Expr>>),
    Unary(Operator, Box<Meta<Expr>>),
    Number(i32),
    Str(String),
    Identifier(String),
    Assignment(Box<Meta<Expr>>, Box<Meta<Expr>>),
    FunctionCall(String, Vec<Meta<Expr>>),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Stmt {
    Block(Vec<Meta<Stmt>>),
    If(Meta<Expr>, Box<Meta<Stmt>>),
    While(Meta<Expr>, Box<Meta<Stmt>>),
    For(Meta<Expr>, Meta<Expr>, Meta<Expr>, Box<Meta<Stmt>>),
    Return(Meta<Expr>),
    Read(String),
    Write(Meta<Expr>),
    Expr(Meta<Expr>),
    Function(String, Vec<Parameter>, Box<Meta<Stmt>>),
    Declaration(Vec<Parameter>),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
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
