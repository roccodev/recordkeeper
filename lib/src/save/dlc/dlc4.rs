use recordkeeper_macros::SaveBin;

use crate::character::{class::ClassAccessory, CHARACTER_MAX};

pub const DLC4_ENEMYPEDIA_MAX_EACH: usize = 200;

#[derive(SaveBin, Debug)]
pub struct Dlc4 {
    /// Number of victories for Enemypedia entries 0-199
    enemypedia_0_199: Box<[u8; DLC4_ENEMYPEDIA_MAX_EACH]>,

    /// Extra inventory, indexed by character ID
    extra_inventory: Box<[Dlc4ExtraInventory; CHARACTER_MAX]>,

    /// Number of victories for Enemypedia entries 200-399
    // lol
    #[loc(0x80c8)]
    enemypedia_200_399: Box<[u8; DLC4_ENEMYPEDIA_MAX_EACH]>,
}

#[derive(SaveBin, Debug)]
#[size(512)]
pub struct Dlc4ExtraInventory {
    /// Likely indexed by class ID
    battle_manual: Box<[ClassAccessory; 64]>,
}
