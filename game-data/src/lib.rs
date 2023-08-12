use std::io::{Read, Write};

use item::ItemRegistry;
use serde::{Deserialize, Serialize};
use std::error::Error;

pub mod item;
pub mod quest;

#[derive(Serialize, Deserialize)]
pub struct GameData {
    pub items: ItemRegistry,
}

#[derive(Serialize, Deserialize)]
pub struct LanguageData {}

pub fn save_game_data(data: &GameData, mut writer: impl Write) -> Result<(), Box<dyn Error>> {
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
