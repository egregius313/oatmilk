use crate::context::{Context, TypingContext};

mod types;
pub use types::*;

pub(crate) trait Compile<Target> {
    /// Compile
    fn compile(self, context: &Context, type_context: &TypingContext) -> Target;
}
