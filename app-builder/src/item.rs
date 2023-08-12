use std::num::NonZeroUsize;

use bdat::{Label, RowRef};
use game_data::item::{Item, Rarity};
use game_data::item::{ItemRegistry, ItemType};

use crate::const_hash;
use crate::BdatRegistry;

pub fn load_items(bdat: &BdatRegistry) -> ItemRegistry {
    let categories = [
        (ItemType::Collection, const_hash!("ITM_Collection")),
        (ItemType::Cylinder, const_hash!("ITM_Cylinder")),
        (ItemType::Accessory, const_hash!("ITM_Accessory")),
        (ItemType::Exchange, const_hash!("ITM_Exchange")),
        (ItemType::Gem, const_hash!("ITM_Gem")),
        (ItemType::Exchange, const_hash!("ITM_Extra")),
        (ItemType::Info, const_hash!("ITM_Info")),
        (ItemType::Precious, const_hash!("ITM_Precious")),
        (ItemType::Collectopedia, const_hash!("ITM_Collepedia")),
    ];

    let mut registry = ItemRegistry::default();

    for (item_type, table_id) in categories {
        let table = bdat.table(&table_id);
        table
            .rows()
            .filter_map(|row| read_item(item_type, table.get_row(row.id()).unwrap()))
            .for_each(|item| registry.register_item(item));
    }

    registry
}

fn read_item(item_type: ItemType, row: RowRef) -> Option<Item> {
    let rarity = row
        .get(&const_hash!("Rarity"))
        .map(|cell| Rarity::try_from(cell.as_single().unwrap().to_integer()).unwrap())
        .unwrap_or(Rarity::Common);
    let mut amount_max = 99;

    match item_type {
        ItemType::Info => amount_max = 1,
        ItemType::Accessory => {
            // Column is absent from no-DLC dumps
            let only_one_flag = row
                .get(&Label::Hash(0xF620E3C8))
                .map(|cell| cell.as_single().unwrap().to_integer() != 0)
                .unwrap_or_default();
            if only_one_flag {
                amount_max = 1;
            }
        }
        _ => {}
    }

    Some(Item {
        id: row.id().try_into().unwrap(),
        name_id: NonZeroUsize::new(
            row[const_hash!("Name")].as_single().unwrap().to_integer() as usize
        ),
        item_type,
        amount_max,
        rarity,
    })
}
