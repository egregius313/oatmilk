use indexmap::IndexMap;
use oat_symbol::Symbol;

pub type Id = Symbol;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UnaryOp {
    Neg,
    Lognot,
    Bitnot,
}

impl UnaryOp {
    pub fn op_type(&self) -> (Type, Type) {
        use UnaryOp::*;
        match self {
            Neg | Bitnot => (Type::Int, Type::Int),
            Lognot => (Type::Bool, Type::Bool),
        }
    }
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

impl BinaryOp {
    /// The type associated with a binary operation. Returns `Some((left,
    /// right), output)` if there is a specific type for the operation, or
    /// `None` if there is no single type (e.g. equality is polymorphic).
    pub fn op_type(&self) -> Option<((Type, Type), Type)> {
        use BinaryOp::*;
        match self {
            Add | Mul | Sub | Shl | Shr | Sar | IAnd | IOr => {
                Some(((Type::Int, Type::Int), Type::Int))
            }
            Lt | Lte | Gt | Gte => Some(((Type::Int, Type::Int), Type::Bool)),
            And | Or => Some(((Type::Bool, Type::Bool), Type::Bool)),
            Eq | Neq => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Bool,
    Int,
    Ref(ReferenceType),
    NullRef(ReferenceType),
}

impl Type {
    pub fn is_nullable(self) -> bool {
        matches!(self, Type::NullRef(_))
    }
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

macro_rules! impl_expression_from_i64 {
    ($t: ty) => {
        impl From<$t> for Expression {
            fn from(v: $t) -> Expression {
                let i: i64 = v.into();
                Expression::CInt(i)
            }
        }
    };
}

impl_expression_from_i64!(i32);
impl_expression_from_i64!(i64);

impl From<&str> for Expression {
    fn from(id: &str) -> Expression {
        Expression::Id(Symbol::intern(id))
    }
}

impl From<String> for Expression {
    fn from(id: String) -> Expression {
        Expression::Id(Symbol::intern(id.as_str()))
    }
}

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
