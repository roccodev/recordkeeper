//! Quest IDs, flag IDs, conditions, etc.

pub struct Quest {
    pub id: u32,
    pub name_id: Option<usize>,
    pub flag_id: usize,
    pub purposes: Vec<QuestPurpose>,
    pub has_branch_b: bool,
}

pub struct QuestPurpose {
    pub id: u32,
}
