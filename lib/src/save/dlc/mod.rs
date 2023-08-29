use std::num::NonZeroUsize;

use crate::flags::BitFlags;
use recordkeeper_macros::SaveBin;

mod challenge;
mod dlc4;
mod gauntlet;
mod masha;

pub use challenge::*;
pub use dlc4::*;
pub use gauntlet::*;
pub use masha::*;

const POW_AUGMENT_NUM_FLAGS: usize = 64;
pub const POW_AUGMENT_NUM: usize = 8;

/// Inoswap (base game) / Affinity Growth (Future Redeemed)
#[derive(SaveBin, Debug)]
#[size(12)]
pub struct PowAugment {
    learned: BitFlags<1, { (POW_AUGMENT_NUM_FLAGS + 31) / 32 }>,
    /// ID for `CHR_PC`
    #[loc(0xa)]
    pub chr_id: u8,
    /// The number of unlocked growth tree tiers
    pub unlocked_tiers: u8,
}

impl PowAugment {
    pub fn is_learned(&self, pow_id: NonZeroUsize) -> bool {
        self.learned
            .get(pow_id.get() - 1)
            .map(|flag| flag != 0)
            .unwrap_or_default()
    }

    pub fn set_learned(&mut self, pow_id: NonZeroUsize, learned: bool) {
        self.learned.set(pow_id.get() - 1, learned as u8 as u32);
    }
}
