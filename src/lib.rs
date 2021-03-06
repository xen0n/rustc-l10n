#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate term;

pub mod errors;
#[macro_use]
mod macros;
pub mod spec;
pub mod render;
