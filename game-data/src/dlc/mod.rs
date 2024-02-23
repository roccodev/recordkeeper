use serde::{Deserialize, Serialize};

use self::{
    challenge::{ChallengeGame, ChallengeLang},
    map::{Dlc4Map, Dlc4MapLang},
    masha::{GameCraftItems, LangCraftItems},
};

pub mod challenge;
pub mod map;
pub mod masha;
pub mod pow_augment;

#[derive(Serialize, Deserialize)]
pub struct DlcData {
    pub masha: GameCraftItems,
    pub challenge: ChallengeGame,
    pub map: Dlc4Map,
}

#[derive(Serialize, Deserialize)]
pub struct DlcLang {
    pub masha: LangCraftItems,
    pub challenge: ChallengeLang,
    pub map: Dlc4MapLang,
}
