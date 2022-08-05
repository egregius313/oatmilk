//! A representation of a subset of x86
#[macro_use]
extern crate derive_more;

mod operands;
pub use crate::operands::{ByteOperand, JumpOperand, Operand, ToOperand};
mod register;
pub use crate::register::*;

// use std::fmt;
// use std::fmt::{Display, Formatter};

/// Labels for code and globals
pub type Label = String;

/// A quad word
pub type Quad = i64;

/// Immediates
#[derive(Clone, PartialEq, Eq, Display)]
pub enum Immediate {
    Literal(Quad),
    Label(Label),
}

impl Immediate {
    pub fn displacement(self) -> Operand {
        Operand::IndDisp(self)
    }
}

impl ToOperand for Immediate {
    fn to_operand(self) -> Operand {
        Operand::Immediate(self)
    }
}

trait Indirect<I> {
    fn indirect(self, i: I) -> Operand;
}

impl Indirect<Immediate> for Register {
    fn indirect(self, i: Immediate) -> Operand {
        Operand::IndDispReg(i, self)
    }
}

impl Indirect<Quad> for Register {
    fn indirect(self, i: Quad) -> Operand {
        Operand::IndDispReg(Immediate::Literal(i), self)
    }
}

impl Indirect<Label> for Register {
    fn indirect(self, i: Label) -> Operand {
        Operand::IndDispReg(Immediate::Label(i), self)
    }
}

#[derive(Copy, Clone, Display)]
pub enum Condition {
    #[display(fmt = "e")]
    Eq,
    #[display(fmt = "ne")]
    Neq,
    #[display(fmt = "g")]
    Gt,
    #[display(fmt = "ge")]
    Ge,
    #[display(fmt = "l")]
    Lt,
    #[display(fmt = "le")]
    Le,
}

#[derive(Clone, Copy)]
pub enum Opcode {
    Movq,
    Pushq,
    Popq,
    Leaq,
    Incq,
    Decq,
    Negq,
    Notq,
    Addq,
    Subq,
    Imulq,
    Xorq,
    Orq,
    Andq,
    Shlq,
    Sarq,
    Shrq,
    Jmp,
    J(Condition),
    Cmpq,
    Set(Condition),
    Callq,
    Retq,
}

impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Opcode::*;
        match self {
            Movq => write!(f, "movq"),
            Pushq => write!(f, "pushq"),
            Popq => write!(f, "popq"),
            Leaq => write!(f, "leaq"),
            Incq => write!(f, "incq"),
            Decq => write!(f, "decq"),
            Negq => write!(f, "negq"),
            Notq => write!(f, "notq"),
            Addq => write!(f, "addq"),
            Subq => write!(f, "subq"),
            Imulq => write!(f, "imulq"),
            Xorq => write!(f, "xorq"),
            Orq => write!(f, "orq"),
            Andq => write!(f, "andq"),
            Shlq => write!(f, "shlq"),
            Sarq => write!(f, "sarq"),
            Shrq => write!(f, "shrq"),
            Jmp => write!(f, "jmp"),
            J(cnd) => write!(f, "j{}", cnd),
            Cmpq => write!(f, "cmpq"),
            Set(cnd) => write!(f, "set{}", cnd),
            Callq => write!(f, "callq"),
            Retq => write!(f, "retq"),
        }
    }
}

pub struct Instruction {
    pub opcode: Opcode,
    pub operands: Vec<Operand>,
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Opcode::*;

        let Instruction { opcode, operands } = self;
        write!(f, "\t{}\t", opcode)?;
        match opcode {
            J(_) | Jmp | Callq => {
                let op = operands[0].clone();
                write!(f, "{}", JumpOperand(op))?;
            }
            Set(_) => {
                let op = operands[0].clone();
                write!(f, "{}", ByteOperand(op))?;
            }
            _ => {
                let s = operands
                    .iter()
                    .map(|op| format!("{}", op))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{}", s)?;
            }
        }
        Ok(())
    }
}

#[derive(From, PartialEq, Display)]
pub enum Data {
    #[display(fmt = "\t.asciz\t\"{}\"", "_0.escape_default()")]
    Asciz(String),
    #[display(fmt = "\t.quad\t{}", _0)]
    Quad(Immediate),
}

pub enum AsmContent {
    Text(Vec<Instruction>),
    Data(Vec<Data>),
}

impl std::fmt::Display for AsmContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use AsmContent::*;
        match self {
            Text(instructions) => {
                write!(f, "\t.text\n")?;
                for instruction in instructions {
                    write!(f, "{}\n", instruction)?;
                }
                Ok(())
            }
            Data(data) => {
                write!(f, "\t.data\n")?;
                for datum in data {
                    write!(f, "{}\n", datum)?;
                }
                Ok(())
            }
        }
    }
}

pub struct AsmBlock {
    pub label: Label,
    pub global: bool,
    pub asm: AsmContent,
}

impl std::fmt::Display for AsmBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use AsmContent::*;
        match &self.asm {
            Text(instructions) => {
                write!(f, "\t.text\n")?;
                if self.global {
                    write!(f, "\t.globl\t{}\n", self.label)?;
                }
                write!(f, "{}:\n", self.label)?;
                for instruction in instructions {
                    write!(f, "{}\n", instruction)?;
                }
                Ok(())
            }
            Data(data) => {
                write!(f, "\t.data\n")?;
                if self.global {
                    write!(f, "\t.globl\t{}\n", self.label)?;
                }
                write!(f, "{}:\n", self.label)?;
                for datum in data {
                    write!(f, "{}\n", datum)?;
                }
                Ok(())
            }
        }
    }
}

#[derive(From)]
pub struct Program {
    pub blocks: Vec<AsmBlock>,
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for block in &self.blocks {
            write!(f, "{}", block)?;
        }
        Ok(())
    }
}
