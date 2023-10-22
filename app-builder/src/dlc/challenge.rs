use bdat::{label_hash, Label};
use game_data::dlc::challenge::{ChallengeData, ChallengeGame, ChallengeLang};

use crate::{lang::filter_table_from_bdat, BdatRegistry, LangBdatRegistry};

pub fn read_game(lang: &BdatRegistry) -> ChallengeGame {
    let challenges = lang.table(label_hash!("BTL_ChTA_List"));

    let challenges = challenges
        .rows()
        .map(|row| {
            let name_id = row.get(label_hash!("Name")).to_integer() as usize;
            ChallengeData {
                id: row.id(),
                name_id,
            }
        })
        .collect();

    ChallengeGame { challenges }
}

pub fn read_lang(lang: &LangBdatRegistry) -> ChallengeLang {
    let challenges = lang.table(&Label::Hash(0x192F6292));

    ChallengeLang {
        challenges: filter_table_from_bdat(challenges),
    }
}
