use game_data::enemy::{EnemyLang, EnemyRegistry, UniqueMonster};

use crate::{const_hash, lang::text_table_from_bdat, BdatRegistry, LangBdatRegistry};

pub fn read_data(bdat: &BdatRegistry) -> EnemyRegistry {
    let uniques = bdat.table(&const_hash!("FLD_UMonsterList"));
    let enemies = bdat.table(&const_hash!("FLD_EnemyData"));

    let uniques = uniques
        .rows()
        .map(|r| uniques.row(r.id()))
        .map(|row| {
            let enemy = row[const_hash!("EnemyID1")]
                .as_single()
                .unwrap()
                .to_integer() as usize;
            let map = row[const_hash!("Zone")].as_single().unwrap().to_integer() as usize;
            let group = row[const_hash!("GroupName")]
                .as_single()
                .unwrap()
                .to_integer() as usize;

            let enemy_name = enemies.row(enemy)[const_hash!("MsgName")]
                .as_single()
                .unwrap()
                .to_integer() as usize;

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
    let enemies = bdat.table(&const_hash!("msg_enemy_name"));
    let enemy_groups = bdat.table(&const_hash!("msg_enemy_group_name"));

    EnemyLang {
        enemies: text_table_from_bdat(enemies),
        enemy_groups: text_table_from_bdat(enemy_groups),
    }
}
