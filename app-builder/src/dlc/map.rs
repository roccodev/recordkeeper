use bdat::{hash::murmur3_str, label_hash, Label};
use game_data::{
    dlc::map::{AchievementName, AchievementSearch, Dlc4Map, Dlc4MapLang, MapAchievement},
    manual::Flag,
};

use crate::{
    gimmick::GimmickData, lang::filter_table_from_bdat, BdatRegistry, LangBdatRegistry, ModernRow,
};

const FLAG_BASE_NAMED: u16 = 5419;
const FLAG_BASE_ENEMY: usize = 12158;
const FLAG_BASE_LOCATION: usize = 6850;
const FLAG_BASE_RESCUE: usize = 12516;

const FLAG_BASE_ARCH_LADDER: usize = 14118;
const FLAG_BASE_ARCH_ELEVATOR: usize = 15119;
// (Broken) rest spots that were repaired (unused?)
const FLAG_BASE_ARCH_COM_SPOT: usize = 16120;
// Ether masts ("towers")
const FLAG_BASE_ARCH_TOWER: usize = 17121;
const FLAG_BASE_ARCH_SLIDE: usize = 17222;
const FLAG_BASE_ARCH_LIFT: usize = 17323;

const ACHIEVEMENT_BASE_FLAGS: &[usize] = &[
    usize::MAX,         // 0: ignored
    10055,              // 1: containers
    11056,              // 2: relics
    11157,              // 3: ether channels
    FLAG_BASE_LOCATION, // 4: locations and rest spots
    7851,               // 5: landmarks
    8852,               // 6: secret areas
    usize::MAX,         // 7: architecture (see above)
    FLAG_BASE_RESCUE,   // 8: npc rescue
    9853,               // 9: kizuna events
    FLAG_BASE_LOCATION, // 10: rest spots (same as locations)
    FLAG_BASE_ENEMY,    // 11: unique monsters
    12415,              // 12: area battle
    FLAG_BASE_RESCUE,   // 13: enemy affordance (?, seems related to npc rescue)
    14017,              // 14: fog rifts
];

pub fn read_game(bdat: &BdatRegistry) -> Dlc4Map {
    let achievements = bdat.table(label_hash!("FLD_MapAchievementSearch"));

    let mut maps = {
        const V: Vec<MapAchievement> = Vec::new();
        [V; 5]
    };

    for row in achievements.rows() {
        let achievement = read_achievement(bdat, row);
        maps[achievement.region as usize - 1].push(achievement);
    }

    Dlc4Map::new(maps.map(Vec::into_boxed_slice))
}

pub fn read_lang(lang: &LangBdatRegistry) -> Dlc4MapLang {
    let map = lang.table(label_hash!("msg_mnu_map_ms"));

    Dlc4MapLang {
        map: filter_table_from_bdat(map),
    }
}

fn read_achievement(bdat: &BdatRegistry, row: ModernRow) -> MapAchievement {
    let ty = row.get(label_hash!("Type")).get_as::<u8>() as u32;
    let region = row.get(Label::Hash(0x09F6EF1A)).get_as::<u16>() as u32;
    MapAchievement {
        region,
        ty,
        searches: read_searches(bdat, row, ty),
    }
}

fn read_searches(bdat: &BdatRegistry, row: ModernRow, ty: u32) -> Box<[AchievementSearch]> {
    let enemies = bdat.table(label_hash!("FLD_EnemyData"));
    let npc_rescues = bdat.table(Label::Hash(0x46B9A047));

    [
        0x0413A76D, 0x57943DB0, 0x7D81DF13, 0x0F1AB3D0, 0x7BB8F985, 0xB5B04756, 0x29C606AD,
        0xAD3C17A1, 0xA2435EF2, 0x7B22FA78, 0xE4AB854A, 0xB35F24DA, 0xA33EB40E, 0x95B8C53B,
        0x52752D41, 0xE656735B, 0x755CF8CE, 0x9301A5BA, 0xFA22DEF5, 0x190E7C40, 0xB027AEE0,
        0xC58331C0, 0xB861D3C4, 0x6C872AA2, 0x7F2D5B09, 0x39D18182, 0x4F5632E7, 0xEAF3E5BC,
        0xA1A6BB84, 0x35BBE75C, 0x04EFE638, 0xB938044F, 0xE6D89975, 0x2F9E4C28, 0xF37E92F5,
        0xA362D217, 0xFB7AD656, 0x495BE0B8, 0x7F4DCE37, 0x9CA3F7A6,
    ]
    .into_iter()
    .map(Label::Hash)
    .map(|field| row.get(field).get_as())
    .filter_map(|gmk| {
        if gmk == 0 {
            return None;
        }
        Some(match ty {
            11 => {
                // Enemy
                let enemy = enemies.row_by_hash(gmk);
                let flag_offset =
                    enemy.get(label_hash!("NamedFlag")).get_as::<u16>() - FLAG_BASE_NAMED;
                AchievementSearch {
                    flag: Flag {
                        bits: 2,
                        index: flag_offset as usize + FLAG_BASE_ENEMY,
                    },
                    name: AchievementName::Enemy(enemy.id() as u32),
                }
            }
            8 => {
                // NPC
                let rescue = npc_rescues.row_by_hash(gmk);
                AchievementSearch {
                    flag: Flag {
                        bits: 2,
                        index: rescue.get(Label::Hash(0x0DEC588C)).get_as::<u16>() as usize,
                    },
                    name: AchievementName::Npc(
                        rescue.get(label_hash!("NpcID")).get_as::<u16>() as u32
                    ),
                }
            }
            _ => gimmick_search(bdat, bdat.gimmicks.get(&gmk).expect("unknown gimmick"), ty),
        })
    })
    .collect()
}

fn gimmick_search<'a>(
    bdat: &'a BdatRegistry,
    mut gmk: &'a GimmickData,
    ty: u32,
) -> AchievementSearch {
    if gmk.type_hash == murmur3_str("Architecture") {
        let arch = bdat
            .table(label_hash!("FLD_Architecture"))
            .row_by_hash(gmk.external_id);
        let discrim = arch.get(Label::Hash(0x67B1D80A)).get_as::<u8>();
        if [0, 2, 3].contains(&discrim) {
            gmk = bdat
                .gimmicks
                .get(&arch.get(Label::Hash(0x8B6F5AD3)).get_as())
                .expect("unknown external gimmick");
        }
    }

    let flag;
    let name = AchievementName::Unknown(gmk.row_id);

    match gmk.type_hash {
        h if h == murmur3_str("Architecture") => {
            let arch = bdat
                .table(label_hash!("FLD_Architecture"))
                .row_by_hash(gmk.external_id);
            let discrim = arch.get(Label::Hash(0x67B1D80A)).get_as::<u8>();
            let base_flag = match discrim {
                1 => FLAG_BASE_ARCH_LADDER,
                4 => FLAG_BASE_ARCH_TOWER,
                5 => FLAG_BASE_ARCH_COM_SPOT,
                0 | 2 | 3 => unreachable!(),
                d => panic!("unknown architecture type {d}"),
            };
            flag = base_flag + gmk.sequential_id as usize;
        }
        h if h == murmur3_str("Elevator") => {
            flag = FLAG_BASE_ARCH_ELEVATOR + gmk.sequential_id as usize;
        }
        h if h == murmur3_str("KizunaEvent") => {
            let event = bdat
                .table(label_hash!("ma40a_GMK_KizunaEvent"))
                .row_by_hash(gmk.external_id);
            flag = event.get(label_hash!("Flag")).get_as::<u16>() as usize;
        }
        h if h == murmur3_str("EtherSlide") => {
            let slide = bdat
                .table(label_hash!("FLD_EtherSlide"))
                .row_by_hash(gmk.external_id);
            let base_flag = if slide.get(label_hash!("Type")).get_as::<u8>() == 1 {
                FLAG_BASE_ARCH_LIFT
            } else {
                FLAG_BASE_ARCH_SLIDE
            };
            flag = base_flag + gmk.sequential_id as usize;
        }
        _ => {
            flag = ACHIEVEMENT_BASE_FLAGS[ty as usize] + gmk.sequential_id as usize;
        }
    }

    AchievementSearch {
        flag: Flag {
            bits: 2,
            index: flag,
        },
        name,
    }
}
