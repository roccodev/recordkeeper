use recordkeeper_macros::SaveBin;

use crate::item::{ItemSlot, ItemType};

pub const CHARACTER_CLASS_ART_MAX: usize = 7;
pub const CHARACTER_CLASS_SKILL_MAX: usize = 8;
pub const CHARACTER_CLASS_GEM_MAX: usize = 10;
pub const CHARACTER_CLASS_ACCESSORY_MAX: usize = 3;

#[derive(SaveBin, Debug)]
#[size(68)]
pub struct CharacterClass {
    pub cp: u32,
    pub unlock_points: u16,
    pub level: u8,

    /// The raw value is `ITM_Gem.Category - 1`.
    /// Level can't be controlled.
    #[loc(0x8)]
    gems: [u8; CHARACTER_CLASS_GEM_MAX],
    arts: [u16; CHARACTER_CLASS_ART_MAX],
    skills: [u16; CHARACTER_CLASS_SKILL_MAX],

    accessories: [ClassAccessory; CHARACTER_CLASS_ACCESSORY_MAX],
}

/// Accessory slot data.
///
/// What is important here is `slot_index`, changing the BDAT ID
/// has no effect.
#[derive(SaveBin, Debug, Default, Clone, Copy)]
pub struct ClassAccessory {
    bdat_id: u16,
    slot_index: u16,
    item_type: u16,
}

pub struct Slot<N>(N);

pub struct SlotMut<'a, N>(&'a mut N);

pub trait EmptySlot {
    fn is_empty(&self) -> bool;
}

pub trait EmptySlotMut: EmptySlot {
    fn set_empty(&mut self);
}

impl CharacterClass {
    pub fn art_slot(&self, index: usize) -> Slot<u16> {
        Slot(self.arts[index])
    }

    pub fn art_slot_mut(&mut self, index: usize) -> SlotMut<u16> {
        SlotMut(&mut self.arts[index])
    }

    pub fn gem_slot(&self, index: usize) -> Slot<u8> {
        Slot(self.gems[index])
    }

    pub fn gem_slot_mut(&mut self, index: usize) -> SlotMut<u8> {
        SlotMut(&mut self.gems[index])
    }

    pub fn skill_slot(&self, index: usize) -> Slot<u16> {
        Slot(self.skills[index])
    }

    pub fn skill_slot_mut(&mut self, index: usize) -> SlotMut<u16> {
        SlotMut(&mut self.skills[index])
    }

    pub fn accessory_slot(&self, index: usize) -> Slot<ClassAccessory> {
        Slot(self.accessories[index])
    }

    pub fn accessory_slot_mut(&mut self, index: usize) -> SlotMut<ClassAccessory> {
        SlotMut(&mut self.accessories[index])
    }

    pub fn arts(&self) -> impl Iterator<Item = Slot<u16>> + '_ {
        self.arts.iter().map(|slot| Slot(*slot))
    }

    pub fn arts_mut(&mut self) -> impl Iterator<Item = SlotMut<u16>> + '_ {
        self.arts.iter_mut().map(SlotMut)
    }

    pub fn gems(&self) -> impl Iterator<Item = Slot<u8>> + '_ {
        self.gems.iter().map(|slot| Slot(*slot))
    }

    pub fn gems_mut(&mut self) -> impl Iterator<Item = SlotMut<u8>> + '_ {
        self.gems.iter_mut().map(SlotMut)
    }

    pub fn skills(&self) -> impl Iterator<Item = Slot<u16>> + '_ {
        self.skills.iter().map(|slot| Slot(*slot))
    }

    pub fn skills_mut(&mut self) -> impl Iterator<Item = SlotMut<u16>> + '_ {
        self.skills.iter_mut().map(SlotMut)
    }

    pub fn accessories(&self) -> impl Iterator<Item = Slot<ClassAccessory>> + '_ {
        self.accessories.iter().map(|slot| Slot(*slot))
    }

    pub fn accessories_mut(&mut self) -> impl Iterator<Item = SlotMut<ClassAccessory>> + '_ {
        self.accessories.iter_mut().map(SlotMut)
    }
}

impl ClassAccessory {
    pub fn bdat_id(&self) -> u16 {
        self.bdat_id
    }

    pub fn item_type(&self) -> ItemType {
        todo!()
    }

    pub fn slot_index(&self) -> u16 {
        self.slot_index
    }
}

impl<N> Slot<N>
where
    Self: EmptySlot,
    N: Copy,
{
    /// Returns the current value, or [`None`] if the slot is empty.
    pub fn get(&self) -> Option<N> {
        (!self.is_empty()).then(|| self.0)
    }
}

impl<'a, N> SlotMut<'a, N>
where
    Self: EmptySlotMut,
    N: Copy,
{
    /// Returns the current value, or [`None`] if the slot is empty.
    pub fn get(&self) -> Option<N> {
        (!self.is_empty()).then(|| *self.0)
    }

    /// Updates the current value. Accepts [`Some`] for a valid entry
    /// and [`None`] for an empty slot.
    pub fn set(&mut self, value: Option<N>) {
        match value {
            Some(n) => *self.0 = n,
            None => self.set_empty(),
        }
    }
}

impl<'a> SlotMut<'a, ClassAccessory> {
    /// Marks the accessory slot as valid based on the given inventory slot.
    ///
    /// Item type and BDAT ID will be updated accordingly. If the inventory
    /// slot is empty, the accessory slot will also be emptied.
    pub fn set_from_inventory(&mut self, inventory_slot: &ItemSlot) {
        let out = &mut self.0;
        if !inventory_slot.is_valid() {
            self.set_empty();
            return;
        }
        out.slot_index = inventory_slot.index();
        out.bdat_id = inventory_slot.item_id();
        out.item_type = inventory_slot.item_type() as u16;
    }
}

impl EmptySlot for Slot<u8> {
    fn is_empty(&self) -> bool {
        self.0 == u8::MAX
    }
}

impl EmptySlot for Slot<u16> {
    fn is_empty(&self) -> bool {
        self.0 == u16::MAX
    }
}

impl<'a> EmptySlot for SlotMut<'a, u8> {
    fn is_empty(&self) -> bool {
        *self.0 == u8::MAX
    }
}

impl<'a> EmptySlot for SlotMut<'a, u16> {
    fn is_empty(&self) -> bool {
        *self.0 == u16::MAX
    }
}

impl<'a> EmptySlotMut for SlotMut<'a, u8> {
    fn set_empty(&mut self) {
        *self.0 = u8::MAX
    }
}

impl<'a> EmptySlotMut for SlotMut<'a, u16> {
    fn set_empty(&mut self) {
        *self.0 = u16::MAX
    }
}

impl EmptySlot for Slot<ClassAccessory> {
    fn is_empty(&self) -> bool {
        self.0.bdat_id == 0 || self.0.item_type == 0
    }
}

impl<'a> EmptySlot for SlotMut<'a, ClassAccessory> {
    fn is_empty(&self) -> bool {
        Slot(*self.0).is_empty()
    }
}

impl<'a> EmptySlotMut for SlotMut<'a, ClassAccessory> {
    fn set_empty(&mut self) {
        *self.0 = ClassAccessory::default();
    }
}
