use recordkeeper_macros::SaveBin;

#[derive(SaveBin)]
pub struct AllFlags {
    // workaround for https://github.com/rust-lang/rust/issues/76560
    // words = flag count / 32 / bits
    flags_1b: BitFlags<1, { 65536 / 32 / 1 }>,
    flags_2b: BitFlags<2, { 65536 / 32 / 2 }>,
    flags_4b: BitFlags<4, { 8192 / 32 / 4 }>,
    flags_8b: ByteFlags<u8, 8192>,
    flags_16b: ByteFlags<u16, 3072>,
    flags_32b: ByteFlags<u32, 2372>
}


#[derive(SaveBin)]
struct BitFlags<const BITS: usize, const WORDS: usize> {
    words: [u32; WORDS]
}

#[derive(SaveBin)]
struct ByteFlags<B, const N: usize> {
    flags: [B; N]
}