//src/core/io/errors.rs
use crate::core::io::read::{Terminator,Section};
#[derive(Debug,PartialEq, Eq)]
pub enum ReadError{
    InvalidTerminator(Terminator),
    InvalidContext(Section),
    MissingTerminator,
    InvalidDataFormat,
    InvalidNumber,
    InvalidInstructionFormat
}