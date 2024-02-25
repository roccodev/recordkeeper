use bdat::{label_hash, Label};
use game_data::{
    dlc::{
        pedia::{Dlc4Collepedia, Enemypedia},
        Regional,
    },
    manual::Flag,
};

use crate::BdatRegistry;

pub fn read_collepedia(bdat: &BdatRegistry) -> Regional<Dlc4Collepedia> {
    let table = bdat.table(Label::Hash(0x947C9B0C));
    let mut entries = table
        .rows()
        .map(|row| {
            let region: u16 = row.get(Label::Hash(0x7A94A94B)).get_as();
            let row = Dlc4Collepedia {
                category: row.get(label_hash!("Category")).get_as(),
                item: row.get(label_hash!("ItemID")).get_as(),
                max: row.get(label_hash!("ReqNum")).get_as(),
                flag: Flag {
                    bits: 2,
                    index: row.get(label_hash!("Flag")).get_as::<u16>() as usize,
                },
                sort_id: row.get(label_hash!("SortID")).get_as(),
            };
            (region - 1, row)
        })
        .collect::<Vec<_>>();
    entries.sort_by_key(|&(region, e)| (region, e.category, e.sort_id));
    entries.into_iter().collect()
}

pub fn read_enemypedia(bdat: &BdatRegistry) -> Regional<Enemypedia> {
    let table = bdat.table(Label::Hash(0xB4158056));
    let locations = bdat.table(label_hash!("ma40a_GMK_Location"));
    let mut entries = table
        .rows()
        .map(|row| {
            let region: u32 = row.get(Label::Hash(0x7A94A94B)).get_as();
            let region = locations.row_by_hash(region).id(); // why Monolith?
            let row = Enemypedia {
                enemy: row
                    .get(label_hash!("EnemyID1"))
                    .get_as::<u16>()
                    .try_into()
                    .expect("empty enemy row"),
                max: row.get(label_hash!("ReqNum")).get_as(),
                flag: Flag {
                    bits: 4,
                    index: row.get(Label::Hash(0x32F9A6F1)).get_as::<u16>() as usize,
                },
                sort_id: row.get(label_hash!("SortID")).get_as(),
            };
            (region - 1, row)
        })
        .collect::<Vec<_>>();
    entries.sort_by_key(|&(region, e)| (region, e.sort_id));
    entries.into_iter().collect()
}
