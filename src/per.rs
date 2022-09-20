pub mod de;
pub mod enc;

pub use self::{de::Decoder, enc::Encoder};

const SIXTEEN_K: u16 = 16384;
const THIRTY_TWO_K: u16 = 32768;
const FOURTY_EIGHT_K: u16 = 49152;
const SIXTY_FOUR_K: u32 = 65536;

fn log2(x: i128) -> u32 {
    i128::BITS - (x - 1).leading_zeros()
}
