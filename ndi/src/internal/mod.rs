#![allow(dead_code)]
#![allow(deref_nullptr)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

#[cfg(target_os = "windows")]
mod bindings_windows;

#[cfg(target_os = "linux")]
mod bindings_linux;

pub mod bindings {
    #[cfg(target_os = "windows")]
    pub use super::bindings_windows::*;

    #[cfg(target_os = "linux")]
    pub use super::bindings_linux::*;
}
