use crate::error::StructError;
use crate::io::SaveBin;
use recordkeeper_macros::SaveBin;
use std::marker::PhantomData;

const FLAG_1_BIT_COUNT: usize = 65536;
const FLAG_2_BIT_COUNT: usize = 65536;
const FLAG_4_BIT_COUNT: usize = 8192;
const FLAG_8_BIT_COUNT: usize = 8192;
const FLAG_16_BIT_COUNT: usize = 3072;
const FLAG_32_BIT_COUNT: usize = 2372;

#[derive(SaveBin, Debug)]
pub struct AllFlags {
    // workaround for https://github.com/rust-lang/rust/issues/76560
    // words = flag count / 32 / bits
    flags_1b: BitFlags<1, { FLAG_1_BIT_COUNT / 32 / 1 }>,
    flags_2b: BitFlags<2, { FLAG_2_BIT_COUNT / 32 / 2 }>,
    flags_4b: BitFlags<4, { FLAG_4_BIT_COUNT / 32 / 4 }>,
    flags_8b: ByteFlags<u8, FLAG_8_BIT_COUNT>,
    flags_16b: ByteFlags<u16, FLAG_16_BIT_COUNT>,
    flags_32b: ByteFlags<u32, FLAG_32_BIT_COUNT>,
}

#[derive(SaveBin, Debug)]
struct BitFlags<const BITS: usize, const WORDS: usize> {
    words: [u32; WORDS],
    _bits: [PhantomData<()>; BITS],
}

#[derive(SaveBin, Debug)]
struct ByteFlags<B: for<'a> SaveBin<'a>, const N: usize>
where
    StructError: for<'a> From<<B as SaveBin<'a>>::Error>,
{
    flags: [B; N],
}
