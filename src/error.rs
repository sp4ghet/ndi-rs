use super::*;

/// Various Errors that the library could return
#[derive(Debug)]
pub enum NDIError {
    /// The system is not compatible with NDI
    NotSupported,
    /// Failed to convert a c `int` into an `enum`
    InvalidEnum(i32, &'static str),
    /// Failed to create an instance of Recv
    RecvCreateError,
    /// Failed to create an instance of Find
    FindCreateError,
    /// Failed to create an instance of Send
    SendCreateError,
    /// Findng the current sources timed out
    FindSourcesTimeout,
}

impl Display for NDIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(&self, f)
    }
}

impl std::error::Error for NDIError {}
