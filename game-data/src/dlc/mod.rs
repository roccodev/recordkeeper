use serde::{Deserialize, Serialize};

use self::masha::{GameCraftItems, LangCraftItems};

pub mod masha;

#[derive(Serialize, Deserialize)]
pub struct DlcData {
    pub masha: GameCraftItems,
}

#[derive(Serialize, Deserialize)]
pub struct DlcLang {
    pub masha: LangCraftItems,
}
