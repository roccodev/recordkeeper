use serde::{Deserialize, Serialize};

use crate::{
    lang::FilterTable,
    manual::{Flag, FlagRange},
};

#[derive(Deserialize, Serialize)]
pub struct Dlc4Map {
    map_achievements: [Box<[MapAchievement]>; 5],
}

#[derive(Deserialize, Serialize)]
pub struct Dlc4MapLang {
    pub map: FilterTable,
}

#[derive(Deserialize, Serialize)]
pub struct Collepedia {
    location_id: u32,
    category: u32,
    item: u32,
    flag: Flag,
    sort_id: u32,
}

#[derive(Deserialize, Serialize)]
pub struct Enemypedia {
    location_id: u32,
    enemies: Box<[u32]>,
    flag: Flag,
    sort_id: u32,
}

#[derive(Deserialize, Serialize)]
pub struct Architecture {
    ty: ArchitectureType,
    event_id: Box<str>,
    end_flag: Flag,
}

#[derive(Deserialize, Serialize)]
pub enum ArchitectureType {
    Repair = 0,
    Make = 1,
}

/*
MAP achievement text hashes (type 1 => ..., 2 => ...)

    local_c0[1][0] = 0x1729fe77;
    local_c0[2][0] = 0xe2bb89d6;
    local_c0[3][0] = 0xb1c9ca4f;
    local_c0[4][0] = 0x947ee7df;
    local_c0[5][0] = 0x34d3c504;
    local_c0[6][0] = 0x648ab23;
    local_c0[7][0] = 0xe8428146;
    local_c0[8][0] = 0x515f1496;
    local_c0[9][0] = 0x26f7df25;
    local_c0[10][0] = 0x9591fd9f;
    local_c0[11][0] = 0x3f44fa87;
    local_c0[12][0] = 0xdc087a84;
    local_c0[13][0] = 0x19bff09f;
    local_c0[14][0] = 0x921c87db;

    0C37D405 and 85601D1A are mast IDs (FLD_Architecture),
    they reveal their respective 20 achievement searches on the map

*/
/// A row from `FLD_MapAchievementSearch`
#[derive(Deserialize, Serialize)]
pub struct MapAchievement {
    pub region: u32,
    pub ty: u32,
    pub searches: Box<[AchievementSearch]>,
}

#[derive(Deserialize, Serialize)]
pub struct AchievementSearch {
    pub flag: Flag,
    pub name: AchievementName,
}

#[derive(Deserialize, Serialize)]
pub enum AchievementName {
    Enemy(u32),
    Npc(u32),
    Unknown(u32),
}

#[derive(Deserialize, Serialize)]
pub enum MapAchievementProgress {
    Hidden = 0,
    Visible = 1,
    Completed = 2,
}

/// # Flag calculation logic
///
/// * Architecture (type 7): base flag + gimmick's sequential ID
///    - If `FLD_Architecture.67B1D80A` is 2, 3, or 5, `FLD_Architecture.8B6F5AD3` is used as the gimmick instead.
///    - If the new gimmick is an ether slide, base flag changes depending on whether it is a slide or a lift
/// * NPC rescues (type 8): `0DEC588C` in table `46B9A047`
/// * Affinity Events (type 9): `ma40a_GMK_KizunaEvent.Flag`
/// * Unique monsters (type 11): base flag + (`FLD_EnemyData.NamedFlag` - 5419)
/// * Everything else: base flag + gimmick's sequential ID
#[derive(Deserialize, Serialize)]
pub struct MapAchievementFlags {
    containers: FlagRange,
    relics: FlagRange,
    ether_channels: FlagRange,
    // base flag + (NamedFlag - 5419)
    uniques: FlagRange,

    // suppression

    // 46B9A047
    npc_rescue: FlagRange,
    fog_enemies: FlagRange,
    // FLD_Architecture
    arch_ladder: FlagRange,
    arch_elevator: FlagRange,
    locations: FlagRange,
    landmarks: FlagRange,
    secret_areas: FlagRange,
    // ma40a_GMK_KizunaEvent
    kizuna_events: FlagRange,
}

impl Dlc4Map {
    pub fn new(map_achievements: [Box<[MapAchievement]>; 5]) -> Self {
        Self { map_achievements }
    }
}
