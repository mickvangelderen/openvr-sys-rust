#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// NOTE(mickvangelderen): The proper way to do it, but no completion by racer :'(.
// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
// pub use self::root::vr::*;

pub mod bindings;
pub mod manual;

pub use self::bindings::*;
pub use self::manual::*;
