use bdat::{label_hash, Label};
use game_data::{
    lang::{FilterEntry, FilterTable},
    npc::{Npc, NpcLang, NpcRegistry},
};

use crate::{BdatRegistry, LangBdatRegistry};

pub fn read_data(bdat: &BdatRegistry) -> NpcRegistry {
    let npcs = bdat.table(label_hash!("FLD_NpcList"));
    NpcRegistry::new(npcs.rows().map(|npc| read_npc(bdat, npc.id() as u32)))
}

pub fn read_lang(bdat: &LangBdatRegistry) -> NpcLang {
    let npcs = bdat.table(label_hash!("msg_npc_name"));
    NpcLang {
        npc_names: FilterTable::new(npcs.rows().filter_map(|row| {
            let text = row.get(label_hash!("name")).as_str();
            (!text.is_empty()).then(|| FilterEntry::new(text, row.id_hash().unwrap()))
        })),
    }
}

pub fn read_npc(bdat: &BdatRegistry, npc_id: u32) -> Npc {
    let resources = bdat.table(label_hash!("FLD_NpcResource"));
    let npc = bdat.table(label_hash!("FLD_NpcList")).row(npc_id);
    let res = resources.row(npc.get(Label::Hash(0x7F0A3296)).get_as::<u16>().into());
    Npc {
        id: npc.id() as u32,
        name_id_hash: res.get(label_hash!("Name")).to_integer(),
    }
}
