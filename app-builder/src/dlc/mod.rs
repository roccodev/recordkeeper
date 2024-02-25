use game_data::dlc::{DlcData, DlcLang};

use crate::{BdatRegistry, LangBdatRegistry};

mod challenge;
mod map;
mod masha;
mod pedia;
pub mod pow_augment;

pub fn read_dlc_game(bdat: &BdatRegistry) -> DlcData {
    DlcData {
        masha: masha::read_game(bdat),
        challenge: challenge::read_game(bdat),
        map: map::read_game(bdat),
        collepedia: pedia::read_collepedia(bdat),
        enemypedia: pedia::read_enemypedia(bdat),
    }
}

pub fn read_dlc_lang(bdat: &LangBdatRegistry) -> DlcLang {
    DlcLang {
        masha: masha::read_lang(bdat),
        challenge: challenge::read_lang(bdat),
        map: map::read_lang(bdat),
    }
}
