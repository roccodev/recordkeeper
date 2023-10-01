use bdat::Label;
use game_data::dlc::challenge::ChallengeLang;

use crate::{lang::filter_table_from_bdat, LangBdatRegistry};

pub fn read_lang(lang: &LangBdatRegistry) -> ChallengeLang {
    let challenges = lang.table(&Label::Hash(0x192F6292));

    ChallengeLang {
        challenges: filter_table_from_bdat(challenges),
    }
}
