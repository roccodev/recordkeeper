use recordkeeper_macros::SaveBin;

use crate::{error::SaveError, item::ITEM_ACCESSORY_MAX, SaveResult};

/// Item ID to be used to mark accessories as crafted.
pub const CRAFTED_ITEM_ID: u16 = 793;

const MASHA_STAT_BOOSTS_MAX: usize = 4;
const MASHA_DATA_MAX: usize = 300;

#[derive(SaveBin, Debug)]
pub struct AccessoryCrafting {
    /// `0xffff` => no item. Otherwise, it's the 0-based index
    /// for the data table.
    offsets: [u16; ITEM_ACCESSORY_MAX],
    data: [CraftItemData; MASHA_DATA_MAX],
}

#[derive(SaveBin, Debug, Clone, Copy, PartialEq, Eq)]
#[size(28)]
pub struct CraftItemData {
    #[loc(0x4)]
    pub stat_boosts: [StatBoost; MASHA_STAT_BOOSTS_MAX],
    /// Index for `E0A85A79`
    pub display_id: u16,
    /// Index for `4548D8B2`
    pub enhance_id: u16,
    pub level: u32,
}

#[derive(SaveBin, Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatBoost {
    pub stat: u16,
    pub amount: u16,
}

pub enum Statistic {
    HP = 1,
    Strength = 2,
    Heal = 3,
    Dexterity = 4,
    Agility = 5,
    CritRate = 6,
    BlockRate = 7,
}

impl AccessoryCrafting {
    /// Returns the crafted accessory data for the given item slot, if it exists.
    ///
    /// Crafted accessories should use the [`CRAFTED_ITEM_ID`] constant as their item ID.
    pub fn get_data(&self, item_slot: usize) -> Option<&CraftItemData> {
        self.offsets
            .get(item_slot)
            .and_then(|i| self.data.get(*i as usize))
    }

    /// Returns a mutable view of the crafted accessory data for the given item slot, if it exists.
    ///
    /// Crafted accessories should use the [`CRAFTED_ITEM_ID`] constant as their item ID.
    pub fn get_data_mut(&mut self, item_slot: usize) -> Option<&mut CraftItemData> {
        self.offsets
            .get_mut(item_slot)
            .and_then(|i| self.data.get_mut(*i as usize))
    }

    /// Removes craft accessory data for the given item slot.
    pub fn remove_data(&mut self, item_slot: usize) {
        if let Some(slot) = self.offsets.get_mut(item_slot) {
            *slot = u16::MAX;
        }
    }

    /// Creates or replaces a crafted data slot for the given item slot.
    ///
    /// ## Errors
    ///
    /// The function fails if an entry must be created, but the craft data inventory is full.
    pub fn set_data(&mut self, item_slot: usize, data: CraftItemData) -> SaveResult<()> {
        let offset = self.offsets[item_slot];

        if offset != u16::MAX {
            // Slot already initialized
            self.data[offset as usize] = data;
            return Ok(());
        }

        // Need to allocate a new data sector: first, find an
        // unused offset.
        let mut offsets = self.offsets.clone();
        offsets.sort_unstable();

        let Some((_, empty)) = offsets
                .into_iter()
                .zip(0..MASHA_DATA_MAX)
                .find(|(a, b)| *a as usize != *b) else { return Err(SaveError::MashaInventoryFull) };

        // Place the data at the new offset.
        self.data[empty] = data;
        self.offsets[item_slot] = empty.try_into().unwrap();

        Ok(())
    }
}

impl Default for CraftItemData {
    fn default() -> Self {
        Self {
            stat_boosts: [StatBoost { stat: 1, amount: 0 }; MASHA_STAT_BOOSTS_MAX],
            display_id: 1,
            enhance_id: 1,
            level: 1,
        }
    }
}
