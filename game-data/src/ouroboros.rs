use serde::{Deserialize, Serialize};

use crate::lang::{Filterable, Id};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct OuroborosRegistry {
    characters: Box<[Ouroboros]>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Ouroboros {
    pub id: usize,
    pub name_id: usize,
    pub share_slot_flag: usize,

    pub tree_nodes: Box<[OuroTreeNode]>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Copy)]
pub enum OuroTreeNode {
    UnlockArt(usize),
    UnlockSkill(usize),
    UpgradeArt(usize),
    UpgradeSkill(usize),
}

impl OuroborosRegistry {
    pub fn new(characters: impl IntoIterator<Item = Ouroboros>) -> Self {
        Self {
            characters: characters.into_iter().collect(),
        }
    }

    pub fn get(&self, id: usize) -> Option<&Ouroboros> {
        id.checked_sub(1).and_then(|i| self.characters.get(i))
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

impl Filterable for Ouroboros {
    fn get_filter<'l>(
        &self,
        language: &'l crate::LanguageData,
    ) -> Option<&'l crate::lang::FilterEntry> {
        language.characters.characters.get(self.name_id)
    }
}

impl Id for Ouroboros {
    fn id(&self) -> usize {
        self.id
    }
}
