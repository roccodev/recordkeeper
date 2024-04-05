use std::collections::HashMap;

use bdat::{label_hash, Label};
use game_data::{
    dlc::community::{CommunityTask, DlcCommunity, DlcCommunityLang, NpcCommunity},
    manual::Flag,
    IdInt,
};

use crate::{
    lang::{sort_key_from_bdat, text_table_from_bdat},
    npc::read_npc,
    BdatRegistry, LangBdatRegistry,
};

pub fn read_game(bdat: &BdatRegistry) -> DlcCommunity {
    let npcs = bdat.table(label_hash!("FLD_NpcList"));
    let challenges = bdat.table(Label::Hash(0xA4A71A39));

    let (npcs, npc_community) = npcs
        .rows()
        .filter_map(|npc| {
            let flag = npc.get(label_hash!("HitonowaFlag")).to_integer();
            if flag == 0 {
                return None;
            }
            let challenge = challenges.row(npc.get(label_hash!("HitonowaChallenge")).to_integer());
            let tasks = (1..=4)
                .map(|i| {
                    let ty = challenge
                        .get(label_hash!(format!("ChallengeType{i}")))
                        .to_integer();
                    let task = challenge
                        .get(label_hash!(format!("ChallengeTask{i}")))
                        .to_integer();
                    (ty, task)
                })
                .take_while(|(_, id)| *id != 0)
                .map(|(ty, task)| read_task(bdat, ty, task, npc.id() as u32))
                .collect();
            let order_flag = npc.get(Label::Hash(0xE7AB0B6E)).to_integer();
            let npc = read_npc(bdat, npc.id() as u32);
            Some((
                npc,
                NpcCommunity {
                    progress_flag: Flag {
                        bits: 2,
                        index: flag,
                    },
                    order_flag: Flag {
                        bits: 8,
                        index: order_flag,
                    },
                    tasks,
                },
            ))
        })
        .fold(
            (Vec::new(), HashMap::new()),
            |(mut npcs, mut map), (npc, comm)| {
                map.insert(npc.id, comm);
                npcs.push(npc);
                (npcs, map)
            },
        );

    DlcCommunity::new(npc_community, npcs)
}

pub fn read_lang(bdat: &LangBdatRegistry) -> DlcCommunityLang {
    let conditions = bdat.table(&Label::Hash(0x68394577));

    // Transform NPC name mapping from Hash-> to ID->
    let names = bdat.table(label_hash!("msg_npc_name"));
    let names_key = bdat.sort_keys.table(label_hash!("msg_npc_name"));
    let resources = bdat.table(label_hash!("FLD_NpcResource"));
    let npc_names = bdat
        .table(label_hash!("FLD_NpcList"))
        .rows()
        .filter_map(|npc| {
            let flag = npc.get(label_hash!("HitonowaFlag")).to_integer();
            if flag == 0 {
                return None;
            }
            let res = resources.row(npc.get(Label::Hash(0x7F0A3296)).get_as::<u16>().into());
            let name_id_hash = res.get(label_hash!("Name")).to_integer();
            Some((npc.id(), names.row_by_hash(name_id_hash).id()))
        });

    DlcCommunityLang {
        condition_lang: text_table_from_bdat(conditions),
        npc_sort: names_key.to_id_key(npc_names),
    }
}

fn read_task(bdat: &BdatRegistry, ty: u32, id: IdInt, npc_id: u32) -> CommunityTask {
    match ty {
        0 => {
            let talk_task = bdat.table(Label::Hash(0x8A6DA2C6)).row(id);
            CommunityTask::Talk {
                npc_id,
                flag: Flag {
                    bits: 4,
                    index: talk_task.get(Label::Hash(0x44C70F2F)).to_integer(),
                },
                max: talk_task.get(Label::Hash(0xB1DDC202)).to_integer(),
            }
        }
        1 => {
            let quest_task = bdat.table(Label::Hash(0x88C90810)).row(id);
            CommunityTask::Quest {
                quest_id: quest_task.get(Label::Hash(0x9E29EB5F)).to_integer(),
            }
        }
        2 => {
            let condition_task = bdat.table(Label::Hash(0x2C1E4B90)).row(id);
            let progress_flag = condition_task.get(Label::Hash(0x982C82A8)).to_integer();
            CommunityTask::Condition {
                msg_id: condition_task.get(Label::Hash(0xE853AC27)).to_integer(),
                progress_flag: (progress_flag != 0).then_some(Flag {
                    bits: 8,
                    index: progress_flag,
                }),
            }
        }
        n => panic!("unknown challenge task {n}"),
    }
}
