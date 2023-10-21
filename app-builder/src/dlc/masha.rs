use bdat::{hash::murmur3_str, label_hash, Label};
use game_data::dlc::masha::{CraftEnhance, GameCraftItems, LangCraftItems};

use crate::{lang::filter_table_from_bdat, BdatRegistry, LangBdatRegistry};

pub fn read_game(bdat: &BdatRegistry) -> GameCraftItems {
    let table = bdat.table(&Label::Hash(0x4548D8B2));
    let enhances = table
        .rows()
        .map(|row| {
            CraftEnhance(
                (1..=5)
                    .map(|i| Label::Hash(murmur3_str(&format!("Enhance{i}"))))
                    .map(|k| row.get(k).to_integer())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            )
        })
        .collect();
    GameCraftItems { enhances }
}

pub fn read_lang(bdat: &LangBdatRegistry) -> LangCraftItems {
    let msg = filter_table_from_bdat(bdat.table(label_hash!("msg_extra_accessory")));
    let item_types = bdat.table(&Label::Hash(0xE0A85A79));

    LangCraftItems::new(
        msg,
        item_types
            .rows()
            .map(|row| (row.id() as u32, row.get(label_hash!("Name")).to_integer())),
    )
}
