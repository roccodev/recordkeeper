use bdat::{label_hash, Label};
use game_data::formation::{FormationData, FormationLang, FormationNameProfile, ProfileName};

use crate::{lang::filter_table_from_bdat, BdatRegistry, LangBdatRegistry};

pub fn read_data(bdat: &BdatRegistry) -> FormationData {
    let names = bdat.table(&Label::Hash(0x33F137E8));
    let challenges = bdat.table(label_hash!("BTL_ChTA_List"));

    let mut challenges_by_list_num = challenges
        .rows()
        .map(|row| {
            (
                row.get(label_hash!("Name")).to_integer() as usize,
                row.get(label_hash!("ListNum")).to_integer(),
            )
        })
        .collect::<Vec<_>>();
    challenges_by_list_num.sort_unstable_by_key(|(_, num)| *num);

    let names = names
        .rows()
        .filter_map(|row| {
            let ty = row.get(label_hash!("Type")).to_integer();
            let save_id = row.get(Label::Hash(0x44E4B99C)).to_integer() as u16;

            let name = match ty {
                0 => return None,
                1 => ProfileName::Literal(row.get(label_hash!("Name")).to_integer() as usize),
                2 => {
                    let sort_id = row.get(label_hash!("SortNo")).to_integer() as usize;
                    ProfileName::Challenge(
                        challenges_by_list_num[sort_id.checked_sub(1).unwrap()].0,
                    )
                }
                n => panic!("unknown type {n}"),
            };

            Some(FormationNameProfile::new(name, save_id))
        })
        .collect();

    FormationData {
        names,
        colors: crate::manual::read_formation_colors().into_iter().collect(),
    }
}

pub fn read_lang(bdat: &LangBdatRegistry) -> FormationLang {
    let names = bdat.table(&Label::Hash(0x96DFBB81));

    FormationLang {
        names: filter_table_from_bdat(names),
    }
}
