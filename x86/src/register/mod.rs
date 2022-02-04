use crate::operands::{Operand, ToOperand};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Register {
    /// Instruction pointer
    Rip,
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rsi,
    Rdi,
    Rbp,
    Rsp,
    R08,
    R09,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Register::*;
        match self {
            Rip => write!(f, "%rip"),
            Rax => write!(f, "%rax"),
            Rbx => write!(f, "%rbx"),
            Rcx => write!(f, "%rcx"),
            Rdx => write!(f, "%rdx"),
            Rsi => write!(f, "%rsi"),
            Rdi => write!(f, "%rdi"),
            Rbp => write!(f, "%rbp"),
            Rsp => write!(f, "%rsp"),
            R08 => write!(f, "%r8"),
            R09 => write!(f, "%r9"),
            R10 => write!(f, "%r10"),
            R11 => write!(f, "%r11"),
            R12 => write!(f, "%r12"),
            R13 => write!(f, "%r13"),
            R14 => write!(f, "%r14"),
            R15 => write!(f, "%r15"),
        }
    }
}

impl ToOperand for Register {
    fn to_operand(self) -> Operand {
        Operand::Register(self)
    }
}

pub struct ByteRegister(pub Register);

impl std::fmt::Display for ByteRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Register::*;
        match self.0 {
            Rip => panic!("%rip used as byte register"),
            Rax => write!(f, "%al"),
            Rbx => write!(f, "%bl"),
            Rcx => write!(f, "%cl"),
            Rdx => write!(f, "%dl"),
            Rsi => write!(f, "%sil"),
            Rdi => write!(f, "%dil"),
            Rbp => write!(f, "%bpl"),
            Rsp => write!(f, "%spl"),
            R08 => write!(f, "%r8b"),
            R09 => write!(f, "%r9b"),
            R10 => write!(f, "%r10b"),
            R11 => write!(f, "%r11b"),
            R12 => write!(f, "%r12b"),
            R13 => write!(f, "%r13b"),
            R14 => write!(f, "%r14b"),
            R15 => write!(f, "%r15b"),
        }
    }
}
