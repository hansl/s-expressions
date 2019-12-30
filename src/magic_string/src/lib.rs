mod chunk;
mod chunk_list;
mod error;
mod magic_buffer;
mod magic_string;

pub use crate::magic_buffer::*;
pub use crate::magic_string::*;

// Tests
mod chunk_test;
mod magic_buffer_test;
mod magic_string_test;
