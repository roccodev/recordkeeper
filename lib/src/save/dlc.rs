use crate::item::DlcExtraInventory;

use recordkeeper_macros::SaveBin;

pub const DLC4_ENEMYPEDIA_MAX_EACH: usize = 200;

#[derive(SaveBin, Debug)]
pub struct Dlc4 {
    /// Number of victories for Enemypedia entries 0-199
    enemypedia_0_199: [u8; DLC4_ENEMYPEDIA_MAX_EACH],

    /// Extra inventory, indexed by character ID
    extra_inventory: [DlcExtraInventory; 64],

    /// Number of victories for Enemypedia entries 200-399
    // lol
    #[loc(0x80c8)]
    enemypedia_200_399: [u8; DLC4_ENEMYPEDIA_MAX_EACH],
}
