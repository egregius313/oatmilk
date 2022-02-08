pub enum Error {
    UnrepresentableValue,
}

pub type Result<T> = std::result::Result<T, Error>;
