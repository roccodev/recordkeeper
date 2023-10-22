use recordkeeper_macros::SaveBin;

pub const ENEMY_TOMBSTONE_MAX: usize = 200;
pub const SOUL_HACK_ACHIEVEMENT_MAX: usize = 220;

#[derive(SaveBin, Debug)]
pub struct EnemyTombstone {
    /// Highest level rematches, 4 bits for each difficulty
    rematches: [u8; 2],
    /// Whether the player has seen the enemy's target bar at least once.
    /// (Used to show/hide enemy info in the Soulhacker list)
    pub seen: bool,
    /// Defeated on this playthrough
    pub defeated: bool,
    /// One record for each difficulty
    time_records: [TombstoneTime; 4],
}

#[derive(SaveBin, Debug)]
pub struct TombstoneTime {
    pub best_time: u16,
    pub best_time_highest_level: u16,
}

#[derive(SaveBin, Debug)]
pub struct SoulHackAchievements {
    /// Indices from `BTL_Achievement`
    progress: [u32; SOUL_HACK_ACHIEVEMENT_MAX],
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Achievement {
    /// Soul Hack achievement complete: art/skill is upgraded
    Completed,
    /// Soul Hack achievement still in progress. Value is current progress.
    InProgress(u32),
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[cfg_attr(feature = "strum", derive(strum::EnumIter, strum::FromRepr))]
#[repr(u32)]
pub enum Difficulty {
    Easy = 1,
    Normal = 0,
    Hard = 2,
    VeryHard = 3,
}

impl EnemyTombstone {
    pub fn time_record(&self, difficulty: Difficulty) -> &TombstoneTime {
        &self.time_records[difficulty as usize]
    }

    pub fn time_record_mut(&mut self, difficulty: Difficulty) -> &mut TombstoneTime {
        &mut self.time_records[difficulty as usize]
    }

    pub fn get_highest_rematch(&self, difficulty: Difficulty) -> u8 {
        let byte = self.rematches[difficulty as usize / 2];
        let byte = if difficulty as usize & 1 != 0 {
            byte >> 4
        } else {
            byte
        };
        byte & 0xf
    }

    pub fn set_highest_rematch(&mut self, difficulty: Difficulty, highest: u8) {
        assert!(highest < 16, "invalid highest rematch");
        let byte = &mut self.rematches[difficulty as usize / 2];
        let (mask, val) = if difficulty as usize & 1 != 0 {
            (0xf, highest << 4)
        } else {
            (0xf0, highest & 0xf)
        };
        *byte = (*byte & mask) | val;
    }
}

impl SoulHackAchievements {
    pub fn get(&self, index: usize) -> Achievement {
        Achievement::from(self.progress[index])
    }

    pub fn set(&mut self, index: usize, new: Achievement) {
        self.progress[index] = new.into();
    }
}

impl From<u32> for Achievement {
    fn from(value: u32) -> Self {
        match value {
            u32::MAX => Achievement::Completed,
            n => Achievement::InProgress(n),
        }
    }
}

impl From<Achievement> for u32 {
    fn from(value: Achievement) -> Self {
        match value {
            Achievement::Completed => u32::MAX,
            Achievement::InProgress(n) => n,
        }
    }
}
