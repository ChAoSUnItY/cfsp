use std::io::Read;

pub use error::{ParseError, ParseResult};
pub use signature::{class_signature, field_signature, method_signature};

use crate::node::class::Class;

mod attribute;
mod class;
mod constant;
mod error;
mod field;
mod method;
mod signature;

/// Parses class file based on given options.
pub fn to_class<R: Read>(class_bytes: &mut R, option: ParsingOption) -> ParseResult<Class> {
    let class = class::class(class_bytes, option)?;

    let mut remain = vec![];

    class_bytes.read_to_end(&mut remain)?;

    if !remain.is_empty() {
        Err(ParseError::Remains(remain.len()))
    } else {
        Ok(class)
    }
}

/// Parsing options allows marking some parsing phase optional.
#[derive(Debug, Default)]
pub struct ParsingOption {
    skip_attribute: bool,
    skip_instruction: bool,
}

impl ParsingOption {
    /// Skips on attribute parsing.
    pub fn skip_attribute(mut self) -> Self {
        self.skip_attribute = true;

        self
    }

    /// Skips on instruction parsing on code attribute.
    pub fn skip_instruction(mut self) -> Self {
        self.skip_instruction = true;

        self
    }
}
