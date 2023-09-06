use std::io::{Read, Write};

use character::{CharacterData, CharacterLang};
use dlc::{DlcData, DlcLang};
use enhance::{EnhanceLang, EnhanceRegistry};
use item::{ItemLanguageRegistry, ItemRegistry};
use manual::ManualData;
use quest::{QuestLang, QuestRegistry};
use scenario::ScenarioRanges;
use serde::{Deserialize, Serialize};
use std::error::Error;

pub mod character;
pub mod dlc;
pub mod enhance;
pub mod item;
pub mod lang;
pub mod manual;
pub mod quest;
pub mod scenario;

#[derive(Serialize, Deserialize)]
pub struct GameData {
    pub items: ItemRegistry,
    pub enhance: EnhanceRegistry,
    pub dlc: DlcData,
    pub events: ScenarioRanges,
    pub quests: QuestRegistry,
    pub characters: CharacterData,

    /// Manually inputted data, that can't be read
    /// from game files.
    pub manual: ManualData,
}

#[derive(Serialize, Deserialize)]
pub struct LanguageData {
    pub items: ItemLanguageRegistry,
    pub enhance: EnhanceLang,
    pub dlc: DlcLang,
    pub quests: QuestLang,
    pub characters: CharacterLang,
}

pub fn save_game_data(data: &GameData, mut writer: impl Write) -> Result<(), Box<dyn Error>> {
    let bytes = bitcode::serialize(&data)?;
    writer.write_all(&bytes)?;
    Ok(())
}

pub fn save_lang_data(data: &LanguageData, mut writer: impl Write) -> Result<(), Box<dyn Error>> {
    let bytes = bitcode::serialize(&data)?;
    writer.write_all(&bytes)?;
    Ok(())
}

pub fn load_game_data(mut reader: impl Read) -> Result<GameData, Box<dyn Error>> {
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes)?;
    let res = bitcode::deserialize(&bytes)?;
    Ok(res)
}

pub fn load_lang_data(mut reader: impl Read) -> Result<LanguageData, Box<dyn Error>> {
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes)?;
    let res = bitcode::deserialize(&bytes)?;
    Ok(res)
}
