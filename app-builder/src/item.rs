use std::num::NonZeroUsize;

use bdat::{label_hash, Label};
use enum_map::enum_map;
use game_data::item::{GemCategory, Item, ItemDetails, ItemLanguageRegistry, Rarity};
use game_data::item::{ItemRegistry, Type};
use recordkeeper::item::ItemType;

use crate::lang::filter_table_from_bdat;
use crate::LangBdatRegistry;
use crate::{BdatRegistry, ModernRow};

pub fn load_items(bdat: &BdatRegistry) -> ItemRegistry {
    let categories = [
        (ItemType::Collection, label_hash!("ITM_Collection")),
        (ItemType::Cylinder, label_hash!("ITM_Cylinder")),
        (ItemType::Accessory, label_hash!("ITM_Accessory")),
        (ItemType::Exchange, label_hash!("ITM_Exchange")),
        (ItemType::Gem, label_hash!("ITM_Gem")),
        (ItemType::Extra, label_hash!("ITM_Extra")),
        (ItemType::Info, label_hash!("ITM_Info")),
        (ItemType::Precious, label_hash!("ITM_Precious")),
        (ItemType::Collectopedia, label_hash!("ITM_Collepedia")),
    ];

    let mut registry = ItemRegistry::default();

    for (item_type, table_id) in categories {
        let table = bdat.table(&table_id);
        table
            .rows()
            .filter_map(|row| read_item(item_type, row))
            .for_each(|item| registry.register_item(item));
    }

    let gems = bdat.table(label_hash!("ITM_Gem"));
    let mut registered = 0u64;
    for row in gems.rows() {
        let category = row.get(label_hash!("Category")).to_integer();
        if registered & (1 << category) == 0 {
            registered |= 1 << category;

            let name_id = row.get(label_hash!("Name")).to_integer() as usize;

            registry.register_gem_category(GemCategory {
                id: category,
                name_id,
            });
        }
    }

    registry
}

pub fn load_item_lang(bdat: &LangBdatRegistry) -> ItemLanguageRegistry {
    let categories = enum_map! {
        Type(ItemType::Collection) => label_hash!("msg_item_collection"),
        Type(ItemType::Cylinder) => label_hash!("msg_item_cylinder"),
        Type(ItemType::Accessory) => label_hash!("msg_item_accessory"),
        Type(ItemType::Exchange) => label_hash!("msg_item_exchange"),
        Type(ItemType::Gem) => label_hash!("msg_item_gem"),
        Type(ItemType::Extra) => label_hash!("msg_item_extra"),
        Type(ItemType::Info) => Label::Hash(0xCA2198EC),
        Type(ItemType::Precious) => label_hash!("msg_item_precious"),
        Type(ItemType::Collectopedia) => Label::Hash(0xBEDB6533),
    };

    ItemLanguageRegistry::new(categories.map(|_, label| filter_table_from_bdat(bdat.table(&label))))
}

fn read_item(item_type: ItemType, row: ModernRow) -> Option<Item> {
    let rarity = row
        .get_if_present(label_hash!("Rarity"))
        .map(|cell| Rarity::try_from(cell.to_integer()).unwrap())
        .unwrap_or(Rarity::Common);
    let mut amount_max = 99;

    match item_type {
        ItemType::Info => amount_max = 1,
        ItemType::Accessory => {
            // Column is absent from no-DLC dumps
            let only_one_flag = row
                .get_if_present(Label::Hash(0xF620E3C8))
                .map(|cell| cell.to_integer() != 0)
                .unwrap_or_default();
            if only_one_flag {
                amount_max = 1;
            }
        }
        _ => {}
    }

    let details = match item_type {
        ItemType::Accessory => {
            row.get_if_present(Label::Hash(0xF620E3C8))
                .map(|cell| ItemDetails::Accessory {
                    is_manual: cell.to_integer() != 0,
                })
        }
        _ => None,
    };

    Some(Item {
        id: row.id().try_into().unwrap(),
        name_id: NonZeroUsize::new(row.get(label_hash!("Name")).to_integer() as usize),
        item_type: Type(item_type),
        amount_max,
        rarity,
        details,
    })
}
