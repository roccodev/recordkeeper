use crate::error::SaveError;
use crate::io::SaveBin;
use recordkeeper_macros::SaveBin;
use std::marker::PhantomData;

const FLAG_1_BIT_COUNT: usize = 65536;
const FLAG_2_BIT_COUNT: usize = 65536;
const FLAG_4_BIT_COUNT: usize = 8192;
const FLAG_8_BIT_COUNT: usize = 8192;
const FLAG_16_BIT_COUNT: usize = 3072;
const FLAG_32_BIT_COUNT: usize = 2372;

const FLAG_1_BIT_COUNT_UNK: usize = 80000;
const FLAG_2_BIT_COUNT_UNK: usize = 31936;

pub enum FlagType {
    Bit,
    TwoBits,
    FourBits,
    Byte,
    Short,
    Int,
}

#[derive(SaveBin, Debug)]
pub struct AllFlags {
    // workaround for https://github.com/rust-lang/rust/issues/76560
    // words = flag count / 32 * bits
    flags_1b: BitFlags<1, { FLAG_1_BIT_COUNT / 32 }>,
    flags_2b: BitFlags<2, { FLAG_2_BIT_COUNT / 32 * 2 }>,
    flags_4b: BitFlags<4, { FLAG_4_BIT_COUNT / 32 * 4 }>,
    flags_8b: ByteFlags<u8, FLAG_8_BIT_COUNT>,
    flags_16b: ByteFlags<u16, FLAG_16_BIT_COUNT>,
    flags_32b: ByteFlags<u32, FLAG_32_BIT_COUNT>,
}

#[derive(SaveBin, Debug)]
pub struct UnknownFlags {
    flags_1b: BitFlags<1, { FLAG_1_BIT_COUNT_UNK / 32 }>,
    flags_2b: BitFlags<2, { FLAG_2_BIT_COUNT_UNK / 32 * 2 }>,
}

#[derive(SaveBin, Debug)]
#[size(WORDS * 4)]
pub struct BitFlags<const BITS: usize, const WORDS: usize> {
    words: [u32; WORDS],
    _bits: [PhantomData<()>; BITS],
}

#[derive(SaveBin, Debug)]
struct ByteFlags<B: for<'a> SaveBin<'a>, const N: usize>
where
    SaveError: for<'a> From<<B as SaveBin<'a>>::ReadError>,
    SaveError: for<'a> From<<B as SaveBin<'a>>::WriteError>,
{
    flags: [B; N],
}

impl AllFlags {
    pub fn get(&self, flag_type: FlagType, index: usize) -> Option<u32> {
        match flag_type {
            FlagType::Bit => self.flags_1b.get(index),
            FlagType::TwoBits => self.flags_2b.get(index),
            FlagType::FourBits => self.flags_4b.get(index),
            FlagType::Byte => self.flags_8b.get(index).map(u32::from),
            FlagType::Short => self.flags_16b.get(index).map(u32::from),
            FlagType::Int => self.flags_32b.get(index),
        }
    }
}

impl<const BITS: usize, const WORDS: usize> BitFlags<BITS, WORDS> {
    const MASK: u32 = (1 << BITS) - 1;
    const SLOT_LEN: usize = u32::BITS as usize / BITS;

    pub fn get(&self, index: usize) -> Option<u32> {
        self.words
            .get(index / Self::SLOT_LEN)
            .map(|&val| val & (Self::MASK << (index / BITS)))
    }
}

impl<B: for<'a> SaveBin<'a>, const N: usize> ByteFlags<B, N>
where
    SaveError: for<'a> From<<B as SaveBin<'a>>::ReadError>,
    SaveError: for<'a> From<<B as SaveBin<'a>>::WriteError>,
    B: Copy,
{
    pub fn get(&self, index: usize) -> Option<B> {
        self.flags.get(index).copied()
    }
}
