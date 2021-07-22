use super::*;

macro_rules! impl_error {
    ($name:ident) => {
        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                <Self as Debug>::fmt(&self, f)
            }
        }

        impl std::error::Error for $name {}
    };
}

/// The system is not compatible with NDI
#[derive(Debug)]
pub struct NotSupported;
impl_error!(NotSupported);

/// Failed to convert a c `int` into an `enum`
#[derive(Debug)]
pub struct InvalidEnum(pub i32, pub &'static str);
impl_error!(InvalidEnum);

/// Failed to create an instance of Recv
#[derive(Debug)]
pub struct RecvCreateError;
impl_error!(RecvCreateError);

/// Failed to create an instance of Find
#[derive(Debug)]
pub struct FindCreateError;
impl_error!(FindCreateError);

/// Failed to create an instance of Send
#[derive(Debug)]
pub struct SendCreateError;
impl_error!(SendCreateError);

/// Findng the current sources timed out
#[derive(Debug)]
pub struct FindSourcesTimeout;
impl_error!(FindSourcesTimeout);
