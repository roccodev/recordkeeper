use recordkeeper_macros::SaveBin;

use super::CHALLENGE_BATTLE_DIFFICULTY_MAX;

pub const EMBLEM_MAX: usize = 300; // best guess

const GAUNTLET_STATE_CHARACTER_MAX: usize = 7;
const GAUNTLET_STATE_EMBLEM_MAX: usize = 60;

#[derive(SaveBin, Debug)]
#[size(72)]
pub struct Gauntlet {
    ranks: [u32; CHALLENGE_BATTLE_DIFFICULTY_MAX],
    stage_reached: [u32; CHALLENGE_BATTLE_DIFFICULTY_MAX],
    high_score: [u32; CHALLENGE_BATTLE_DIFFICULTY_MAX],
    time: [f32; CHALLENGE_BATTLE_DIFFICULTY_MAX],
    play_count: [u32; CHALLENGE_BATTLE_DIFFICULTY_MAX],
    in_progress: bool, // unsure
    _unk1: bool,
    _unk2: bool,
    _unk3: bool, // could be whether rewards for ranks A/B have been received
    _unk4: bool,
    #[loc(0x44)]
    clear_count: u32,
}

/// Archsage's Gauntlet save state
#[derive(SaveBin, Debug)]
#[size(584)]
pub struct GauntletState {
    timestamp: DateTime,
    /// Actually randomly generated
    random: u64,
    active: bool,

    #[loc(0x18)]
    gauntlet_id: u32,
    lead_character_id: u32, // unsure
    difficulty: u32,

    /// ID for `RSC_WeatherSet`
    #[loc(0x24)]
    weather: u32,

    /// Character IDs currently in the party
    #[loc(0x38)]
    party_characters: [u32; GAUNTLET_STATE_CHARACTER_MAX],
    _unk_array_1: [u32; GAUNTLET_STATE_CHARACTER_MAX],

    /// Emblems currently active
    emblems: [u32; GAUNTLET_STATE_EMBLEM_MAX],

    _unk_array_2: [u32; 11],
    _unk_array_3: [u8; 32],

    /// Current map ID. (Actual map ID = value + 75, ID for `SYS_MapList`)
    map_id: u32,
    /// Set if a map jump needs to happen. ID for `BTL_ChSU_MapBattleLock`.
    next_map_pos_id: u32,
    last_stage: u32,
    current_score: u32,
    /// 0-3
    shuffle_tickets: u32,
    /// 100: one charge, 200: two charges, 300: three charges
    launch_charge: f32,
    /// 0-100
    nopwatch_gauge: f32,
    /// 0-6
    no_ko_streak: u32,

    /// Number of noponstones gained so far. Unclear why it is here, as it can be calculated
    /// from the stage number.
    nopon_stone_reward: u32,

    /// Number of purchased heroes
    hero_buy_count: u32,
    /// Number of nopwatch refill purchases
    nopwatch_buy_count: u32,

    #[loc(0x1e4)]
    /// ID for `BTL_ChSU_SettingGate`. It's likely that setting both of these will
    /// open the whimsy screen when the save is loaded.
    // needs testing
    whimsy_negative: u32,
    /// ID for `BTL_ChSU_SettingGate`
    whimsy_positive: u32,
    /// 0-900
    chain_gauge: f32,

    // End screen stats
    /// Score spent in the shop
    #[loc(0x23c)]
    score_spent: u32,
    emblems_bought: u32,
}

#[derive(SaveBin, Debug)]
#[size(8)]
pub struct EmblemItem {
    unlocked: bool,
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
