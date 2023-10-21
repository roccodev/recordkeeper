use serde::{Deserialize, Serialize};

use crate::{lang::Filterable, GameData, LanguageData};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct PowAugment {
    pub nodes: Box<[AugmentNode]>,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum AugmentNode {
    UnlockArt(u32) = 1,
    UpgradeArt(u32) = 2,
    UnlockSkill(u32) = 3,
    UpgradeSkill(u32) = 4,
}

impl AugmentNode {
    pub fn get_param_name<'l>(&self, game: &GameData, lang: &'l LanguageData) -> Option<&'l str> {
        match self {
            Self::UnlockArt(id) | Self::UpgradeArt(id) => game
                .characters
                .get_art(*id as usize)
                .and_then(|a| a.get_filter(lang)),
            Self::UnlockSkill(id) | Self::UpgradeSkill(id) => game
                .characters
                .get_skill(*id as usize)
                .and_then(|s| s.get_filter(lang)),
        }
        .map(|t| t.text())
    }
}
