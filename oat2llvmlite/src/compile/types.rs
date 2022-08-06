use crate::{
    context::{Context, TypingContext},
    Compile,
};

use llvmlite as llvm;
use oat_ast as oat;

impl Compile<llvm::Type> for oat::ReturnType {
    fn compile(self, context: &Context, type_context: &TypingContext) -> llvm::Type {
        match self {
            oat::ReturnType::ReturnVoid => llvm::Type::Void,
            oat::ReturnType::ReturnValue(t) => t.compile(&context, &type_context),
        }
    }
}

impl Compile<llvm::Type> for oat::Type {
    fn compile(self, context: &Context, type_context: &TypingContext) -> llvm::Type {
        match self {
            oat::Type::Bool => llvm::Type::I1,
            oat::Type::Int => llvm::Type::I64,
            oat::Type::Ref(rt) | oat::Type::NullRef(rt) => llvm::Type::Ptr(Box::new(match rt {
                oat::ReferenceType::String => llvm::Type::I8,
                oat::ReferenceType::Struct(id) => llvm::Type::Namedt(id),
                oat::ReferenceType::Array(rt) => llvm::Type::Struct(vec![
                    llvm::Type::I64,
                    llvm::Type::Array(0, Box::new(rt.compile(&context, &type_context))),
                ]),
                oat::ReferenceType::Function(arg_types, ret_type) => llvm::Type::Fun(
                    arg_types
                        .into_iter()
                        .map(|t| t.compile(&context, &type_context))
                        .collect(),
                    Box::new(ret_type.compile(&context, &type_context)),
                ),
            })),
            // _ => panic!("Cannot represent {:?}", self),
        }
    }
}
