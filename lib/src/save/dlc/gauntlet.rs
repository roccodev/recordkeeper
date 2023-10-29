use recordkeeper_macros::SaveBin;

use crate::{enemy::Difficulty, flags::BitFlags, MapTime};

use super::{ChallengeDifficulty, ChallengeRank, CHALLENGE_BATTLE_DIFFICULTY_MAX};

pub const EMBLEM_MAX: usize = 300; // best guess

const GAUNTLET_STATE_CHARACTER_MAX: usize = 7;
const GAUNTLET_STATE_EMBLEM_MAX: usize = 60;
const GAUNTLET_STATE_NOTIFICATION_MAX: usize = 11;
const GAUNTLET_STATE_JUMP_MAX: usize = 32;
const GAUNTLET_STATE_SYS_OPEN_MAX: usize = 16;
const GAUNTLET_STATE_WHIMSY_MAX: usize = 2;

#[derive(SaveBin, Debug)]
#[size(72)]
pub struct Gauntlet {
    ranks: [u32; CHALLENGE_BATTLE_DIFFICULTY_MAX],
    stage_reached: [u32; CHALLENGE_BATTLE_DIFFICULTY_MAX],
    high_score: [u32; CHALLENGE_BATTLE_DIFFICULTY_MAX],
    time: [f32; CHALLENGE_BATTLE_DIFFICULTY_MAX],
    play_count: [u32; CHALLENGE_BATTLE_DIFFICULTY_MAX],
    /// Purpose unclear
    pub cleared: bool,
    /// Shows the "new" notification dot
    pub new: bool,
    /// Whether the gauntlet currently has a 3x reward bonus
    pub has_bonus: bool,
    /// Whether the Rank B reward has been claimed
    pub reward_b: bool,
    /// Whether the Rank A reward has been claimed
    pub reward_a: bool,
    #[loc(0x44)]
    pub clear_count: u32,
}

/// Archsage's Gauntlet save state
#[derive(SaveBin, Debug)]
#[size(584)]
pub struct GauntletState {
    pub timestamp: DateTime,
    /// Actually randomly generated
    pub random: u64,
    pub active: bool,

    #[loc(0x18)]
    pub gauntlet_id: u32,
    lead_character_id: u32,
    challenge_difficulty: u32,
    pub map_time: MapTime<u32>,
    /// ID for `RSC_WeatherSet`
    pub weather: u32,
    _unk_f: f32,
    /// Original game difficulty, used to reset after the run ends.
    pub game_difficulty: u32,

    /// Character IDs currently in the party.
    ///
    /// The first entry is ignored by the game, and is instead set to
    /// `lead_character_id`.
    pub party_characters: [u32; GAUNTLET_STATE_CHARACTER_MAX],
    /// Join queue for effects (?)
    pub hero_join_queue: [u32; GAUNTLET_STATE_CHARACTER_MAX],

    /// Emblems currently active
    pub emblems: [u32; GAUNTLET_STATE_EMBLEM_MAX],

    /// IDs for `1178340A`. Notifications shown at the end of the round.
    pub notifications: [u32; GAUNTLET_STATE_NOTIFICATION_MAX],
    /// Map jumps that have already been visited in the current map.
    ///
    /// Index relative to the first row with the proper `Map` value in `BTL_ChSU_MapBattleLock`.
    pub visited_jumps: [bool; GAUNTLET_STATE_JUMP_MAX],

    /// Current map ID. (Actual map ID = value + 75, ID for `SYS_MapList`)
    pub map_id: u32,
    /// Set if a map jump needs to happen. ID for `BTL_ChSU_MapBattleLock`.
    pub next_map_pos_id: u32,
    pub last_stage: u32,
    pub current_score: u32,
    /// 0-3
    pub shuffle_tickets: u32,
    /// 100: one charge, 200: two charges, 300: three charges
    pub launch_charge: f32,
    /// 0-100
    pub nopwatch_gauge: f32,
    /// 0-6
    pub no_ko_streak: u32,

    /// Number of noponstones gained so far. Unclear why it is here, as it can be calculated
    /// from the stage number.
    pub nopon_stone_reward: u32,

    /// Number of purchased heroes
    pub hero_buy_count: u32,
    /// Number of nopwatch refill purchases
    pub nopwatch_buy_count: u32,

    #[loc(0x1e0)]
    flags: BitFlags<1, 1>,
    /// ID for `BTL_ChSU_SettingGate`. It's likely that setting both of these will
    /// open the whimsy screen when the save is loaded.
    // needs testing
    pub whimsy: [u32; GAUNTLET_STATE_WHIMSY_MAX],
    /// 0-900
    pub chain_gauge: f32,
    /// Unsure
    level_multiplier: f32,
    /// On load, if this is `true`, `level_multiplier` is applied, then this is set to `false`.
    has_level_multiplier: bool,
    /// Unlocked SysOpens that were locked by the gauntlet. Used to reset at the end of the run.
    unlocked_sys_opens: [u32; GAUNTLET_STATE_SYS_OPEN_MAX],

    // End screen stats
    /// Total score gained (does not decrease)
    #[loc(0x23c)]
    pub score_gained: u32,
    pub emblems_bought: u32,
}

#[derive(SaveBin, Debug)]
#[size(8)]
pub struct EmblemItem {
    pub unlocked: bool,
}

#[derive(SaveBin, Debug)]
#[size(8)]
pub struct DateTime {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

impl Gauntlet {
    pub fn get_rank(&self, difficulty: ChallengeDifficulty) -> ChallengeRank {
        self.ranks[difficulty as usize].try_into().unwrap()
    }

    pub fn set_rank(&mut self, difficulty: ChallengeDifficulty, rank: ChallengeRank) {
        self.ranks[difficulty as usize] = rank as u32;
    }

    pub fn get_best_time(&self, difficulty: ChallengeDifficulty) -> f32 {
        self.time[difficulty as usize]
    }

    pub fn set_best_time(&mut self, difficulty: ChallengeDifficulty, time: f32) {
        self.time[difficulty as usize] = time;
    }

    pub fn get_high_score(&self, difficulty: ChallengeDifficulty) -> u32 {
        self.high_score[difficulty as usize]
    }

    pub fn set_high_score(&mut self, difficulty: ChallengeDifficulty, score: u32) {
        self.high_score[difficulty as usize] = score;
    }

    pub fn get_stage_reached(&self, difficulty: ChallengeDifficulty) -> u32 {
        self.stage_reached[difficulty as usize]
    }

    pub fn set_stage_reached(&mut self, difficulty: ChallengeDifficulty, stage: u32) {
        self.stage_reached[difficulty as usize] = stage;
    }

    pub fn get_play_count(&self, difficulty: ChallengeDifficulty) -> u32 {
        self.play_count[difficulty as usize]
    }

    pub fn set_play_count(&mut self, difficulty: ChallengeDifficulty, count: u32) {
        self.play_count[difficulty as usize] = count;
    }
}

impl GauntletState {
    pub fn get_challenge_difficulty(&self) -> ChallengeDifficulty {
        Difficulty::try_from(self.challenge_difficulty)
            .expect("invalid difficulty")
            .try_into()
            .expect("difficulty not supported")
    }

    pub fn set_challenge_difficulty(&mut self, difficulty: ChallengeDifficulty) {
        self.challenge_difficulty = difficulty as u32;
    }

    pub fn get_lead_character(&self) -> u32 {
        self.lead_character_id
    }

    pub fn set_lead_character(&mut self, lead: u32) {
        self.lead_character_id = lead;
        self.party_characters[0] = lead;
    }
}
