use bdat::{hash::murmur3_str, Label, RowRef, Table};
use game_data::quest::{PurposeTask, Quest, QuestLang, QuestPurpose, QuestRegistry, TaskType};

use crate::{const_hash, lang::text_table_from_bdat, BdatRegistry, LangBdatRegistry};

pub fn read_quests(bdat: &BdatRegistry) -> QuestRegistry {
    let quests = bdat.table(&const_hash!("QST_List"));
    let purposes = bdat.table(&const_hash!("QST_Purpose"));
    let tasks = bdat.table(&const_hash!("QST_Task"));

    // This is an empty row with parameters set to 0.
    // The game usually includes these rows as spacers
    let min_dlc4_quest = quests
        .get_row_by_hash(0xF7E2ACDC)
        .expect("no dlc4 quest marker")
        .id();

    let mut quests = quests
        .rows()
        .map(|r| read_quest(&quests.row(r.id())))
        .collect::<Vec<_>>();

    for row in purposes.rows().map(|r| purposes.row(r.id())) {
        read_purpose(&row, &tasks, &mut quests);
    }

    QuestRegistry::new(quests, min_dlc4_quest)
}

pub fn read_quest_lang(bdat: &LangBdatRegistry) -> QuestLang {
    QuestLang::new(text_table_from_bdat(
        bdat.table(&const_hash!("msg_qst_task")),
    ))
}

fn read_quest(row: &RowRef) -> Quest {
    let flag = row[const_hash!("FlagPrt")]
        .as_single()
        .unwrap()
        .to_integer() as usize;
    let name_id = row[const_hash!("QuestTitle")]
        .as_single()
        .unwrap()
        .to_integer() as usize;

    Quest {
        id: row.id(),
        name_id: (name_id != 0).then_some(name_id),
        flag,
        purposes: vec![],
    }
}

fn read_purpose(row: &RowRef, task_table: &Table, quests: &mut Vec<Quest>) {
    let quest_id = row[const_hash!("QuestID")]
        .as_single()
        .unwrap()
        .to_integer() as usize;
    // FlagLd???
    let flag = row[const_hash!("Flagld")].as_single().unwrap().to_integer() as usize;

    let task_row = row[const_hash!("TaskID")].as_single().unwrap().to_integer() as usize;
    let task_row = task_table.row(task_row);

    let Ok(tasks) = (1..=4)
        .map(|i| read_task(&task_row, i))
        .collect::<Vec<_>>()
        .try_into() else { unreachable!() };

    quests[quest_id.checked_sub(1).expect("quest ID out of bounds")]
        .purposes
        .push(QuestPurpose {
            id: row.id(),
            flag,
            tasks,
        });
}

fn read_task(row: &RowRef, id: usize) -> Option<PurposeTask> {
    let task_id = row[Label::Hash(murmur3_str(&format!("TaskID{id}")))]
        .as_single()
        .unwrap()
        .to_integer() as usize;
    if task_id == 0 {
        return None;
    }
    let ty = row[Label::Hash(murmur3_str(&format!("TaskType{id}")))]
        .as_single()
        .unwrap()
        .to_integer() as usize;
    let branch = row[Label::Hash(murmur3_str(&format!("Branch{id}")))]
        .as_single()
        .unwrap()
        .to_integer();
    let name = row[Label::Hash(murmur3_str(&format!("TaskLog{id}")))]
        .as_single()
        .unwrap()
        .to_integer() as usize;
    let flag = row[Label::Hash(murmur3_str(&format!("TaskFlag{id}")))]
        .as_single()
        .unwrap()
        .to_integer() as usize;

    Some(PurposeTask {
        id: task_id,
        name_id: (name != 0).then_some(name),
        task_type: TaskType::from_repr(ty).expect("unknown type"),
        branch: branch.try_into().unwrap(),
        flag,
    })
}
