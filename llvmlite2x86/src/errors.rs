/// Errors that can occur during the backend code generation
pub enum Error {
    DerefNonPointer,
}

pub type Result<T> = std::result::Result<T, Error>;
