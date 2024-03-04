use std::cmp::Ordering;

use recordkeeper_macros::SaveBin;

use crate::{
    character::{class::ClassAccessory, CHARACTER_MAX},
    chrono::ChronologicalOrder,
    flags::FlagType,
    SaveData,
};

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

pub struct CommunityChrono<'a> {
    save: &'a mut SaveData,
    flag_type: FlagType,
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

impl<'a> CommunityChrono<'a> {
    pub fn new(save: &'a mut SaveData, flag_type: FlagType) -> Self {
        Self { save, flag_type }
    }

    /// Checks whether an NPC community entry is registered in the order.
    /// If it is not registered, then no task from that entry was completed.
    pub fn is_present(&self, flag: usize) -> bool {
        self.save
            .flags
            .get(self.flag_type, flag)
            .is_some_and(|f| f != 0)
    }

    /// Removes an NPC community entry from the order.
    pub fn delete(&mut self, flag: usize) {
        self.save.flags.set(self.flag_type, flag, 0);
    }
}

impl<'a> ChronologicalOrder for CommunityChrono<'a> {
    /// Compares the order of two NPC community entries. Parameters are flag IDs
    fn cmp_entries(&self, id_a: usize, id_b: usize) -> Ordering {
        self.save
            .flags
            .get(self.flag_type, id_a)
            .cmp(&self.save.flags.get(self.flag_type, id_b))
    }

    /// Swaps the order of two NPC community entries. Parameters are flag IDs
    fn swap(&mut self, id_a: usize, id_b: usize) {
        let val_a = self
            .save
            .flags
            .get(self.flag_type, id_a)
            .expect("id_a out of bounds");
        let val_b = self
            .save
            .flags
            .get(self.flag_type, id_b)
            .expect("id_b out of bounds");
        self.save.flags.set(self.flag_type, id_a, val_b);
        self.save.flags.set(self.flag_type, id_b, val_a);
    }

    /// Inserts a new NPC community entry at the end of the order. Parameter is
    /// a flag ID.
    fn insert(&mut self, id: usize) {
        if self.is_present(id) {
            return;
        }
        self.save.flags.set(
            self.flag_type,
            id,
            self.save.dlc4_community_order_max as u32,
        );
        self.save.dlc4_community_order_max += 1;
    }
}
