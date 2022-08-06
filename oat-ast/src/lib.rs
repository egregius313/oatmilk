#[macro_use]
extern crate derive_more;

use indexmap::IndexMap;
use oat_symbol::Symbol;

pub type Id = Symbol;

pub mod macros;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UnaryOp {
    Neg,
    Lognot,
    Bitnot,
}

impl UnaryOp {
    pub const fn op_type(&self) -> (Type, Type) {
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
    pub const fn op_type(&self) -> Option<((Type, Type), Type)> {
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

#[derive(Debug, PartialEq, Eq, Clone, Display)]
pub enum Type {
    #[display(fmt = "bool")]
    Bool,
    #[display(fmt = "int")]
    Int,
    Ref(ReferenceType),
    NullRef(ReferenceType),
}

impl Type {
    pub const fn is_nullable(&self) -> bool {
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

impl std::fmt::Display for ReferenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReferenceType::String => write!(f, "string"),
            ReferenceType::Struct(id) => write!(f, "{}", id.name()),
            ReferenceType::Array(t) => write!(f, "{}[]", t),
            ReferenceType::Function(arg_types, return_type) => {
                write!(f, "(")?;
                arg_types
                    .into_iter()
                    .try_for_each(|t| write!(f, "{}, ", t))?;
                write!(f, ") -> {}", return_type)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Display)]
pub enum ReturnType {
    #[display(fmt = "void")]
    ReturnVoid,
    ReturnValue(Type),
}

#[derive(Debug, PartialEq, Eq, Clone, From)]
pub enum Expression {
    CNull(ReferenceType),
    CBool(bool),
    #[from(types(i32, u32))]
    CInt(i64),
    CStr(String),
    #[from]
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

impl std::ops::Add for Expression {
    type Output = Expression;

    fn add(self, rhs: Expression) -> Expression {
        Expression::Binary {
            op: BinaryOp::Add,
            left: Box::new(self),
            right: Box::new(rhs),
        }
    }
}

impl std::ops::Add<i64> for Expression {
    type Output = Expression;

    fn add(self, rhs: i64) -> Expression {
        self + Expression::CInt(rhs)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn from_i64() {
        assert_eq!(super::Expression::CInt(23_i64), 23_i64.into())
    }

    #[test]
    fn from_i32() {
        assert_eq!(super::Expression::CInt(23_i64), 23_i32.into())
    }
}

impl From<&str> for Expression {
    fn from(id: &str) -> Expression {
        Expression::Id(Symbol::intern(id))
    }
}

#[cfg(test)]
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
    /// ```oat
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
