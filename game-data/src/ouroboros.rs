use serde::{Deserialize, Serialize};

use crate::{
    lang::{Filterable, Id},
    GameData, IdInt, LanguageData,
};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct OuroborosRegistry {
    characters: Box<[Ouroboros]>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Ouroboros {
    pub id: IdInt,
    pub name_id: IdInt,
    pub share_slot_flag: IdInt,

    pub tree_nodes: Box<[OuroTreeNode]>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Copy, PartialOrd, Eq, Ord)]
pub enum OuroTreeNode {
    UnlockArt(IdInt),
    UnlockSkill(IdInt),
    UpgradeArt(IdInt),
    UpgradeSkill(IdInt),
}

impl OuroborosRegistry {
    pub fn new(characters: impl IntoIterator<Item = Ouroboros>) -> Self {
        Self {
            characters: characters.into_iter().collect(),
        }
    }

    pub fn get(&self, id: u32) -> Option<&Ouroboros> {
        id.checked_sub(1)
            .and_then(|i| self.characters.get(i as usize))
    }

    pub fn as_slice(&self) -> &[Ouroboros] {
        &self.characters
    }
}

impl Ouroboros {
    pub fn tree_nodes(&self) -> impl Iterator<Item = (usize, OuroTreeNode)> + '_ {
        self.tree_nodes.iter().enumerate().map(|(i, n)| (i + 1, *n))
    }
}

impl OuroTreeNode {
    pub fn get_param_name<'l>(&self, game: &GameData, lang: &'l LanguageData) -> Option<&'l str> {
        match self {
            OuroTreeNode::UnlockArt(id) | Self::UpgradeArt(id) => game
                .characters
                .get_art(*id)
                .and_then(|a| a.get_filter(lang)),
            OuroTreeNode::UnlockSkill(id) | OuroTreeNode::UpgradeSkill(id) => game
                .characters
                .get_skill(*id)
                .and_then(|s| s.get_filter(lang)),
        }
        .map(|t| t.text())
    }
}

impl Filterable for Ouroboros {
    fn get_filter<'l>(
        &self,
        language: &'l crate::LanguageData,
    ) -> Option<&'l crate::lang::FilterEntry> {
        language.characters.characters.get(self.name_id)
    }
}

impl Id for Ouroboros {
    fn id(&self) -> IdInt {
        self.id
    }
}
