#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

pub use self::bindings::*;

#[cfg(feature = "bindgen")]
mod bindings;