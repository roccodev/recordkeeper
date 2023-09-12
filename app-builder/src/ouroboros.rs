use std::rc::Rc;

use bdat::Label;
use game_data::ouroboros::OuroTreeNode;
use game_data::ouroboros::Ouroboros;
use game_data::ouroboros::OuroborosRegistry;

use crate::const_hash;
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
    let ouro_table = bdat.table(&const_hash!("CHR_UroBody"));

    OuroborosRegistry::new(
        ouro_table
            .rows()
            .take(TABLES.len())
            .map(|r| ouro_table.row(r.id()))
            .map(|row| {
                let name_id = row[const_hash!("Name")].as_single().unwrap().to_integer() as usize;
                let share_slot_flag = row[const_hash!("Flag_ShareSlot")]
                    .as_single()
                    .unwrap()
                    .to_integer() as usize;

                Ouroboros {
                    id: row.id(),
                    name_id,
                    share_slot_flag,
                    tree_nodes: read_ouro_tree(bdat, row.id().checked_sub(1).unwrap()),
                }
            }),
    )
}

fn read_ouro_tree(bdat: &BdatRegistry, i: usize) -> Box<[OuroTreeNode]> {
    let table = bdat.table(&TABLES[i]);
    let mut tree: Box<[OuroTreeNode]> = table
        .rows()
        .map(|r| table.row(r.id()))
        .map(|row| {
            let ty = row[const_hash!("Type")].as_single().unwrap().to_integer();
            let param = row[const_hash!("Param")].as_single().unwrap().to_integer() as usize;

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
