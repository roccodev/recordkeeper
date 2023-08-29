//! Item types, names, max amounts, etc.

use std::num::NonZeroUsize;

use crate::lang::{Nameable, TextEntry, TextTable};
use enum_map::{Enum, EnumMap};
use serde::{Deserialize, Serialize};

use crate::LanguageData;

#[derive(Serialize, Deserialize, Default)]
pub struct ItemRegistry {
    items: EnumMap<ItemType, Vec<Item>>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Item {
    pub id: u32,
    pub name_id: Option<NonZeroUsize>,
    pub item_type: ItemType,
    pub amount_max: u32,
    pub rarity: Rarity,
}

#[derive(Serialize, Deserialize, Enum, PartialEq, Clone, Copy)]
pub enum ItemType {
    Cylinder = 1,
    Gem = 2,
    Collection = 3,
    Info = 4,
    Accessory = 5,
    Precious = 6,
    Exchange = 7,
    Extra = 8,
    Collectopedia = 9,
}

#[derive(Serialize, Deserialize, PartialEq, Copy, Clone)]
pub enum Rarity {
    Common,
    Rare,
    Legendary,
}

#[derive(Debug)]
pub struct RarityFromIntError;

#[derive(Serialize, Deserialize)]
pub struct ItemLanguageRegistry {
    tables: EnumMap<ItemType, TextTable>,
}

impl ItemRegistry {
    pub fn get_item(&self, item_type: ItemType, id: u32) -> Option<&Item> {
        let items = &self.items[item_type];
        items
            .binary_search_by_key(&id, |item| item.id)
            .ok()
            .map(|idx| &items[idx])
    }

    pub fn register_item(&mut self, item: Item) {
        let items = &mut self.items[item.item_type];
        let index = items
            .binary_search_by_key(&item.id, |item| item.id)
            .expect_err("duplicate item");
        items.insert(index, item);
    }

    pub fn items_by_type(&self, item_type: ItemType) -> &[Item] {
        &self.items[item_type]
    }
}

impl ItemLanguageRegistry {
    pub fn new(tables: EnumMap<ItemType, TextTable>) -> Self {
        Self { tables }
    }
}

impl ItemType {
    pub fn lang_id(self) -> &'static str {
        match self {
            Self::Cylinder => "cylinder",
            Self::Gem => "gem",
            Self::Collection => "collection",
            Self::Collectopedia => "collepedia",
            Self::Info => "info",
            Self::Accessory => "accessory",
            Self::Precious => "precious",
            Self::Exchange => "exchange",
            Self::Extra => "extra",
        }
    }
}

impl Rarity {
    pub fn lang_id(self) -> &'static str {
        match self {
            Self::Common => "common",
            Self::Rare => "rare",
            Self::Legendary => "legendary",
        }
    }
}

impl Nameable for Item {
    fn get_name<'l>(&self, language: &'l LanguageData) -> Option<&'l TextEntry> {
        self.name_id
            .and_then(|id| language.items.tables[self.item_type].get(id.get()))
    }
}

impl TryFrom<u32> for Rarity {
    type Error = RarityFromIntError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Rarity::Common,
            1 => Rarity::Rare,
            2 => Rarity::Legendary,
            _ => return Err(RarityFromIntError),
        })
    }
}
