use recordkeeper_macros::SaveBin;
use thiserror::Error;

use crate::enemy::Difficulty;

use super::{EmblemItem, Gauntlet, GauntletState, EMBLEM_MAX};

const CHALLENGE_BATTLE_NUM_CHALLENGES: usize = 18;
pub const CHALLENGE_BATTLE_NUM_GAUNTLET: usize = 4;
pub const CHALLENGE_BATTLE_NUM_GAUNTLET_STATES: usize = 1; // likely a 1-item array in the game
pub const CHALLENGE_BATTLE_DIFFICULTY_MAX: usize = 3;

#[derive(SaveBin, Debug)]
#[size(28900)]
pub struct ChallengeBattle {
    #[assert(6)]
    _unk: u32,
    challenges_1_18: Box<[Challenge; CHALLENGE_BATTLE_NUM_CHALLENGES]>,

    gauntlet: Box<[Gauntlet; CHALLENGE_BATTLE_NUM_GAUNTLET]>,

    #[loc(0x368)]
    gauntlet_states: Box<[GauntletState; CHALLENGE_BATTLE_NUM_GAUNTLET_STATES]>,

    #[loc(0x6320)]
    challenges_19: [Challenge; 1], // easier to work with

    /// Actual size: number of rows in `BTL_ChSU_Emblem`
    #[loc(0x658c)]
    emblem_shop: Box<[EmblemItem; EMBLEM_MAX]>,

    #[loc(0x70cc)]
    pub nopon_stone_red: u32,
    pub nopon_stone_blue: u32,
    // Two flags here for whether the current challenge has a bonus, not relevant for saves
}

#[derive(SaveBin, Debug)]
#[size(32)]
pub struct Challenge {
    ranks: [u32; CHALLENGE_BATTLE_DIFFICULTY_MAX],
    best_time: [f32; CHALLENGE_BATTLE_DIFFICULTY_MAX],
    pub clear_count: u32,
    /// Purpose unclear
    pub cleared: bool,
    /// Shows the "new" notification dot
    pub new: bool,
    /// Whether the challenge currently has a 3x reward bonus
    pub has_bonus: bool,
    /// Whether the Rank A reward has been claimed
    pub claimed_reward: bool,
}

/// Difficulties supported by challenge battle modes.
///
/// Notably, this is the same as [`Difficulty`], but without Very Hard.
///
/// [`Difficulty`]: crate::save::enemy::Difficulty
#[cfg_attr(feature = "strum", derive(strum::EnumIter, strum::FromRepr))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChallengeDifficulty {
    Easy = 1,
    Normal = 0,
    Hard = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "strum", derive(strum::EnumIter, strum::FromRepr))]
pub enum ChallengeRank {
    None = 0,
    S = 1,
    A = 2,
    B = 3,
    C = 4,
}

#[derive(Error, Debug)]
#[error("unknown rank {0}")]
pub struct RankFromIntError(u32);

#[derive(Error, Debug)]
#[error("unsupported difficulty: {0:?}")]
pub struct FromDifficultyError(Difficulty);

impl ChallengeBattle {
    const CHALLENGE_2_START: usize = CHALLENGE_BATTLE_NUM_CHALLENGES + 1;

    /// Returns a view of a challenge record.
    ///
    /// The ID starts at 1.
    ///
    /// ## Panics
    /// Panics if the ID is out of bounds.
    pub fn challenge(&self, id: usize) -> &Challenge {
        match id {
            1..=CHALLENGE_BATTLE_NUM_CHALLENGES => &self.challenges_1_18[id - 1],
            Self::CHALLENGE_2_START => &self.challenges_19[id - Self::CHALLENGE_2_START],
            _ => panic!("id out of bounds"),
        }
    }

    /// Returns a mutable view of a challenge record.
    ///
    /// The ID starts at 1.
    ///
    /// ## Panics
    /// Panics if the ID is out of bounds.
    pub fn challenge_mut(&mut self, id: usize) -> &mut Challenge {
        match id {
            1..=CHALLENGE_BATTLE_NUM_CHALLENGES => &mut self.challenges_1_18[id - 1],
            Self::CHALLENGE_2_START => &mut self.challenges_19[id - Self::CHALLENGE_2_START],
            _ => panic!("id out of bounds"),
        }
    }

    /// Returns a view of a gauntlet record.
    ///
    /// The ID starts at 1.
    ///
    /// ## Panics
    /// Panics if the ID is out of bounds.
    pub fn gauntlet(&self, id: usize) -> &Gauntlet {
        &self.gauntlet[id.checked_sub(1).expect("id > 0")]
    }

    /// Returns a mutable view of a gauntlet record.
    ///
    /// The ID starts at 1.
    ///
    /// ## Panics
    /// Panics if the ID is out of bounds.
    pub fn gauntlet_mut(&mut self, id: usize) -> &mut Gauntlet {
        &mut self.gauntlet[id.checked_sub(1).expect("id > 0")]
    }

    /// Returns a view of an emblem shop item.
    ///
    /// The ID starts at 1.
    ///
    /// ## Panics
    /// Panics if the ID is out of bounds.
    pub fn emblem(&self, id: usize) -> &EmblemItem {
        &self.emblem_shop[id.checked_sub(1).expect("id > 0")]
    }

    /// Returns a mutable view of an emblem shop item.
    ///
    /// The ID starts at 1.
    ///
    /// ## Panics
    /// Panics if the ID is out of bounds.
    pub fn emblem_mut(&mut self, id: usize) -> &mut EmblemItem {
        &mut self.emblem_shop[id.checked_sub(1).expect("id > 0")]
    }

    /// Returns an iterator over the challenge records.
    pub fn challenges(&self) -> impl Iterator<Item = &Challenge> {
        self.challenges_1_18.iter().chain(self.challenges_19.iter())
    }

    /// Returns an iterator over the gauntlet records.
    pub fn gauntlets(&self) -> impl Iterator<Item = &Gauntlet> {
        self.gauntlet.iter()
    }

    pub fn gauntlet_save(&self) -> &GauntletState {
        &self.gauntlet_states[0]
    }

    pub fn gauntlet_save_mut(&mut self) -> &mut GauntletState {
        &mut self.gauntlet_states[0]
    }
}

impl Challenge {
    pub fn get_rank(&self, difficulty: ChallengeDifficulty) -> ChallengeRank {
        self.ranks[difficulty as usize].try_into().unwrap()
    }

    pub fn set_rank(&mut self, difficulty: ChallengeDifficulty, rank: ChallengeRank) {
        self.ranks[difficulty as usize] = rank as u32;
    }

    pub fn get_best_time(&self, difficulty: ChallengeDifficulty) -> f32 {
        self.best_time[difficulty as usize]
    }

    pub fn set_best_time(&mut self, difficulty: ChallengeDifficulty, time: f32) {
        self.best_time[difficulty as usize] = time;
        if time != 0.0 {
            self.cleared = true;
        }
    }
}

impl TryFrom<u32> for ChallengeRank {
    type Error = RankFromIntError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::None,
            1 => Self::S,
            2 => Self::A,
            3 => Self::B,
            4 => Self::C,
            n => return Err(RankFromIntError(n)),
        })
    }
}

impl TryFrom<Difficulty> for ChallengeDifficulty {
    type Error = FromDifficultyError;

    fn try_from(value: Difficulty) -> Result<Self, Self::Error> {
        Ok(match value {
            Difficulty::Easy => Self::Easy,
            Difficulty::Normal => Self::Normal,
            Difficulty::Hard => Self::Hard,
            d => return Err(FromDifficultyError(d)),
        })
    }
}

impl From<ChallengeDifficulty> for Difficulty {
    fn from(value: ChallengeDifficulty) -> Self {
        match value {
            ChallengeDifficulty::Easy => Self::Easy,
            ChallengeDifficulty::Normal => Self::Normal,
            ChallengeDifficulty::Hard => Self::Hard,
        }
    }
}
