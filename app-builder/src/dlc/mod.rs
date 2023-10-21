use game_data::dlc::{DlcData, DlcLang};

use crate::{BdatRegistry, LangBdatRegistry};

mod challenge;
mod masha;
pub mod pow_augment;

pub fn read_dlc_game(bdat: &BdatRegistry) -> DlcData {
    DlcData {
        masha: masha::read_game(bdat),
    }
}

pub fn read_dlc_lang(bdat: &LangBdatRegistry) -> DlcLang {
    DlcLang {
        masha: masha::read_lang(bdat),
        challenge: challenge::read_lang(bdat),
    }
}
