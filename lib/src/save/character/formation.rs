use recordkeeper_macros::SaveBin;

use crate::util::FixVec;

use super::{
    class::CharacterClass,
    slot::{Slot, SlotMut},
    CHARACTER_MAX, OUROBOROS_ART_MAX, OUROBOROS_MAX, OUROBOROS_SKILL_MAX, PARTY_MAX,
};

pub const PARTY_FORMATION_MAX: usize = 15;

#[derive(SaveBin, Debug)]
#[size(9360)]
pub struct PartyFormation {
    name_id: u64, // unsure
    pub party: FixVec<u16, PARTY_MAX>,
    /// Indexed by character ID
    pub characters: [CharacterFormation; CHARACTER_MAX],
    pub ouroboros: [OuroborosFormation; OUROBOROS_MAX],
}

#[derive(SaveBin, Debug)]
#[size(144)]
pub struct CharacterFormation {
    #[loc(0x4)]
    class: CharacterClass,
    current_class: u16,
    character_id: u16, // unsure
    costume_id: u16,
    attachment: u16, // unsure
}

#[derive(SaveBin, Debug)]
pub struct OuroborosFormation {
    pub ouroboros_id: u16,
    pub art_ids: [u16; OUROBOROS_ART_MAX],
    pub linked_skills: [u16; OUROBOROS_SKILL_MAX],
}

impl PartyFormation {
    /// Returns the ouroboros formation slot for the given Ouroboros ID. (1-6)
    pub fn ouroboros(&self, ouro_id: u16) -> Option<&OuroborosFormation> {
        self.ouroboros.iter().find(|o| o.ouroboros_id == ouro_id)
    }

    /// Returns a mutable ouroboros slot for the Ouroboros ID. (1-6)
    ///
    /// The function searches for the ouroboros's slot, and returns a mutable
    /// reference to the first empty slot if it can't be found.
    ///
    /// ## Panics
    /// The function panics if the ouroboros slot could not be found, and there
    /// were no empty slots to edit.
    pub fn ouroboros_mut(&mut self, ouro_id: u16) -> &mut OuroborosFormation {
        // Can't iter_mut twice because of borrow checker limitations
        for (i, slot) in self.ouroboros.iter().enumerate() {
            if slot.ouroboros_id == ouro_id {
                return &mut self.ouroboros[i];
            }
        }
        self.ouroboros
            .iter_mut()
            .find(|o| o.ouroboros_id == 0)
            .expect("no suitable slot for ouro ID")
    }
}

impl OuroborosFormation {
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
