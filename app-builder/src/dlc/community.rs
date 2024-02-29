use bdat::{label_hash, Label, TableAccessor};
use game_data::{
    dlc::community::{CommunityTask, DlcCommunity, DlcCommunityLang, NpcCommunity},
    manual::Flag,
};

use crate::{lang::text_table_from_bdat, BdatRegistry, LangBdatRegistry};

pub fn read_game(bdat: &BdatRegistry) -> DlcCommunity {
    let npcs = bdat.table(label_hash!("FLD_NpcList"));
    let challenges = bdat.table(Label::Hash(0xA4A71A39));

    let npc_community = npcs.rows().filter_map(|npc| {
        let flag = npc.get(label_hash!("HitonowaFlag")).to_integer();
        if flag == 0 {
            return None;
        }
        let challenge =
            challenges.row(npc.get(label_hash!("HitonowaChallenge")).to_integer() as usize);
        let tasks = (1..=4)
            .map(|i| {
                let ty = challenge
                    .get(label_hash!(format!("ChallengeType{i}")))
                    .to_integer();
                let task = challenge
                    .get(label_hash!(format!("ChallengeTask{i}")))
                    .to_integer() as usize;
                (ty, task)
            })
            .take_while(|(_, id)| *id != 0)
            .map(|(ty, task)| read_task(bdat, ty, task, npc.id() as u32))
            .collect();
        Some((
            npc.id(),
            NpcCommunity {
                progress_flag: Flag {
                    bits: 2,
                    index: flag as usize,
                },
                order_flag: Flag {
                    bits: 8,
                    index: npc.get(Label::Hash(0xE7AB0B6E)).to_integer() as usize,
                },
                tasks,
            },
        ))
    });

    DlcCommunity::new(npc_community)
}

pub fn read_lang(bdat: &LangBdatRegistry) -> DlcCommunityLang {
    let conditions = bdat.table(&Label::Hash(0x68394577));

    DlcCommunityLang {
        condition_lang: text_table_from_bdat(conditions),
    }
}

fn read_task(bdat: &BdatRegistry, ty: u32, id: usize, npc_id: u32) -> CommunityTask {
    match ty {
        0 => {
            let talk_task = bdat.table(Label::Hash(0x8A6DA2C6)).row(id);
            CommunityTask::Talk {
                npc_id,
                flag: Flag {
                    bits: 4,
                    index: talk_task.get(Label::Hash(0x44C70F2F)).to_integer() as usize,
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
            CommunityTask::Condition {
                msg_id: condition_task.get(Label::Hash(0xE853AC27)).to_integer(),
            }
        }
        n => panic!("unknown challenge task {n}"),
    }
}
