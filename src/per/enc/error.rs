use crate::types::constraints::Range;
use snafu::*;

#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("invalid length, expected: {expected}; actual: {length}"))]
    InvalidLength {
        length: usize,
        expected: Range<usize>,
    },
    #[snafu(display("wrapped der encoding error: {source}"))]
    Der { source: crate::der::enc::Error },
    #[snafu(display("custom error:\n{}", msg))]
    Custom { msg: alloc::string::String },
}

impl Error {
    pub fn check_length(length: usize, expected: Range<usize>) -> Result<(), Self> {
        expected.contains_or_else(&length, || Self::InvalidLength { length, expected })
    }
}

impl crate::enc::Error for Error {
    fn custom<D: core::fmt::Display>(msg: D) -> Self {
        Self::Custom {
            msg: msg.to_string(),
        }
    }
}
