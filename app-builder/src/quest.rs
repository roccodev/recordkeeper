use bdat::{
    label_hash,
    modern::{ModernRowRef, ModernTable},
};
use game_data::quest::{PurposeTask, Quest, QuestLang, QuestPurpose, QuestRegistry, TaskType};

use crate::{lang::text_table_from_bdat, BdatRegistry, LangBdatRegistry};

pub fn read_quests(bdat: &BdatRegistry) -> QuestRegistry {
    let quests = bdat.table(label_hash!("QST_List"));
    let purposes = bdat.table(label_hash!("QST_Purpose"));
    let tasks = bdat.table(label_hash!("QST_Task"));

    // This is an empty row with parameters set to 0.
    // The game usually includes these rows as spacers
    let min_dlc4_quest = quests
        .get_row_by_hash(0xF7E2ACDC)
        .expect("no dlc4 quest marker")
        .id();

    let mut quests = quests.rows().map(read_quest).collect::<Vec<_>>();

    for row in purposes.rows() {
        read_purpose(&row, &tasks, &mut quests);
    }

    QuestRegistry::new(quests, min_dlc4_quest)
}

pub fn read_quest_lang(bdat: &LangBdatRegistry) -> QuestLang {
    QuestLang::new(text_table_from_bdat(
        bdat.table(label_hash!("msg_qst_task")),
    ))
}

fn read_quest(row: ModernRowRef) -> Quest {
    let flag = row.get(label_hash!("FlagPrt")).to_integer();
    let name_id = row.get(label_hash!("QuestTitle")).to_integer();

    Quest {
        id: row.id(),
        name_id: (name_id != 0).then_some(name_id),
        flag,
        purposes: vec![],
    }
}

fn read_purpose(row: &ModernRowRef, task_table: &ModernTable, quests: &mut Vec<Quest>) {
    let quest_id = row.get(label_hash!("QuestID")).to_integer();
    // FlagLd???
    let flag = row.get(label_hash!("Flagld")).to_integer();

    let task_row = row.get(label_hash!("TaskID")).to_integer();
    let task_row = task_table.row(task_row);

    let Ok(tasks) = (1..=4)
        .map(|i| read_task(&task_row, i))
        .collect::<Vec<_>>()
        .try_into()
    else {
        unreachable!()
    };

    quests[quest_id.checked_sub(1).expect("quest ID out of bounds") as usize]
        .purposes
        .push(QuestPurpose {
            id: row.id(),
            flag,
            tasks,
        });
}

fn read_task(row: &ModernRowRef, id: usize) -> Option<PurposeTask> {
    let task_id = row.get(label_hash!(format!("TaskID{id}"))).to_integer();
    if task_id == 0 {
        return None;
    }
    let ty = row.get(label_hash!(format!("TaskType{id}"))).to_integer();
    let branch = row.get(label_hash!(format!("Branch{id}"))).to_integer();
    let name = row.get(label_hash!(format!("TaskLog{id}"))).to_integer();
    let flag = row.get(label_hash!(format!("TaskFlag{id}"))).to_integer();

    Some(PurposeTask {
        id: task_id,
        name_id: (name != 0).then_some(name),
        task_type: TaskType::from_repr(ty as usize).expect("unknown type"),
        branch: branch.try_into().unwrap(),
        flag,
    })
}
