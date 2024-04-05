//! `sortkey.bin` reader. We prefer this over manual sorting because it can handle JP alphabetical
//! ordering.

use bdat::{BdatVersion, Label};
use binrw::BinRead;
use game_data::{lang::SortKey, IdInt};

#[derive(BinRead, Debug)]
#[br(little)]
pub struct SortKeys {
    #[br(align_after = 64)]
    table_count: u32,
    #[br(count = table_count)]
    tables: Vec<BdatKey>,
}

#[derive(BinRead, Debug)]
#[br(little, magic = b"KEY\0")]
pub struct BdatKey {
    entry_count: u32,
    #[br(align_after = 64)]
    bdat_hash: u32,
    #[br(count = entry_count, align_after = 64)]
    entries: Vec<u32>,
}

impl SortKeys {
    pub fn table(&self, name: Label) -> &BdatKey {
        let Label::Hash(hash) = name.into_hash(BdatVersion::Modern) else {
            unreachable!()
        };
        self.tables
            .iter()
            .find(|t| t.bdat_hash == hash)
            .expect("table not found")
    }
}

impl BdatKey {
    /// Gets the sort key for the given BDAT row index.
    ///
    /// The index starts at 0, so it should be `row ID - table base ID`.
    pub fn get(&self, index: usize) -> u32 {
        self.entries[index]
    }

    /// Converts this key table to a [`SortKey`] which is indexed by
    /// row ID.
    ///
    /// This function takes an ID->name ID mapping and returns the order
    /// in which the (former) IDs should appear.
    pub fn to_id_key(&self, id_name_map: impl IntoIterator<Item = (IdInt, IdInt)>) -> SortKey {
        let base_id = 1; // Common to all lang tables
        let mut id_name_map: Vec<_> = id_name_map.into_iter().collect();
        id_name_map.sort_unstable_by_key(|(_, name_id)| self.get((name_id - base_id) as usize));
        id_name_map.into_iter().map(|(id, _)| id).collect()
    }
}
