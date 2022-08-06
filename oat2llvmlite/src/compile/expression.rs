use crate::{
    context::{Context, TypingContext},
    Compile,
};

use llvmlite as llvm;
use llvmlite::{Instruction, Operand, Type as LLType};
use oat_ast as oat;

impl Compile<(LLType, Operand, Vec<Instruction>)> for oat::Expression {
    fn compile(
        self,
        context: &Context,
        type_context: &TypingContext,
    ) -> (LLType, Operand, Vec<Instruction>) {
        use oat::Expression::*;
        match self {
            CInt(i) => (LLType::I64, Operand::Const(i), vec![]),
            CBool(b) => (LLType::I1, Const(b as i64), vec![]),
            CNull(rt) => {
                let t = oat::Type::ReferenceType(rt).compile(&context, &type_context);
                (t, Operand::Null, vec![])
            }
            _ => todo!("Implement {:?} expression", self),
        }
    }
}
