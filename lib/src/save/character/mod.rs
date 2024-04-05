use std::num::NonZeroU32;

use recordkeeper_macros::SaveBin;

use crate::flags::BitFlags;

use class::CharacterClass;
use slot::{Slot, SlotMut};

pub const PARTY_MAX: usize = 16;
pub const PARTY_GUEST_MAX: usize = 8;
pub const CHARACTER_MAX: usize = 64;
pub const OUROBOROS_MAX: usize = 6;

const CHARACTER_CLASS_MAX: usize = 64;

pub const OUROBOROS_ART_MAX: usize = 5;
pub const OUROBOROS_SKILL_MAX: usize = 2;

pub mod class;
pub mod formation;
pub mod slot;

#[derive(SaveBin, Debug, Clone, Copy)]
#[size(4444)]
pub struct Character {
    pub level: u32,
    pub exp: u32,
    pub bonus_exp: u32,

    flags: BitFlags<1, 1>,

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
pub struct CharacterSets {
    /// Characters that can be added to the party.
    /// Bit index = Character ID - 1
    pub selectable_characters: BitFlags<1, 2>,
    /// Characters that are fully unlocked.  
    /// Bit index = Character ID - 1
    pub permanent_characters: BitFlags<1, 2>,
    /// Characters that can only join the party temporarily.  
    /// Bit index = Character ID - 1
    pub temporary_characters: BitFlags<1, 2>,
}

#[derive(SaveBin, Debug)]
pub struct Ouroboros {
    pub art_ids: [u16; 5],
    #[loc(0xc)]
    pub sp: u32,
    pub linked_skills: [u16; 2],

    #[loc(0x34)]
    pub skill_tree: OuroborosTree,
}

#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "strum", derive(strum::EnumIter))]
pub enum CharacterFlag {
    UnloadDlcCostume = 0,
    HasEyepatch = 1,
    Dlc4MasterArt1 = 2,
    Dlc4MasterArt2 = 3,
    Dlc4MasterArt3 = 4,
    Dlc4Gem2 = 5,
    Dlc4Gem3 = 6,
    Dlc4Accessory2 = 7,
    Dlc4Accessory3 = 8,
}

#[derive(SaveBin, Debug)]
pub struct OuroborosTree {
    raw: BitFlags<1, 2>,
}

impl Character {
    pub fn class_data(&self, class_id: NonZeroU32) -> &CharacterClass {
        &self.class_inventory[usize::try_from(u32::from(class_id) - 1).unwrap()]
    }

    pub fn class_data_mut(&mut self, class_id: NonZeroU32) -> &mut CharacterClass {
        &mut self.class_inventory[usize::try_from(u32::from(class_id) - 1).unwrap()]
    }

    pub fn is_flag_set(&self, flag: CharacterFlag) -> bool {
        self.flags.get(flag as usize).unwrap() != 0
    }

    pub fn set_flag(&mut self, flag: CharacterFlag, value: bool) {
        self.flags.set(flag as usize, u8::from(value).into())
    }
}

impl Ouroboros {
    pub fn art_slot(&self, index: usize) -> Slot<u16> {
        Slot(self.art_ids[index])
    }

    pub fn art_slot_mut(&mut self, index: usize) -> SlotMut<u16> {
        SlotMut(&mut self.art_ids[index])
    }

    pub fn linked_skill_slot(&self, index: usize) -> Slot<u16> {
        Slot(self.linked_skills[index])
    }

    pub fn linked_skill_slot_mut(&mut self, index: usize) -> SlotMut<u16> {
        SlotMut(&mut self.linked_skills[index])
    }
}

impl OuroborosTree {
    pub fn get(&self, index: usize) -> bool {
        self.raw.get(index).expect("index out of bounds") != 0
    }

    pub fn set(&mut self, index: usize, val: bool) {
        self.raw.set(index, u8::from(val).into())
    }
}
