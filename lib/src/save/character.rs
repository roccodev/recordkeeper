use std::ops::Deref;

use recordkeeper_macros::SaveBin;

use crate::util::FixVec;

pub const PARTY_MAX: usize = 16;
pub const PARTY_GUEST_MAX: usize = 8;
pub const CHARACTER_MAX: usize = 64;
pub const OUROBOROS_MAX: usize = 6;
pub const PARTY_FORMATION_MAX: usize = 15;

const CHARACTER_CLASS_MAX: usize = 64;
pub const CHARACTER_CLASS_ART_MAX: usize = 7;
pub const CHARACTER_CLASS_SKILL_MAX: usize = 8;

#[derive(SaveBin, Debug)]
#[size(4444)]
pub struct Character {
    pub level: u32,
    pub exp: u32,
    pub bonus_exp: u32,

    #[loc(0x10)]
    pub selected_class: u8,

    #[loc(0x14)]
    class_inventory: [CharacterClass; CHARACTER_CLASS_MAX],

    pub costume_id: u16,
    /// The level the character joined the party at. Seems to be the character's
    /// ending level for NG+.
    pub arrival_level: u8,
    pub dirty_level: u8,
    pub attachment: u8, // unsure
}

#[derive(SaveBin, Debug)]
pub struct Ouroboros {
    pub art_ids: [u16; 5],
    pub exp: u16, // ???
    pub sp: u32,
    pub linked_skills: [u16; 2],

    #[loc(0x34)]
    pub skill_tree: OuroborosTree,
}

#[derive(SaveBin, Debug)]
#[size(68)]
pub struct CharacterClass {
    cp: u32,
    unlock_points: u16,
    level: u8,

    /// The raw value is `ITM_Gem.Category - 1`.
    /// Level can't be controlled.
    #[loc(0x8)]
    gems: [u8; 10],
    arts: [u16; CHARACTER_CLASS_ART_MAX],
    skills: [u16; CHARACTER_CLASS_SKILL_MAX],

    accessories: [ClassAccessory; 3],
}

#[derive(SaveBin, Debug)]
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

#[derive(SaveBin, Debug)]
pub struct OuroborosTree {
    raw: u64,
}

#[derive(SaveBin, Debug)]
#[size(9360)]
pub struct PartyFormation {
    name_id: u64, // unsure
    party: FixVec<u16, PARTY_MAX>,
    /// Indexed by character ID
    characters: [CharacterFormation; CHARACTER_MAX],
    ouroboros: [OuroborosFormation; OUROBOROS_MAX],
}

#[derive(SaveBin, Debug)]
#[size(144)]
struct CharacterFormation {
    #[loc(0x4)]
    class: CharacterClass,
    current_class: u16,
    character_id: u16, // unsure
    costume_id: u16,
    attachment: u16, // unsure
}

#[derive(SaveBin, Debug)]
struct OuroborosFormation {
    pub ouroboros_id: u16,
    pub art_ids: [u16; 5],
    pub linked_skills: [u16; 2],
}

impl Character {
    pub fn class_data(&self, class_id: usize) -> &CharacterClass {
        &self.class_inventory[class_id.checked_sub(1).expect("class ID must be >= 1")]
    }

    pub fn class_data_mut(&mut self, class_id: usize) -> &mut CharacterClass {
        &mut self.class_inventory[class_id.checked_sub(1).expect("class ID must be >= 1")]
    }
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

    pub fn arts(&self) -> impl Iterator<Item = Slot<u16>> + '_ {
        self.arts.iter().map(|slot| Slot(*slot))
    }

    pub fn arts_mut(&mut self) -> impl Iterator<Item = SlotMut<u16>> + '_ {
        self.arts.iter_mut().map(|slot| SlotMut(slot))
    }

    pub fn gems(&self) -> impl Iterator<Item = Slot<u8>> + '_ {
        self.gems.iter().map(|slot| Slot(*slot))
    }

    pub fn gems_mut(&mut self) -> impl Iterator<Item = SlotMut<u8>> + '_ {
        self.gems.iter_mut().map(|slot| SlotMut(slot))
    }

    pub fn skills(&self) -> impl Iterator<Item = Slot<u16>> + '_ {
        self.skills.iter().map(|slot| Slot(*slot))
    }

    pub fn skills_mut(&mut self) -> impl Iterator<Item = SlotMut<u16>> + '_ {
        self.skills.iter_mut().map(|slot| SlotMut(slot))
    }
}

impl<N> Slot<N>
where
    Self: EmptySlot,
    N: Copy,
{
    /// Returns the current value, or [`None`] if the slot is empty.
    pub fn get(&self) -> Option<N> {
        self.is_empty().then(|| self.0)
    }
}

impl<'a, N> SlotMut<'a, N>
where
    Self: EmptySlotMut,
    N: Copy,
{
    /// Returns the current value, or [`None`] if the slot is empty.
    pub fn get(&self) -> Option<N> {
        self.is_empty().then(|| *self.0)
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

impl EmptySlot for Slot<u8> {
    fn is_empty(&self) -> bool {
        self.0 != u8::MAX
    }
}

impl EmptySlot for Slot<u16> {
    fn is_empty(&self) -> bool {
        self.0 != u16::MAX
    }
}

impl<'a> EmptySlot for SlotMut<'a, u8> {
    fn is_empty(&self) -> bool {
        *self.0 != u8::MAX
    }
}

impl<'a> EmptySlot for SlotMut<'a, u16> {
    fn is_empty(&self) -> bool {
        *self.0 != u16::MAX
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
