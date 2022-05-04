use indexmap::IndexMap;
use oat_symbol::Symbol;

pub type Id = Symbol;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UnaryOp {
    Neg,
    Lognot,
    Bitnot,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    And,
    Or,
    IAnd,
    IOr,
    Shl,
    Shr,
    Sar,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Bool,
    Int,
    Ref(ReferenceType),
    NullRef(ReferenceType),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ReferenceType {
    String,
    Struct(Id),
    Array(Box<Type>),
    Function(Vec<Type>, Box<ReturnType>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ReturnType {
    ReturnVoid,
    ReturnValue(Type),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    CNull(ReferenceType),
    CBool(bool),
    CInt(i64),
    CStr(String),
    Id(Id),
    CArr(Type, Vec<Expression>),
    NewArr(Type, Box<Expression>),
    // NewArrInit of ty * exp node * id * exp node,
    Index {
        value: Box<Expression>,
        index: Box<Expression>,
    },
    Length(Box<Expression>),
    CStruct(Id, Vec<(Id, Expression)>),
    Proj(Box<Expression>, Id),
    Call(Box<Expression>, Vec<Expression>),
    Binary {
        op: BinaryOp,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Unary(UnaryOp, Box<Expression>),
}

impl<T: Into<i64>> From<T> for Expression {
    fn from(v: T) -> Expression {
        let i: i64 = v.into();
        Expression::CInt(i)
    }
}

// impl From<&str> for Expression {
//     fn from(v: &str) -> Expression {
//         Expression::Id(String::from(v))
//     }
// }

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Statement {
    Assignment(Expression, Expression),
    Declaration(Id, Expression),
    SCall(Expression, Vec<Expression>),
    If {
        condition: Expression,
        then: Block,
        else_: Block,
    },
    /// Statement for casting nullable expressions into the non-null values
    ///
    /// ## Example:
    /// ```
    /// if? (string s = str) {
    ///     write(s);
    /// } else {
    ///     /* str was null */
    ///     write("nothing");
    /// }
    /// ```
    Cast(ReferenceType, Id, Expression, Block, Block),
    /// Represents
    ///
    /// ```c
    /// for (init; condition; update) {
    ///     body;
    /// }
    /// ```
    For {
        init: Vec<(Id, Expression)>,
        condition: Option<Expression>,
        update: Option<Box<Statement>>,
        body: Block,
    },
    While {
        condition: Expression,
        body: Block,
    },
    Return(Option<Expression>),
}

pub type Block = Vec<Statement>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GlobalDeclaration {
    pub name: Id,
    pub init: Expression,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FunctionDecl {
    pub return_type: ReturnType,
    pub name: Id,
    pub args: Vec<(Type, Id)>,
    pub body: Block,
}

// #[derive(Debug, PartialEq)]
// pub struct Field {
//     pub name: Id,
//     pub field_type: Type,
// }

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct TypeDeclaration {
    pub name: Id,
    pub fields: IndexMap<Id, Type>,
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum Declaration {
    Variable(GlobalDeclaration),
    Function(FunctionDecl),
    Type(TypeDeclaration),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Program {
    pub declarations: Vec<Declaration>,
}
