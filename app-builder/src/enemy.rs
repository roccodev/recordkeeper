use bdat::{label_hash, TableAccessor};
use game_data::enemy::{EnemyLang, EnemyRegistry, UniqueMonster};

use crate::{lang::text_table_from_bdat, BdatRegistry, LangBdatRegistry};

pub fn read_data(bdat: &BdatRegistry) -> EnemyRegistry {
    let uniques = bdat.table(label_hash!("FLD_UMonsterList"));
    let enemies = bdat.table(label_hash!("FLD_EnemyData"));

    let uniques = uniques
        .rows()
        .map(|row| {
            let enemy = row.get(label_hash!("EnemyID1")).to_integer() as usize;
            let map = row.get(label_hash!("Zone")).to_integer() as usize;
            let group = row.get(label_hash!("GroupName")).to_integer() as usize;

            let enemy_name = enemies.row(enemy).get(label_hash!("MsgName")).to_integer() as usize;

            UniqueMonster {
                id: row.id(),
                map_id: map,
                name_id: enemy_name,
                group_name: (group != 0).then_some(group),
            }
        })
        .collect();

    EnemyRegistry {
        unique_monsters: uniques,
    }
}

pub fn read_lang(bdat: &LangBdatRegistry) -> EnemyLang {
    let enemies = bdat.table(label_hash!("msg_enemy_name"));
    let enemy_groups = bdat.table(label_hash!("msg_enemy_group_name"));

    EnemyLang {
        enemies: text_table_from_bdat(enemies),
        enemy_groups: text_table_from_bdat(enemy_groups),
    }
}
