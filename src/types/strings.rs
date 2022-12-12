mod ia5;
mod visible;
mod constrained;

use bitvec::prelude::*;

use crate::prelude::*;

pub use {
    visible::VisibleString,
    ia5::Ia5String,
    alloc::string::String as Utf8String,
};

// ///  The `GeneralString` type.
// pub type GeneralString = Implicit<tag::GENERAL_STRING, Utf8String>;

pub(crate) use constrained::{DynConstrainedCharacterString, ConstrainedCharacterString, StaticPermittedAlphabet, try_from_permitted_alphabet};

const PRINTABLE_WIDTH: usize = 7;
const NUMERIC_WIDTH: usize = 4;
const BMP_WIDTH: usize = u16::BITS as usize;

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PrintableString(ConstrainedCharacterString<PRINTABLE_WIDTH>);

impl StaticPermittedAlphabet for PrintableString {
    const CHARACTER_SET: &'static [u32] = &bytes_to_chars([
        b'A', b'B', b'C', b'E', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K',
        b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W',
        b'X', b'Y', b'Z', b'a', b'b', b'c', b'e', b'd', b'e', b'f', b'g', b'h',
        b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't',
        b'u', b'v', b'w', b'x', b'y', b'z', b'\'', b'(',b')', b'+', b',', b'-',
        b'.', b'/', b':', b'=', b'?'
    ]);
}

impl PrintableString {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, InvalidPrintableString> {
        if bytes.iter().copied().map(u32::from).all(|b| Self::CHARACTER_SET.contains(&b)) {
            Ok(Self(ConstrainedCharacterString::from_raw_bits(bytes.into_iter().flat_map(|b| b.view_bits::<Msb0>()[1..8].to_owned()).collect())))
        } else {
            Err(InvalidPrintableString)
        }
    }
}

#[derive(snafu::Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
#[snafu(display("Invalid printable string"))]
pub struct InvalidPrintableString;

impl TryFrom<alloc::vec::Vec<u8>> for PrintableString {
    type Error = InvalidPrintableString;

    fn try_from(value: alloc::vec::Vec<u8>) -> Result<Self, Self::Error> {
        Self::from_bytes(&value)
    }
}

impl TryFrom<&'_ [u8]> for PrintableString {
    type Error = InvalidPrintableString;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::from_bytes(value)
    }
}


impl core::ops::Deref for PrintableString {
    type Target = ConstrainedCharacterString<PRINTABLE_WIDTH>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsnType for PrintableString {
    const TAG: Tag = Tag::PRINTABLE_STRING;
}

impl Encode for PrintableString {
    fn encode_with_tag_and_constraints<'constraints, E: Encoder>(&self, encoder: &mut E, tag: Tag, constraints: Constraints<'constraints>) -> Result<(), E::Error> {
        encoder.encode_printable_string(tag, constraints, &self).map(drop)
    }
}

impl Decode for PrintableString {
    fn decode_with_tag_and_constraints<'constraints, D: Decoder>(decoder: &mut D, tag: Tag, constraints: Constraints<'constraints>) -> Result<Self, D::Error> {
        decoder.decode_printable_string(tag, constraints)
    }
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NumericString(ConstrainedCharacterString<NUMERIC_WIDTH>);

impl NumericString {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, InvalidNumericString> {
        let mut buffer = BitString::new();
        let map = Self::index_map();

        for byte in bytes {
            match map.get(&(*byte as u32)) {
                Some(index) => buffer.extend_from_bitslice(&index.view_bits::<Msb0>()[0..4]),
                None => return Err(InvalidNumericString),
            }
        }

        Ok(Self(ConstrainedCharacterString::from_raw_bits(buffer)))
    }
}

const fn bytes_to_chars<const N: usize>(input: [u8; N]) -> [u32; N] {
    let mut chars: [u32; N] = [0; N];

    let mut index = 0;
    while index < N {
        chars[index] = input[index] as u32;
        index += 1;
    }

    chars
}

impl StaticPermittedAlphabet for NumericString {
    const CHARACTER_SET: &'static [u32] = &bytes_to_chars([
        b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b' ',
    ]);
}

#[derive(snafu::Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
#[snafu(display("Invalid numeric string"))]
pub struct InvalidNumericString;

impl TryFrom<alloc::vec::Vec<u8>> for NumericString {
    type Error = InvalidNumericString;

    fn try_from(value: alloc::vec::Vec<u8>) -> Result<Self, Self::Error> {
        Self::from_bytes(&value)
    }
}


impl core::ops::Deref for NumericString {
    type Target = ConstrainedCharacterString<NUMERIC_WIDTH>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsnType for NumericString {
    const TAG: Tag = Tag::NUMERIC_STRING;
}

impl Encode for NumericString {
    fn encode_with_tag_and_constraints<'constraints, E: Encoder>(&self, encoder: &mut E, tag: Tag, constraints: Constraints<'constraints>) -> Result<(), E::Error> {
        encoder.encode_numeric_string(tag, constraints, &self).map(drop)
    }
}

impl Decode for NumericString {
    fn decode_with_tag_and_constraints<'constraints, D: Decoder>(decoder: &mut D, tag: Tag, constraints: Constraints<'constraints>) -> Result<Self, D::Error> {
        decoder.decode_numeric_string(tag, constraints)
    }
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TeletexString(Vec<u8>);

impl TeletexString {
    pub fn new(vec: Vec<u8>) -> Self {
        Self(vec)
    }
}

impl From<Vec<u8>> for TeletexString {
    fn from(vec: Vec<u8>) -> Self {
        Self::new(vec)
    }
}

impl core::ops::Deref for TeletexString {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsnType for TeletexString {
    const TAG: Tag = Tag::TELETEX_STRING;
}

impl Encode for TeletexString {
    fn encode_with_tag_and_constraints<'constraints, E: Encoder>(&self, encoder: &mut E, tag: Tag, constraints: Constraints<'constraints>) -> Result<(), E::Error> {
        encoder.encode_teletex_string(tag, constraints, &self).map(drop)
    }
}

impl Decode for TeletexString {
    fn decode_with_tag_and_constraints<'constraints, D: Decoder>(decoder: &mut D, tag: Tag, constraints: Constraints<'constraints>) -> Result<Self, D::Error> {
        decoder.decode_teletex_string(tag, constraints)
    }
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BmpString(ConstrainedCharacterString<BMP_WIDTH>);

impl core::ops::Deref for BmpString {
    type Target = ConstrainedCharacterString<BMP_WIDTH>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsnType for BmpString {
    const TAG: Tag = Tag::BMP_STRING;
}

impl Encode for BmpString {
    fn encode_with_tag_and_constraints<'constraints, E: Encoder>(&self, encoder: &mut E, tag: Tag, constraints: Constraints<'constraints>) -> Result<(), E::Error> {
        encoder.encode_bmp_string(tag, constraints, &self).map(drop)
    }
}

impl Decode for BmpString {
    fn decode_with_tag_and_constraints<'constraints, D: Decoder>(decoder: &mut D, tag: Tag, constraints: Constraints<'constraints>) -> Result<Self, D::Error> {
        decoder.decode_bmp_string(tag, constraints)
    }
}
