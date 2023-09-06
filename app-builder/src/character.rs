use bdat::Table;
use game_data::character::{Art, Character, CharacterData, CharacterLang, Skill};

use crate::const_hash;
use crate::lang::filter_table_from_bdat;
use crate::{BdatRegistry, LangBdatRegistry};

pub fn read_data(bdat: &BdatRegistry) -> CharacterData {
    let characters = bdat.table(&const_hash!("CHR_PC"));
    let arts = bdat.table(&const_hash!("BTL_Arts_PC"));
    let skills = bdat.table(&const_hash!("BTL_Skill_PC"));

    let characters =
        read_id_name_pairs(characters).map(|(id, name)| Character { id, name_id: name });
    let arts = read_id_name_pairs(arts).map(|(id, name)| Art { id, name_id: name });
    let skills = read_id_name_pairs(skills).map(|(id, name)| Skill { id, name_id: name });

    CharacterData::new(characters, arts, skills)
}

pub fn read_lang(bdat: &LangBdatRegistry) -> CharacterLang {
    let characters = bdat.table(&const_hash!("msg_player_name"));
    let arts = bdat.table(&const_hash!("msg_btl_arts_name"));
    let skills = bdat.table(&const_hash!("msg_btl_skill_name"));

    CharacterLang {
        characters: filter_table_from_bdat(characters),
        arts: filter_table_from_bdat(arts),
        skills: filter_table_from_bdat(skills),
    }
}

fn read_id_name_pairs<'a>(table: &'a Table) -> impl Iterator<Item = (usize, usize)> + 'a {
    table.rows().map(|r| table.row(r.id())).map(|row| {
        let name = row[const_hash!("Name")].as_single().unwrap().to_integer() as usize;
        (row.id(), name)
    })
}
