use bdat::{label_hash, ModernTable};
use game_data::dlc::pow_augment::{AugmentNode, PowAugment};

pub fn read_for_character(table: &ModernTable) -> PowAugment {
    let nodes = table
        .rows()
        .map(|row| {
            let ty = row.get(label_hash!("Type")).to_integer();
            let param = row.get(label_hash!("Param")).to_integer();
            (match ty {
                1 => AugmentNode::UnlockArt,
                2 => AugmentNode::UnlockSkill,
                3 => AugmentNode::UpgradeArt,
                4 => AugmentNode::UpgradeSkill,
                n => panic!("unknown node type {n}"),
            })(param)
        })
        .collect();
    PowAugment { nodes }
}
