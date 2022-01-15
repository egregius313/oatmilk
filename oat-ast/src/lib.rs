pub type Id = String;

pub enum UnaryOp {
    Neg,
    Lognot,
    Bitnot,
}

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

pub enum Type {
    Bool,
    Int,
    Ref(ReferenceType),
    NullRef(ReferenceType),
}

pub enum ReferenceType {
    String,
    Struct(Id),
    Array(Box<Type>),
    Function(Vec<Box<Type>>, Box<ReturnType>),
}

pub enum ReturnType {
    ReturnVoid,
    ReturnValue(Box<Type>),
}

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

pub enum Statement {
    Assignment(Expression, Expression),
    Declaration(Id, Expression),
    SCall(Expression, Vec<Expression>),
    If {
        condition: Expression,
        then: Vec<Statement>,
        else_: Vec<Statement>,
    },
    /// Statement for casting nullable expressions into the non-null values
    ///
    /// ## Example:
    /// if? (string s = str) {
    ///     write(s);
    /// } else {
    ///     /* str was null */
    ///     write("nothing");
    /// }
    Cast(
        ReferenceType,
        Id,
        Expression,
        Vec<Statement>,
        Vec<Statement>,
    ),
    For {
        init: Vec<(Id, Expression)>,
        condition: Option<Expression>,
        update: Option<Box<Statement>>,
        body: Vec<Statement>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    Return(Option<Expression>),
}
