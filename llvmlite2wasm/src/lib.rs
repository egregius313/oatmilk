// TODO: Remove after compilation completed
#![allow(unused_imports)]

use llvmlite::ast as llvm;
use wasm_encoder::{CodeSection, Function, Instruction, Module, ValType};

mod errors;
pub use errors::{Error, Result};

#[derive(Clone)]
enum Type {
    Void,
    Value(ValType),
    Function(Vec<ValType>, Vec<ValType>),
}

impl Type {
    fn value_type(self) -> ValType {
        match self {
            Type::Value(v) => v,
            _ => panic!("Cannot have non-value types in functions"),
        }
    }
}

/// Determine the type of something in WebAssembly
trait WasmTypable {
    fn wasm_type(&self) -> Type;
}

impl WasmTypable for llvm::FunctionType {
    fn wasm_type(&self) -> Type {
        let ret_type = match self.ret_type.wasm_type() {
            Type::Void => vec![],
            Type::Value(v) => vec![v],
            _ => todo!("Backend: complete return type implementation"),
        };
        // let mut arg_types = vec![];
        // for at in self.arg_types.iter() {
        //     arg_types.push(at.wasm_type().value_type())
        // }
        Type::Function(
            self.arg_types
                .iter()
                .map(|t| t.clone().wasm_type().value_type())
                .collect(),
            ret_type,
        )
    }
}

impl WasmTypable for llvm::Type {
    fn wasm_type(&self) -> Type {
        match self {
            llvm::Type::Void => Type::Void,
            llvm::Type::I1 => Type::Value(ValType::I32),
            llvm::Type::I8 => Type::Value(ValType::I32),
            llvm::Type::I64 => Type::Value(ValType::I64),
            llvm::Type::Fun(f_arg_types, f_ret_type) => {
                let ret_type = match f_ret_type.wasm_type() {
                    Type::Void => vec![],
                    Type::Value(v) => vec![v],
                    _ => todo!("Backend: complete return type implementation"),
                };
                Type::Function(
                    f_arg_types
                        .iter()
                        .map(|t| t.clone().wasm_type().value_type())
                        .collect(),
                    ret_type,
                )
            }
            llvm::Type::Ptr(_) => todo!("Backend: Implement pointers"),
            llvm::Type::Struct(_) => todo!("Backend: Implement structs"),
            llvm::Type::Array(_, _) => todo!("Backend: Implement arrays"),
            _ => todo!("Currently only integers, booleans, and functions are implemented"),
        }
    }
}

impl WasmTypable for llvm::FunctionDecl {
    fn wasm_type(&self) -> Type {
        let llvm::FunctionDecl { type_signature, .. } = self;

        type_signature.wasm_type()
    }
}

/// Compilation target
///
/// `Target` represents what type to compile to.
trait Compile<Target> {
    fn compile(&self) -> Result<Target>;
}

impl Compile<Function> for llvm::FunctionDecl {
    fn compile(&self) -> Result<Function> {
        let locals = vec![];
        let mut function = Function::new(locals);

        Ok(function)
    }
}

impl Compile<Module> for llvm::Program {
    fn compile(&self) -> Result<Module> {
        let mut module = Module::new();

        let mut code_section = CodeSection::new();

        for function in &self.functions {
            let (_name, body) = function;
            let fn_code = body.compile()?;
            code_section.function(&fn_code);
        }
        module.section(&code_section);
        Ok(module)
    }
}
