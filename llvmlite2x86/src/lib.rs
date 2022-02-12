#![allow(unused_imports)]

#[macro_use]
extern crate lazy_static;

use indexmap;

use llvmlite;
use x86;
use x86::Register::*;
use x86::ToOperand;

mod sizeof;

mod errors;

use errors::{Error, Result};

type Layout = indexmap::IndexMap<llvmlite::Uid, x86::Operand>;
// type TypeContext = indexmap::IndexMap<llvmlite::Tid, llvmlite::Type>;

/// Compilation Context
pub struct Context {
    types: llvmlite::TypeContext,
    layout: Layout,
}

impl Context {
    fn new(types: llvmlite::TypeContext, layout: Layout) -> Context {
        Context { types, layout }
    }
}

macro_rules! literal {
    ($x: expr) => {
        x86::Immediate::Literal($x)
    };
}

#[inline]
fn rbp_offset(offset: i64) -> x86::Operand {
    x86::Operand::IndDispReg(literal!(offset), Rbp)
}

const NULL: x86::Operand = x86::Operand::Immediate(x86::Immediate::Literal(0));

// lazy_static! {
//     static ref ARGUMENTS: Vec<x86::Operand> = {
//         use x86::Register::*;
//         vec![Rdi, Rsi, Rdx, Rcx, R08, R09]
//             .iter()
//             .map(|r| r.to_operand())
//             .collect()
//     };
// }

// fn compile_argument(i: usize) -> x86::Operand {
//     ARGUMENTS.get(i)
// }

fn compile_condition(cond: llvmlite::Condition) -> x86::Condition {
    match cond {
        llvmlite::Condition::Eq => x86::Condition::Eq,
        llvmlite::Condition::Ne => x86::Condition::Neq,
        llvmlite::Condition::Slt => x86::Condition::Lt,
        llvmlite::Condition::Sle => x86::Condition::Le,
        llvmlite::Condition::Sge => x86::Condition::Ge,
        llvmlite::Condition::Sgt => x86::Condition::Gt,
    }
}

macro_rules! instruction {
    ($instruction: ident, $($arguments: expr),*) => {{
        use x86::Register::*;
        x86::Instruction {
            opcode: x86::Opcode::$instruction,
            operands: vec![$($arguments.to_operand()),*]
        }
    }}
}

macro_rules! indirect {
    ($label: expr, $reg: ident) => {{
        let label = x86::Immediate::Label($label);
        x86::Operand::IndDispReg(label, x86::Register::$reg)
    }};
}

fn compile_operand(
    context: &Context,
    operand: llvmlite::Operand,
    dest: x86::Operand,
) -> x86::Instruction {
    match operand {
        llvmlite::Operand::Null => instruction!(Movq, NULL, dest),
        llvmlite::Operand::Const(i) => instruction!(Movq, literal!(i), dest),
        llvmlite::Operand::Gid(label) => instruction!(Leaq, indirect!(label, Rip), dest),
        llvmlite::Operand::Id(id) => {
            let value = context.layout.get(&id).unwrap().clone();
            instruction!(Movq, value, dest)
        }
    }
}

fn compile_block(context: &Context, block: llvmlite::Block) -> Result<Vec<x86::Instruction>> {
    todo!("Implement blocks")
}

/// Generate code that computes a pointer value.
///
/// # Arguments
///
/// * `type_` must be a `Ptr` type, so we can
/// * `op` is the base address of the calculation
fn compile_get_element_pointer(
    context: &Context,
    type_: &llvmlite::Type,
    op: &llvmlite::Operand,
    path: &Vec<llvmlite::Operand>,
) -> Result<Vec<x86::Instruction>> {
    // use llvmlite::Type::*;
    // let mut ty = match &*type_ {
    //     Ptr(t) => Array(0, t.clone()),
    //     _ => return Err(Error::DerefNonPointer),
    // };
    // let mut instructions = vec![instruction!(Addq,)];
    // for offset in path {
    //     match ty {
    //         Struct(ref types) => match offset {
    //             llvmlite::Operand::Const(n) => {
    //                 let offset: usize = types
    //                     .iter()
    //                     .take(*n as usize)
    //                     .map(|t| sizeof::sizeof(&context.types, t))
    //                     .sum();
    //                 ty = types[*n as usize];
    //             }
    //         },
    //         Array(_, ref el_type) => {}
    //         Namedt(name) => {}
    //         _ => {
    //             panic!("Get element pointer of non-reference type")
    //         }
    //     }
    // }
    // Ok(instructions)
    todo!("Get element pointer")
}

fn compile_function_declaration(
    context: &Context,
    label: String,
    decl: llvmlite::FunctionDecl,
) -> Result<x86::AsmBlock> {
    let mut instructions: Vec<x86::Instruction> = vec![];
    // let tmpsize: i64 = todo!("implement tmpsize");
    let tmpsize = 8 * context.layout.len();
    let prologue = vec![
        instruction!(Pushq, Rbp),
        instruction!(Movq, Rsp, Rbp),
        instruction!(Subq, tmpsize as i64, Rsp),
    ];
    // x86::AsmBlock::Text(instructions)
    // todo!("Make the blocks")
    //
    Ok(x86::AsmBlock {
        label,
        global: true,
        asm: x86::AsmContent::Text(instructions),
    })
}

fn compile_global_declaration(
    context: &Context,
    decl: llvmlite::GlobalDeclaration,
) -> Result<Vec<x86::Data>> {
    let llvmlite::GlobalDeclaration(_, initializer) = decl;
    use llvmlite::GlobalInitializer::*;
    use x86::Data::*;
    Ok(match initializer {
        Null => vec![literal!(0).into()],
        Gid(id) => vec![x86::Immediate::Label(id).into()],
        Int(i) => vec![literal!(i).into()],
        String(s) => vec![Asciz(s)],
        Array(_initializers) | Struct(_initializers) => todo!("Global arrays and structs"),
        Bitcast(_, _, _) => todo!("Global bitcasts"),
    })
}

pub fn compile_program(context: &Context, program: &llvmlite::Program) -> Result<x86::Program> {
    Ok(x86::Program { blocks: vec![] })
}
