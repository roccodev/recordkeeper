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

impl Dlc4 {
    /// Gets the current number of victories against an enemypedia enemy.
    ///
    /// The `index` parameter is `32F9A6F1` from `B4158056`, - 2190.
    ///
    /// ## Panics
    /// Panics if the index is out of bounds (`0 <= index < 400`)
    pub fn get_enemypedia_count(&self, index: usize) -> u8 {
        if index < 200 {
            self.enemypedia_0_199[index]
        } else {
            self.enemypedia_200_399[index - 200]
        }
    }

    /// Updates the current number of victories against an enemypedia enemy.
    ///
    /// The `index` parameter is `32F9A6F1` from `B4158056`, - 2190.
    ///
    /// ## Panics
    /// Panics if the index is out of bounds (`0 <= index < 400`)
    pub fn set_enemypedia_count(&mut self, index: usize, count: u8) {
        if index < 200 {
            self.enemypedia_0_199[index] = count;
        } else {
            self.enemypedia_200_399[index - 200] = count;
        }
    }
}
