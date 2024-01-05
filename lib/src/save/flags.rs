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

#[derive(PartialEq, Clone, Copy)]
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
    flags_1b: BitFlags<1, { FLAG_1_BIT_COUNT.div_ceil(32) }>,
    flags_2b: BitFlags<2, { FLAG_2_BIT_COUNT.div_ceil(16) }>,
    flags_4b: BitFlags<4, { FLAG_4_BIT_COUNT.div_ceil(8) }>,
    flags_8b: ByteFlags<u8, FLAG_8_BIT_COUNT>,
    flags_16b: ByteFlags<u16, FLAG_16_BIT_COUNT>,
    flags_32b: ByteFlags<u32, FLAG_32_BIT_COUNT>,
}

#[derive(SaveBin, Debug, Clone, Copy)]
#[size(WORDS * 4)]
pub struct BitFlags<const BITS: usize, const WORDS: usize> {
    words: [u32; WORDS],
    _bits: [PhantomData<()>; BITS],
}

#[derive(SaveBin, Debug)]
struct ByteFlags<B: SaveBin, const N: usize>
where
    SaveError: From<<B as SaveBin>::ReadError>,
    SaveError: From<<B as SaveBin>::WriteError>,
{
    flags: [B; N],
}

impl FlagType {
    pub fn from_bits(bits: usize) -> Self {
        match bits {
            1 => Self::Bit,
            2 => Self::TwoBits,
            4 => Self::FourBits,
            8 => Self::Byte,
            16 => Self::Short,
            32 => Self::Int,
            n => panic!("unknown bit count for flag {n}"),
        }
    }

    pub const fn is_valid(&self, value: u32) -> bool {
        match self {
            Self::Bit => value < 2,
            Self::TwoBits => value < 4,
            Self::FourBits => value < 16,
            Self::Byte => value <= u8::MAX as u32,
            Self::Short => value <= u16::MAX as u32,
            Self::Int => true,
        }
    }

    pub const fn num_bits(&self) -> u32 {
        match self {
            Self::Bit => 1,
            Self::TwoBits => 2,
            Self::FourBits => 4,
            Self::Byte => u8::BITS,
            Self::Short => u16::BITS,
            Self::Int => u32::BITS,
        }
    }

    pub const fn num_flags(&self) -> usize {
        match self {
            Self::Bit => FLAG_1_BIT_COUNT,
            Self::TwoBits => FLAG_2_BIT_COUNT,
            Self::FourBits => FLAG_4_BIT_COUNT,
            Self::Byte => FLAG_8_BIT_COUNT,
            Self::Short => FLAG_16_BIT_COUNT,
            Self::Int => FLAG_32_BIT_COUNT,
        }
    }
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

    pub fn set(&mut self, flag_type: FlagType, index: usize, new_value: u32) {
        match flag_type {
            FlagType::Bit => self.flags_1b.set(index, new_value),
            FlagType::TwoBits => self.flags_2b.set(index, new_value),
            FlagType::FourBits => self.flags_4b.set(index, new_value),
            FlagType::Byte => self.flags_8b.set(index, new_value.try_into().unwrap()),
            FlagType::Short => self.flags_16b.set(index, new_value.try_into().unwrap()),
            FlagType::Int => self.flags_32b.set(index, new_value),
        }
    }
}

impl<const BITS: usize, const WORDS: usize> BitFlags<BITS, WORDS> {
    const MASK: u32 = (1 << BITS) - 1;
    const SLOT_LEN: usize = u32::BITS as usize / BITS;
    const MAX_SHIFT: usize = u32::BITS as usize - BITS;

    pub fn get(&self, index: usize) -> Option<u32> {
        let shift = (index * BITS) & Self::MAX_SHIFT;
        self.words
            .get(index / Self::SLOT_LEN)
            .map(|&val| (val & (Self::MASK << shift)) >> shift)
    }

    pub fn set(&mut self, index: usize, value: u32) {
        assert!(
            value <= Self::MASK,
            "value too big for {}-bit flag, found {value}",
            BITS
        );
        let shift = (index * BITS) & Self::MAX_SHIFT;
        let reset = !(Self::MASK << shift);
        self.words
            .get_mut(index / Self::SLOT_LEN)
            .map(|slot| *slot = (*slot & reset) | (value & Self::MASK) << shift)
            .expect("index out of bounds")
    }
}

impl<B: SaveBin, const N: usize> ByteFlags<B, N>
where
    SaveError: From<<B as SaveBin>::ReadError>,
    SaveError: From<<B as SaveBin>::WriteError>,
    B: Copy,
{
    pub fn get(&self, index: usize) -> Option<B> {
        self.flags.get(index).copied()
    }

    pub fn set(&mut self, index: usize, value: B) {
        self.flags
            .get_mut(index)
            .map(|f| *f = value)
            .expect("index out of bounds")
    }
}

impl<const BITS: usize, const WORDS: usize> Default for BitFlags<BITS, WORDS> {
    fn default() -> Self {
        Self {
            words: [0; WORDS],
            _bits: [PhantomData; BITS],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BitFlags;

    #[test]
    fn bitflag_set() {
        let mut flags_1b = BitFlags::<1, 1>::default(); // 32 1-bit flags
        let mut flags_4b = BitFlags::<4, 1>::default(); // 8 4-bit flags

        for i in 0..32 {
            flags_1b.set(i, i as u32 & 1);
        }

        for i in 0..32 {
            assert_eq!(i as u32 & 1, flags_1b.get(i).unwrap());
        }

        for i in 0..8 {
            flags_4b.set(i, i as u32);
        }

        for i in 0..8 {
            assert_eq!(i as u32, flags_4b.get(i).unwrap());
        }

        for i in 0..32 {
            flags_1b.set(i, 0);
        }

        for i in 0..32 {
            assert_eq!(0, flags_1b.get(i).unwrap());
        }

        for i in 0..8 {
            flags_4b.set(i, 0);
        }

        for i in 0..8 {
            assert_eq!(0, flags_4b.get(i).unwrap());
        }
    }
}
