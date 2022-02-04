use super::Operand;

pub struct JumpOperand(pub Operand);

impl std::fmt::Display for JumpOperand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Operand::*;
        match &self.0 {
            i @ Immediate(_) => write!(f, "{}", i),
            Register(r) => write!(f, "*{}", r),
            IndDisp(i) => write!(f, "*{}", i),
            IndReg(r) => write!(f, "*({})", r),
            IndDispReg(i, r) => write!(f, "*{}({})", i, r),
        }
    }
}
