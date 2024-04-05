use bdat::{
    label_hash,
    modern::{ModernRowRef, ModernTable},
    Label,
};
use game_data::dlc::challenge::{
    ChallengeData, ChallengeGame, ChallengeLang, Emblem, GauntletMap, Whimsy,
};

use crate::{
    lang::{filter_table_from_bdat, text_table_from_bdat},
    BdatRegistry, LangBdatRegistry,
};

pub fn read_game(bdat: &BdatRegistry) -> ChallengeGame {
    let challenges = bdat.table(label_hash!("BTL_ChTA_List"));
    let gauntlets = bdat.table(label_hash!("BTL_ChSU_List"));
    let emblems = bdat.table(label_hash!("BTL_ChSU_Emblem"));
    let gauntlet_jumps = bdat.table(label_hash!("BTL_ChSU_MapBattleLock"));
    let maps = bdat.table(label_hash!("SYS_MapList"));
    let map_resources = bdat.table(label_hash!("RSC_MapFile"));
    let whimsy = bdat.table(label_hash!("BTL_ChSU_SettingGate"));

    let challenges = challenges.rows().map(read_challenge).collect();
    let gauntlets = gauntlets.rows().map(read_challenge).collect();

    let emblems = emblems
        .rows()
        .filter(|row| {
            let next = row.get(Label::Hash(0x007829EF)).to_integer();
            next == 0
        })
        .fold((0, Vec::<Emblem>::new()), |(id, mut vec), row| {
            let name_id = row.get(label_hash!("Name")).to_integer();
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

    let mut gauntlet_maps = gauntlet_jumps
        .rows()
        .map(|r| read_gauntlet_map(&maps, &map_resources, r))
        .collect::<Vec<_>>();
    gauntlet_maps.dedup_by_key(|m| m.id);

    let whimsy = whimsy.rows().map(read_whimsy).collect();

    ChallengeGame {
        challenges,
        gauntlets,
        emblems,
        gauntlet_maps: gauntlet_maps.into_boxed_slice(),
        whimsy,
    }
}

pub fn read_lang(lang: &LangBdatRegistry) -> ChallengeLang {
    let challenges = lang.table(&Label::Hash(0x192F6292));
    let emblems = lang.table(label_hash!("msg_btl_ChSU_emblem_name"));
    let whimsy = lang.table(label_hash!("msg_btl_ChSU_gate_caption"));

    ChallengeLang {
        challenges: filter_table_from_bdat(challenges),
        emblems: text_table_from_bdat(emblems),
        whimsy: filter_table_from_bdat(whimsy),
    }
}

fn read_challenge(row: ModernRowRef) -> ChallengeData {
    let name_id = row.get(label_hash!("Name")).to_integer();
    ChallengeData {
        id: row.id(),
        name_id,
    }
}

fn read_gauntlet_map(
    maps: &ModernTable,
    resources: &ModernTable,
    jump: ModernRowRef,
) -> GauntletMap {
    let map = jump.get(label_hash!("Map")).to_integer();
    let map = maps.row_by_hash(map);
    let resource = resources.row(map.get(label_hash!("ResourceId")).to_integer());
    let default_resource = resource.get(label_hash!("DefaultResource")).as_str();
    let original_resource = resources
        .rows()
        .find(|r| default_resource == r.get(label_hash!("DefaultBdatPrefix")).as_str())
        .expect("no matching map resource");
    let original_map = maps
        .rows()
        .find(|r| original_resource.id() == r.get(label_hash!("ResourceId")).to_integer())
        .expect("no map for resource");
    GauntletMap {
        id: map.id(),
        based_on_lang_id: original_map.get(label_hash!("Name")).to_integer(),
    }
}

fn read_whimsy(row: ModernRowRef) -> Whimsy {
    let caption = row.get(label_hash!("Caption")).to_integer();
    Whimsy {
        id: row.id(),
        caption,
    }
}
