use recordkeeper_macros::SaveBin;

use crate::{flags::BitFlags, util::FixVec};

use class::CharacterClass;
use slot::{Slot, SlotMut};

pub const PARTY_MAX: usize = 16;
pub const PARTY_GUEST_MAX: usize = 8;
pub const CHARACTER_MAX: usize = 64;
pub const OUROBOROS_MAX: usize = 6;
pub const PARTY_FORMATION_MAX: usize = 15;

const CHARACTER_CLASS_MAX: usize = 64;

pub const OUROBOROS_ART_MAX: usize = 5;
pub const OUROBOROS_SKILL_MAX: usize = 2;

pub mod class;
pub mod slot;

#[derive(SaveBin, Debug)]
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
pub struct Ouroboros {
    pub art_ids: [u16; 5],
    pub exp: u16, // ???
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
    Dlc4Gem1 = 5,
    Dlc4Gem2 = 6,
    Dlc4Gem3 = 7,
    Dlc4Accessory2 = 8,
    Dlc4Accessory3 = 9,
}

#[derive(SaveBin, Debug)]
pub struct OuroborosTree {
    raw: BitFlags<1, 2>,
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
