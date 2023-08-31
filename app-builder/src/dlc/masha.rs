use bdat::{hash::murmur3_str, Label};
use game_data::dlc::masha::{CraftEnhance, GameCraftItems, LangCraftItems};

use crate::{const_hash, lang::text_table_from_bdat, BdatRegistry, LangBdatRegistry};

pub fn read_game(bdat: &BdatRegistry) -> GameCraftItems {
    let table = bdat.table(&Label::Hash(0x4548D8B2));
    let enhances = table
        .rows()
        .map(|row| {
            let row = table.row(row.id());
            CraftEnhance(
                (1..=5)
                    .map(|i| Label::Hash(murmur3_str(&format!("Enhance{i}"))))
                    .map(|k| row[k].as_single().unwrap().to_integer())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            )
        })
        .collect();
    GameCraftItems { enhances }
}

pub fn read_lang(bdat: &LangBdatRegistry) -> LangCraftItems {
    let msg = text_table_from_bdat(bdat.table(&const_hash!("msg_extra_accessory")));
    let item_types = bdat.table(&Label::Hash(0xE0A85A79));

    LangCraftItems::new(
        msg,
        item_types.rows().map(|row| {
            let row = item_types.row(row.id());
            (
                row.id() as u32,
                row[const_hash!("Name")].as_single().unwrap().to_integer(),
            )
        }),
    )
}
