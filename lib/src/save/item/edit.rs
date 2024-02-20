use crate::{
    dlc::{AccessoryCrafting, CraftItemData},
    SaveData, SaveResult,
};

use super::{ItemSlot, ItemType, SlotFlags};

pub struct ItemEditor<'a> {
    crafting: &'a mut AccessoryCrafting,
    slot: &'a mut ItemSlot,
    slot_id: usize,
    item_type: ItemType,
}

impl<'a> ItemEditor<'a> {
    /// Creates an item editor for the given item type and slot ID.
    ///
    /// ## Panics
    /// Panics if the slot ID is out of bounds for the given item type.
    pub fn new(save: &'a mut SaveData, item_type: ItemType, slot_id: usize) -> Self {
        let crafting = &mut save.accessory_crafting;
        let slot = &mut save.inventory.slots_mut(item_type)[slot_id];

        Self {
            crafting,
            slot,
            slot_id,
            item_type,
        }
    }

    /// Changes the slot's item ID.
    ///
    /// The slot will be initialized and given an amount of 1.  
    /// If the item is a crafted accessory, its extra data will be initialized if absent.
    ///
    /// An item ID of `0` will clear the item slot.
    ///
    /// ## Errors
    /// The function can fail if crafted data initialization fails.
    pub fn set_item_id(&mut self, item_id: u16) -> SaveResult<()> {
        if item_id == 0 {
            self.clear();
            return Ok(());
        }

        self.slot.item_id = item_id;
        self.init()?;

        Ok(())
    }

    /// Changes the slot's item amount.
    ///
    /// An amount of `0` will clear the item slot.
    pub fn set_amount(&mut self, amount: u16) {
        if amount != 0 {
            self.slot.amount = amount;
        } else {
            self.clear();
        }
    }

    /// Clears the item slot.
    ///
    /// If the item is a crafted accessory, its extra data will also be cleared.
    pub fn clear(&mut self) {
        if self.slot.is_crafted_accessory() {
            // Delete accessory crafting slot
            self.crafting.remove_data(self.slot_id);
            self.slot.flags &= !(SlotFlags::HasCraftData as u8);
        }

        let slot = &mut self.slot;
        slot.item_id = 0;
        slot.amount = 0;
        slot.chronological_id = 0;
        slot.item_type = 0;
        slot.flags &= !(SlotFlags::Active as u8);
    }

    /// Returns a mutable view of the accessory crafting data for the item slot, if present.
    pub fn craft_data_mut(&mut self) -> Option<&mut CraftItemData> {
        self.crafting.get_data_mut(self.slot_id)
    }

    fn init(&mut self) -> SaveResult<()> {
        let slot = &mut self.slot;
        slot.slot_index = self.slot_id.try_into().unwrap();
        slot.amount = 1;
        slot.item_type = self.item_type as u32;
        slot.flags |= SlotFlags::Active as u8;

        if slot.is_crafted_accessory() && self.crafting.get_data(self.slot_id).is_none() {
            // Init accessory crafting slot, if not initialized
            slot.flags |= SlotFlags::HasCraftData as u8;
            self.crafting
                .set_data(self.slot_id, CraftItemData::default())?;
        }
        Ok(())
    }
}
