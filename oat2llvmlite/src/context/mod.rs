use indexmap::IndexMap;

use llvmlite as ll;
use oat_ast as oat;

pub type Context = IndexMap<oat::Id, (ll::Type, ll::Operand)>;

pub struct TypingContext(pub IndexMap<oat::Id, IndexMap<oat::Id, oat::Type>>);

impl TypingContext {
    fn get_type(&self, name: &oat::Id) -> Option<IndexMap<oat::Id, oat::Type>> {
        self.0.get(name)
    }

    /// Return the index and type of a field
    fn get_field(&self, type_name: &oat::Id, field_name: &oat::Id) -> Option<(usize, oat::Type)> {
        let struct_ = self.0.get(type_name)?;
        let (i, _, type_) = struct_.get_full(field_name)?;
        Some((i, type_))
    }
}
