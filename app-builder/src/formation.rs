use bdat::Label;
use game_data::formation::{FormationData, FormationLang, FormationNameProfile, ProfileName};

use crate::{const_hash, lang::filter_table_from_bdat, BdatRegistry, LangBdatRegistry};

pub fn read_data(bdat: &BdatRegistry) -> FormationData {
    let names = bdat.table(&Label::Hash(0x33F137E8));
    let challenges = bdat.table(&const_hash!("BTL_ChTA_List"));

    let mut challenges_by_list_num = challenges
        .rows()
        .map(|r| challenges.row(r.id()))
        .map(|row| {
            (
                row[const_hash!("Name")].as_single().unwrap().to_integer() as usize,
                row[const_hash!("ListNum")]
                    .as_single()
                    .unwrap()
                    .to_integer(),
            )
        })
        .collect::<Vec<_>>();
    challenges_by_list_num.sort_unstable_by_key(|(_, num)| *num);

    let names = names
        .rows()
        .map(|r| names.row(r.id()))
        .filter_map(|row| {
            let ty = row[const_hash!("Type")].as_single().unwrap().to_integer();
            let save_id = row[Label::Hash(0x44E4B99C)]
                .as_single()
                .unwrap()
                .to_integer() as u16;

            let name = match ty {
                0 => return None,
                1 => ProfileName::Literal(
                    row[const_hash!("Name")].as_single().unwrap().to_integer() as usize,
                ),
                2 => {
                    let sort_id =
                        row[const_hash!("SortNo")].as_single().unwrap().to_integer() as usize;
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
