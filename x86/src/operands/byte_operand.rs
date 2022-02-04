use super::Operand;
use crate::register::ByteRegister;

#[derive(PartialEq)]
pub struct ByteOperand(pub Operand);

impl std::fmt::Display for ByteOperand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Operand::*;
        match &self.0 {
            Immediate(i) => write!(f, "${}", i),
            Register(r) => write!(f, "{}", ByteRegister(*r)),
            IndDisp(i) => write!(f, "{}", i),
            IndReg(r) => write!(f, "({})", r),
            IndDispReg(i, r) => write!(f, "{}({})", i, r),
        }
    }
}
