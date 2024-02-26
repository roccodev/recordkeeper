use std::num::NonZeroU16;

use recordkeeper::item::ItemType;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, FromRepr};

use crate::{lang::Nameable, manual::Flag, GameData, LanguageData};

#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub struct Dlc4Collepedia {
    pub category: u16,
    pub item: u16,
    pub max: u8,
    pub flag: Flag,
    pub sort_id: u8,
}

#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub struct Enemypedia {
    // There can be more than one but they have the same name
    pub enemy: NonZeroU16,
    pub flag: Flag,
    pub sort_id: u16,
    pub max: u8,
    pub slot_id: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, EnumIter, FromRepr)]
pub enum PediaStatus {
    /// Never had the item in inventory
    Unknown = 0,
    /// Item has been picked up at least once (even if amount is currently 0)
    InProgress = 1,
    /// Item registered in the collectopedia
    Complete = 2,
}

pub enum PediaValue {
    Number { max: u8, slot_id: usize },
    TriState,
}

pub trait PediaItem {
    fn flag(&self) -> Flag;
    fn item(&self) -> PediaValue;
    fn get_name<'d>(&self, game: &GameData, lang: &'d LanguageData) -> Option<&'d str>;
}

impl PediaValue {
    pub const fn flag_max(&self) -> u32 {
        match self {
            PediaValue::Number { max, .. } => *max as u32,
            PediaValue::TriState => 2,
        }
    }
}

impl PediaItem for Dlc4Collepedia {
    fn flag(&self) -> Flag {
        self.flag
    }

    fn item(&self) -> PediaValue {
        PediaValue::TriState
    }

    fn get_name<'d>(&self, game: &GameData, lang: &'d LanguageData) -> Option<&'d str> {
        let item = game
            .items
            .get_item(ItemType::Collection, self.item as u32)?;
        item.get_name_str(lang)
    }
}

impl PediaItem for Enemypedia {
    fn flag(&self) -> Flag {
        self.flag
    }

    fn item(&self) -> PediaValue {
        PediaValue::Number {
            max: self.max,
            slot_id: self.slot_id,
        }
    }

    fn get_name<'d>(&self, game: &GameData, lang: &'d LanguageData) -> Option<&'d str> {
        let enemy = game.enemies.get_enemy(self.enemy.into())?;
        enemy.get_name_str(lang)
    }
}
