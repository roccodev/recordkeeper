use bdat::{label_hash, Label};
use game_data::ouroboros::OuroTreeNode;
use game_data::ouroboros::Ouroboros;
use game_data::ouroboros::OuroborosRegistry;

use crate::BdatRegistry;

static TABLES: [Label; 6] = [
    Label::Hash(0xE0E1A313),
    Label::Hash(0x15728D92),
    Label::Hash(0x91F54F44),
    Label::Hash(0x758629C4),
    Label::Hash(0xF58F35C6),
    Label::Hash(0x9A4C2763),
];

pub fn read_ouroboros(bdat: &BdatRegistry) -> OuroborosRegistry {
    let ouro_table = bdat.table(label_hash!("CHR_UroBody"));

    OuroborosRegistry::new(ouro_table.rows().take(TABLES.len()).map(|row| {
        let name_id = row.get(label_hash!("Name")).to_integer();
        let share_slot_flag = row.get(label_hash!("Flag_ShareSlot")).to_integer();

        Ouroboros {
            id: row.id(),
            name_id,
            share_slot_flag,
            tree_nodes: read_ouro_tree(bdat, row.id().checked_sub(1).unwrap() as usize),
        }
    }))
}

fn read_ouro_tree(bdat: &BdatRegistry, i: usize) -> Box<[OuroTreeNode]> {
    let table = bdat.table(&TABLES[i]);
    let mut tree: Box<[OuroTreeNode]> = table
        .rows()
        .map(|row| {
            let ty = row.get(label_hash!("Type")).to_integer();
            let param = row.get(label_hash!("Param")).to_integer();

            (match ty {
                1 => OuroTreeNode::UnlockArt,
                2 => OuroTreeNode::UnlockSkill,
                3 => OuroTreeNode::UpgradeArt,
                4 => OuroTreeNode::UpgradeSkill,
                n => panic!("unknown node type {n}"),
            })(param)
        })
        .collect();
    tree.sort_unstable();
    tree
}
