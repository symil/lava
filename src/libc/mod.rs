mod memory;
mod c_bindings;

pub use self::c_bindings::*;
pub use self::memory::*;

#[allow(non_camel_case_types)]
pub type void = i8;