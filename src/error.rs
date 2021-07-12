use super::*;

/// Various Errors that the library could return
#[derive(Debug)]
pub enum NDIError {
    /// The system is not compatible with NDI
    NotSupported,
    /// Failed to convert a c `int` into an `enum`
    InvalidEnum(i32, &'static str),
    RecvCreateError,
    FindCreateError,
    SendCreateError,
    FindSourcesTimeout,
}

impl Display for NDIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(&self, f)
    }
}

impl std::error::Error for NDIError {}
