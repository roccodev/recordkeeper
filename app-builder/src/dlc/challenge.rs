use bdat::{label_hash, Label};
use game_data::dlc::challenge::{ChallengeData, ChallengeGame, ChallengeLang};

use crate::{lang::filter_table_from_bdat, BdatRegistry, LangBdatRegistry, ModernRow};

pub fn read_game(lang: &BdatRegistry) -> ChallengeGame {
    let challenges = lang.table(label_hash!("BTL_ChTA_List"));
    let gauntlets = lang.table(label_hash!("BTL_ChSU_List"));

    let challenges = challenges.rows().map(read_challenge).collect();
    let gauntlets = gauntlets.rows().map(read_challenge).collect();

    ChallengeGame {
        challenges,
        gauntlets,
    }
}

pub fn read_lang(lang: &LangBdatRegistry) -> ChallengeLang {
    let challenges = lang.table(&Label::Hash(0x192F6292));

    ChallengeLang {
        challenges: filter_table_from_bdat(challenges),
    }
}

fn read_challenge(row: ModernRow) -> ChallengeData {
    let name_id = row.get(label_hash!("Name")).to_integer() as usize;
    ChallengeData {
        id: row.id(),
        name_id,
    }
}
