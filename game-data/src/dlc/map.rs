use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use strum::{EnumIter, FromRepr};

use crate::{
    lang::{FilterEntry, Filterable, Id, Nameable, TextEntry, TextTable},
    manual::{Flag, FlagRange},
    IdInt, LanguageData,
};

#[derive(Deserialize, Serialize)]
pub struct Dlc4Map {
    map_achievements: [Box<[MapAchievement]>; 5],
    pub regions: [Dlc4Region; 5],
}

#[derive(Deserialize, Serialize)]
pub struct Dlc4MapLang {
    pub map: TextTable,
    pub achievement_type_map: HashMap<u32, u32>,
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct Dlc4Region {
    pub id: IdInt,
    pub name: IdInt,
}

#[derive(Deserialize, Serialize)]
pub struct Architecture {
    ty: ArchitectureType,
    event_id: Box<str>,
    end_flag: Flag,
}

#[derive(Deserialize, Serialize, Debug, FromRepr)]
#[repr(u8)]
pub enum ArchitectureType {
    Elevator = 0,
    Ladder = 1,
    Slide = 2,
    Lift = 3,
    Tower = 4,
    ComSpot = 5,
}

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

#[derive(Deserialize, Serialize, Debug)]
pub enum AchievementName {
    Enemy {
        name_id: u32,
    },
    Npc {
        name_id: u32,
    },
    Location {
        name_id: u32,
    },
    ComSpot {
        name_id: u32,
    },
    Architecture {
        ty: ArchitectureType,
        x: f32,
        y: f32,
        z: f32,
    },
    Unknown {
        x: f32,
        y: f32,
        z: f32,
    },
}

#[derive(
    Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, FromRepr, EnumIter,
)]
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
    pub fn new(map_achievements: [Box<[MapAchievement]>; 5], regions: [Dlc4Region; 5]) -> Self {
        Self {
            map_achievements,
            regions,
        }
    }

    pub fn achievements(&self, region: usize) -> &[MapAchievement] {
        &self.map_achievements[region]
    }

    pub fn all_achievements(&self) -> impl Iterator<Item = &MapAchievement> {
        self.map_achievements
            .iter()
            .flat_map(|region| region.iter())
    }

    pub fn regions(&self) -> &[Dlc4Region] {
        &self.regions
    }
}

impl ArchitectureType {
    pub fn lang_id(&self) -> &'static str {
        match self {
            ArchitectureType::Elevator => "elevator",
            ArchitectureType::Ladder => "ladder",
            ArchitectureType::Slide => "slide",
            ArchitectureType::Lift => "lift",
            ArchitectureType::Tower => "tower",
            ArchitectureType::ComSpot => "com",
        }
    }
}

impl Nameable for MapAchievement {
    fn get_name<'l>(&self, language: &'l LanguageData) -> Option<&'l TextEntry> {
        let name_id = language.dlc.map.achievement_type_map.get(&self.ty)?;
        language.dlc.map.map.get(*name_id)
    }
}

impl Filterable for Dlc4Region {
    fn get_filter<'l>(&self, language: &'l LanguageData) -> Option<&'l FilterEntry> {
        language.field.locations.get(self.name)
    }
}

impl Id for Dlc4Region {
    fn id(&self) -> IdInt {
        self.id
    }
}
