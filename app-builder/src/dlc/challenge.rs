use bdat::{label_hash, Label};
use game_data::dlc::challenge::{ChallengeData, ChallengeGame, ChallengeLang, Emblem};

use crate::{
    lang::{filter_table_from_bdat, text_table_from_bdat},
    BdatRegistry, LangBdatRegistry, ModernRow,
};

pub fn read_game(lang: &BdatRegistry) -> ChallengeGame {
    let challenges = lang.table(label_hash!("BTL_ChTA_List"));
    let gauntlets = lang.table(label_hash!("BTL_ChSU_List"));
    let emblems = lang.table(label_hash!("BTL_ChSU_Emblem"));

    let challenges = challenges.rows().map(read_challenge).collect();
    let gauntlets = gauntlets.rows().map(read_challenge).collect();

    let emblems = emblems
        .rows()
        .filter(|row| {
            let next = row.get(Label::Hash(0x007829EF)).to_integer();
            next == 0
        })
        .fold((0, Vec::<Emblem>::new()), |(id, mut vec), row| {
            let name_id = row.get(label_hash!("Name")).to_integer() as usize;
            let levels = row.id() - id;
            vec.push(Emblem {
                id: row.id() - levels + 1,
                name_id,
                levels,
            });
            (row.id(), vec)
        })
        .1
        .into_boxed_slice();

    ChallengeGame {
        challenges,
        gauntlets,
        emblems,
    }
}

pub fn read_lang(lang: &LangBdatRegistry) -> ChallengeLang {
    let challenges = lang.table(&Label::Hash(0x192F6292));
    let emblems = lang.table(label_hash!("msg_btl_ChSU_emblem_name"));

    ChallengeLang {
        challenges: filter_table_from_bdat(challenges),
        emblems: text_table_from_bdat(emblems),
    }
}

fn read_challenge(row: ModernRow) -> ChallengeData {
    let name_id = row.get(label_hash!("Name")).to_integer() as usize;
    ChallengeData {
        id: row.id(),
        name_id,
    }
}
