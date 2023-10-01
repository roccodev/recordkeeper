use serde::{Deserialize, Serialize};

use self::{
    challenge::ChallengeLang,
    masha::{GameCraftItems, LangCraftItems},
};

pub mod challenge;
pub mod masha;

#[derive(Serialize, Deserialize)]
pub struct DlcData {
    pub masha: GameCraftItems,
}

#[derive(Serialize, Deserialize)]
pub struct DlcLang {
    pub masha: LangCraftItems,
    pub challenge: ChallengeLang,
}
