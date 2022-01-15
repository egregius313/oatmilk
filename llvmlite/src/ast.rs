//! LLVMLite is represented internally as an AST, representing the subset of
//! LLVM.

use indexmap::IndexMap;

/// Type identifiers
pub type Tid = String;
/// Global identifiers
pub type Gid = String;
/// Local identifiers
pub type Uid = String;
/// Labels for functions and constants
pub type Label = String;

/// LLVM types
#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Void,
    I1,
    I8,
    I64,
    Ptr(Box<Type>),
    Struct(Vec<Type>),
    Array(usize, Box<Type>),
    Fun(Vec<Type>, Box<Type>),
    Namedt(Tid),
}

/// The type signature of a function
#[derive(Debug, PartialEq, Eq)]
pub struct FunctionType {
    pub arg_types: Vec<Type>,
    pub ret_type: Type,
}

/// Syntactic Values
#[derive(Debug, PartialEq, Eq)]
pub enum Operand {
    Null,
    Const(i64),
    Gid(Gid),
    Id(Uid),
}

impl PartialOrd for Operand {
    fn partial_cmp(&self, other: &Operand) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering::*;
        use Operand::*;
        match (self, other) {
            (Null, Null) => Some(Equal),
            (Const(x), Const(y)) => x.partial_cmp(y),
            (_, _) => None,
        }
    }
}

/// Binary i64 operations
#[derive(Debug, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Shl,
    Lshr,
    Ashr,
    And,
    Or,
    Xor,
}

/// Comparison Operators
#[derive(Debug, PartialEq, Eq)]
pub enum Condition {
    Eq,
    Ne,
    Slt,
    Sle,
    Sge,
}

/// Non-terminating instructions
#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Binop(BinaryOperator, Type, Operand, Operand),
    Alloca(Type),
    Load(Type, Operand),
    Store(Type, Operand, Operand),
    Icmp(Condition, Type, Operand, Operand),
    Call(Type, Operand, Vec<(Type, Operand)>),
    Bitcast(Type, Operand, Type),
    Gep(Type, Operand, Vec<Operand>),
}

/// Terminators of a block
#[derive(Debug, PartialEq, Eq)]
pub enum Terminator {
    Ret(Type, Option<Operand>),
    Break(Label),
    CondBreak(Operand, Label, Label),
}

/// Blocks
#[derive(Debug, PartialEq, Eq)]
pub struct Block {
    pub instructions: Vec<(Uid, Instruction)>,
    pub terminator: (Uid, Terminator),
}

/// Control flow graphs
#[derive(Debug, PartialEq, Eq)]
pub struct ControlFlowGraph {
    pub entry: Block,
    pub blocks: IndexMap<Label, Block>,
}

/// Function declarations
#[derive(Debug, PartialEq, Eq)]
pub struct FunctionDecl {
    pub type_signature: FunctionType,
    pub parameters: Vec<Uid>,
    pub cfg: ControlFlowGraph,
}

/// The initializers for a global definition in LLVMLite.
///
/// Global definitions are more limited than regular expressions.
#[derive(Debug, PartialEq, Eq)]
pub enum GlobalInitializer {
    Null,
    Gid(Gid),
    Int(i64),
    String(String),
    Array(Vec<(Type, GlobalInitializer)>),
    Struct(Vec<(Type, GlobalInitializer)>),
    Bitcast(Type, Box<GlobalInitializer>, Type),
}

/// Top-level definitions for values
#[derive(Debug, PartialEq, Eq)]
pub struct GlobalDeclaration(Type, GlobalInitializer);

/// An LLVMLite Program
#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    pub types: IndexMap<Tid, Type>,
    pub globals: IndexMap<Gid, GlobalDeclaration>,
    pub functions: IndexMap<Gid, GlobalDeclaration>,
    pub externals: IndexMap<Gid, Type>,
}
