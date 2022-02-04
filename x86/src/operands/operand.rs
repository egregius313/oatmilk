use crate::operands::ToOperand;
use crate::{Immediate, Register};

#[derive(Clone, PartialEq)]
pub enum Operand {
    Immediate(Immediate),
    Register(Register),
    /// Indirect by displacement
    IndDisp(Immediate),
    /// Indirect by register: (%reg)
    IndReg(Register),
    /// Indirect displacment(%reg)
    IndDispReg(Immediate, Register),
}

impl ToOperand for Operand {
    fn to_operand(self) -> Operand {
        self
    }
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Operand::*;
        match self {
            Immediate(i) => write!(f, "${}", i),
            Register(r) => write!(f, "{}", r),
            IndDisp(i) => write!(f, "{}", i),
            IndReg(r) => write!(f, "({})", r),
            IndDispReg(i, r) => write!(f, "{}({})", i, r),
        }
    }
}
