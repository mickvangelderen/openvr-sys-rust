// NOTE(mickvangelderen): The proper way to do it, but no completion by racer :'(.
// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
// pub use self::root::vr::*;

mod bindings;
pub use self::bindings::root::vr::*;
