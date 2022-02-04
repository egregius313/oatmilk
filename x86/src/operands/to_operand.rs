use crate::{Immediate, Label, Operand, Quad};

/// Helper trait for functions and macros to use to reduce boilerplate when
/// encoding x86. Instructions have operands, which are often simple things like
/// immediates (specified by integer literals) or registers.
pub trait ToOperand {
    fn to_operand(self) -> Operand;
}

impl ToOperand for Quad {
    fn to_operand(self) -> Operand {
        Operand::Immediate(Immediate::Literal(self))
    }
}

impl ToOperand for Label {
    fn to_operand(self) -> Operand {
        Operand::Immediate(Immediate::Label(self))
    }
}
