use indexmap::IndexMap;

use oat_ast as oat;

pub(crate) type FieldSet = IndexMap<oat::Id, oat::Type>;

#[derive(Default)]
pub(crate) struct TypingContext(pub IndexMap<oat::Id, FieldSet>);

impl TypingContext {
    pub(crate) fn get_type(&self, name: &oat::Id) -> Option<&IndexMap<oat::Id, oat::Type>> {
        self.0.get(name)
    }

    /// Return the index and type of a field
    pub(crate) fn get_field(
        &self,
        type_name: &oat::Id,
        field_name: &oat::Id,
    ) -> Option<(usize, &oat::Type)> {
        let struct_ = self.0.get(type_name)?;
        let (i, _, type_) = struct_.get_full(field_name)?;
        Some((i, type_))
    }

    pub(crate) fn from_declarations(declarations: &Vec<oat::TypeDeclaration>) -> Self {
        let mut tc = Self::default();

        for oat::TypeDeclaration { name, fields } in declarations.iter() {
            tc.0.insert(*name, fields.clone());
        }

        tc
    }
}
