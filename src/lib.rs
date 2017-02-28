pub mod decoder;
pub mod cpu;
mod instruction_set;
mod execution;
mod instructions;

#[macro_use]
extern crate bitflags;

extern crate zero;
