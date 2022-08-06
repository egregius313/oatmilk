use indexmap::IndexMap;

use llvmlite as ll;
use oat_ast as oat;

mod type_context;
pub use type_context::*;

#[derive(Default, Clone)]
pub(crate) struct Context {
    operands: IndexMap<oat::Id, (ll::Type, ll::Operand)>,
}

impl Context {
    pub(crate) fn new() -> Self {
        Context {
            operands: Default::default(),
        }
    }

    pub(crate) fn extend_operand(&self, id: oat::Id, type_: ll::Type, op: ll::Operand) -> Self {
        let mut copy = self.clone();
        copy.operands.insert(id, (type_, op));
        copy
    }
}
