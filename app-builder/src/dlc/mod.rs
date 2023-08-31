use game_data::dlc::{DlcData, DlcLang};

use crate::{BdatRegistry, LangBdatRegistry};

mod masha;

pub fn read_dlc_game(bdat: &BdatRegistry) -> DlcData {
    DlcData {
        masha: masha::read_game(bdat),
    }
}

pub fn read_dlc_lang(bdat: &LangBdatRegistry) -> DlcLang {
    DlcLang {
        masha: masha::read_lang(bdat),
    }
}
