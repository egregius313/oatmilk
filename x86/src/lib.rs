//! A representation of a subset of x86

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
#[derive(Clone, PartialEq, Eq)]
pub enum Immediate {
    Literal(Quad),
    Label(Label),
}

impl From<Immediate> for Data {
    fn from(im: Immediate) -> Data {
        Data::Quad(im)
    }
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

impl std::fmt::Display for Immediate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Immediate::*;
        match self {
            Literal(x) => write!(f, "{}", x),
            Label(lbl) => write!(f, "{}", lbl),
        }
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

#[derive(Copy, Clone)]
pub enum Condition {
    Eq,
    Neq,
    Gt,
    Ge,
    Lt,
    Le,
}

impl std::fmt::Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Condition::*;
        match self {
            Eq => write!(f, "e"),
            Neq => write!(f, "ne"),
            Gt => write!(f, "g"),
            Ge => write!(f, "ge"),
            Lt => write!(f, "l"),
            Le => write!(f, "le"),
        }
    }
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

pub enum Data {
    Asciz(String),
    Quad(Immediate),
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Data::*;
        match self {
            Asciz(s) => write!(f, "\t.asciz\t\"{}\"", s.escape_default()),
            Quad(i) => write!(f, "\t.quad\t{}", i),
        }
    }
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
