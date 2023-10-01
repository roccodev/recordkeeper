use recordkeeper_macros::SaveBin;

use crate::{util::FixVec, SaveData};

use super::{
    class::CharacterClass,
    slot::{Slot, SlotMut},
    Character, Ouroboros, CHARACTER_MAX, OUROBOROS_ART_MAX, OUROBOROS_MAX, OUROBOROS_SKILL_MAX,
    PARTY_MAX,
};

pub const PARTY_FORMATION_MAX: usize = 15;

#[derive(SaveBin, Debug)]
#[size(9360)]
pub struct PartyFormation {
    pub name: FormationName,
    #[loc(0x8)]
    pub party: FixVec<u16, PARTY_MAX>,
    /// Indexed by character ID
    pub characters: [CharacterFormation; CHARACTER_MAX],
    pub ouroboros: [OuroborosFormation; OUROBOROS_MAX],
}

#[derive(SaveBin, Debug, Default)]
pub struct FormationName {
    /// ID for `33F137E8`
    pub name_id: u16,
    /// Discriminator, can be set in-game
    pub number: u16,
    /// Color ID, 0-14
    pub color_id: u16,
}

#[derive(SaveBin, Debug)]
#[size(144)]
pub struct CharacterFormation {
    #[loc(0x4)]
    pub class: CharacterClass,
    pub current_class: u16,
    pub character_id: u16,
    pub costume_id: u16,
    pub attachment: u8,
}

#[derive(SaveBin, Debug)]
pub struct OuroborosFormation {
    pub ouroboros_id: u16,
    pub art_ids: [u16; OUROBOROS_ART_MAX],
    pub linked_skills: [u16; OUROBOROS_SKILL_MAX],
}

impl PartyFormation {
    /// Creates a new party formation from the current state of the save file.
    pub fn from_save(save: &SaveData, name: FormationName) -> Self {
        Self {
            name,
            party: save.party_characters,
            characters: save
                .characters
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    CharacterFormation::from_save(c, i.checked_add(1).unwrap().try_into().unwrap())
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            ouroboros: save
                .ouroboros
                .iter()
                .enumerate()
                .map(|(i, o)| {
                    OuroborosFormation::from_save(o, i.checked_add(1).unwrap().try_into().unwrap())
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }

    /// Returns whether a valid party formation is currently in the slot.
    pub fn is_valid(&self) -> bool {
        !self.party.is_empty()
    }

    /// Returns the character formation slot for the given character ID. (starts at 1)
    pub fn character(&self, char_id: u16) -> Option<&CharacterFormation> {
        self.characters.iter().find(|o| o.character_id == char_id)
    }

    /// Returns the ouroboros formation slot for the given Ouroboros ID. (1-6)
    pub fn ouroboros(&self, ouro_id: u16) -> Option<&OuroborosFormation> {
        self.ouroboros.iter().find(|o| o.ouroboros_id == ouro_id)
    }

    /// Returns a mutable character slot for the given character ID. (starts at 1)
    ///
    /// The function searches for the character's slot, and returns a mutable
    /// reference to the first empty slot if it can't be found.
    ///
    /// ## Panics
    /// The function panics if the character's slot could not be found, and there
    /// were no empty slots to edit.
    pub fn character_mut(&mut self, char_id: u16) -> &mut CharacterFormation {
        // Can't iter_mut twice because of borrow checker limitations
        for (i, slot) in self.characters.iter().enumerate() {
            if slot.character_id == char_id {
                return &mut self.characters[i];
            }
        }
        let slot = self
            .characters
            .iter_mut()
            .find(|c| c.character_id == 0)
            .expect("no suitable slot for ouro ID");
        slot.character_id = char_id;
        slot
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
        let slot = self
            .ouroboros
            .iter_mut()
            .find(|o| o.ouroboros_id == 0)
            .expect("no suitable slot for ouro ID");
        slot.ouroboros_id = ouro_id;
        slot
    }

    /// Clears the saved formation.
    pub fn clear(&mut self) {
        self.party.clear();
    }
}

impl CharacterFormation {
    /// Creates a new character formation from the current state of the save file.
    pub fn from_save(save_char: &Character, char_id: u16) -> Self {
        let current_class = save_char.selected_class;
        Self {
            class: if current_class != 0 {
                *save_char.class_data(current_class as usize)
            } else {
                Default::default()
            },
            current_class: current_class as u16,
            character_id: char_id,
            costume_id: save_char.costume_id,
            attachment: save_char.attachment,
        }
    }
}

impl OuroborosFormation {
    /// Creates a new ouroboros formation from the current state of the save file.
    pub fn from_save(save_ouro: &Ouroboros, char_id: u16) -> Self {
        Self {
            ouroboros_id: char_id,
            art_ids: save_ouro.art_ids,
            linked_skills: save_ouro.linked_skills,
        }
    }

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
