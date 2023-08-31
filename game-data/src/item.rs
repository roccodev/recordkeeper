//! Item types, names, max amounts, etc.

use std::num::NonZeroUsize;

use crate::lang::{FilterTable, Filterable};
use enum_map::{Enum, EnumArray, EnumMap};
use recordkeeper::item::ItemType;
use serde::{Deserialize, Serialize};

use crate::LanguageData;

#[derive(Serialize, Deserialize, Default)]
pub struct ItemRegistry {
    items: EnumMap<Type, Vec<Item>>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Item {
    pub id: u32,
    pub name_id: Option<NonZeroUsize>,
    pub item_type: Type,
    pub amount_max: u32,
    pub rarity: Rarity,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(try_from = "u32", into = "u32")]
pub struct Type(pub ItemType);

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
    tables: EnumMap<Type, FilterTable>,
}

impl ItemRegistry {
    pub fn get_item(&self, item_type: ItemType, id: u32) -> Option<&Item> {
        let items = &self.items[Type(item_type)];
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
        &self.items[Type(item_type)]
    }
}

impl ItemLanguageRegistry {
    pub fn new(tables: EnumMap<Type, FilterTable>) -> Self {
        Self { tables }
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

impl Filterable for Item {
    fn get_filter<'l>(&self, language: &'l LanguageData) -> Option<&'l crate::lang::FilterEntry> {
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

impl Enum for Type {
    const LENGTH: usize = 9;

    fn from_usize(value: usize) -> Self {
        Self(ItemType::try_from(u32::try_from(value).unwrap() + 1).unwrap())
    }

    fn into_usize(self) -> usize {
        self.0 as u32 as usize - 1
    }
}

impl<T> EnumArray<T> for Type {
    type Array = [T; Self::LENGTH];
}

impl From<Type> for u32 {
    fn from(value: Type) -> Self {
        value.0 as u32
    }
}

impl TryFrom<u32> for Type {
    type Error = <ItemType as TryFrom<u32>>::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        ItemType::try_from(value).map(|t| Self(t))
    }
}
