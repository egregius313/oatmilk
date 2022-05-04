use crate::context::{Context, TypingContext};

pub trait Compile<Target> {
    /// Compile
    fn compile(self, context: &Context, type_context: &TypingContext) -> Target;
}
