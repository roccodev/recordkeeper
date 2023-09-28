use std::num::NonZeroUsize;

use bdat::{Label, RowRef, Table};
use game_data::character::{
    Art, Attachment, Character, CharacterData, CharacterLang, Class, Costume, Skill, SoulHack,
};

use crate::const_hash;
use crate::lang::filter_table_from_bdat;
use crate::{BdatRegistry, LangBdatRegistry};

const UI_NAME_HASHES: [Label; 6] = [
    const_hash!("UIName1"),
    const_hash!("UIName2"),
    const_hash!("UIName3"),
    const_hash!("UIName4"),
    const_hash!("UIName5"),
    const_hash!("UIName6"),
];

pub fn read_data(bdat: &BdatRegistry) -> CharacterData {
    let characters = bdat.table(&const_hash!("CHR_PC"));
    let arts = bdat.table(&const_hash!("BTL_Arts_PC"));
    let skills = bdat.table(&const_hash!("BTL_Skill_PC"));
    let classes = bdat.table(&const_hash!("BTL_Talent"));
    let attachments = bdat.table(&const_hash!("MNU_Attachment"));
    let costumes_table = bdat.table(&const_hash!("RSC_PcCostumeOpen"));

    let characters =
        read_id_name_pairs(characters).map(|(id, name)| Character { id, name_id: name });
    let arts = read_id_name_pairs(arts).map(|(id, name)| Art {
        id,
        name_id: name,
        soul_hack: read_soul_hack(
            &arts.row(id),
            Label::Hash(0xA2275574),
            const_hash!("EnArtsAchieve"),
        ),
    });
    let skills = read_id_name_pairs(skills).map(|(id, name)| Skill {
        id,
        name_id: name,
        soul_hack: read_soul_hack(
            &skills.row(id),
            Label::Hash(0xA6E42F10),
            const_hash!("EnSkillAchieve"),
        ),
    });
    let classes = read_id_name_pairs(classes).map(|(id, name)| Class { id, name_id: name });
    let attachments =
        read_id_name_pairs(attachments).map(|(id, name)| Attachment { id, name_id: name });

    const EMPTY: Vec<Costume> = Vec::new();
    let mut costumes = [EMPTY; 6];
    for i in 0..costumes.len() {
        read_costume(&costumes_table, i, &mut costumes[i]);
    }

    CharacterData::new(characters, arts, skills, classes, attachments, costumes)
}

pub fn read_lang(bdat: &LangBdatRegistry) -> CharacterLang {
    let characters = bdat.table(&const_hash!("msg_player_name"));
    let arts = bdat.table(&const_hash!("msg_btl_arts_name"));
    let skills = bdat.table(&const_hash!("msg_btl_skill_name"));
    let classes = bdat.table(&const_hash!("msg_btl_talent_name"));
    let misc = bdat.table(&const_hash!("msg_mnu_char_ms"));

    CharacterLang {
        characters: filter_table_from_bdat(characters),
        arts: filter_table_from_bdat(arts),
        skills: filter_table_from_bdat(skills),
        classes: filter_table_from_bdat(classes),
        misc: filter_table_from_bdat(misc),
    }
}

fn read_id_name_pairs<'a>(table: &'a Table) -> impl Iterator<Item = (usize, usize)> + 'a {
    table.rows().map(|r| table.row(r.id())).map(|row| {
        let name = row[const_hash!("Name")].as_single().unwrap().to_integer() as usize;
        (row.id(), name)
    })
}

fn read_costume(table: &Table, char_id: usize, out: &mut Vec<Costume>) {
    for row in table.rows().map(|r| table.row(r.id())) {
        let id = row.id();
        let name_id = row[&UI_NAME_HASHES[char_id]]
            .as_single()
            .unwrap()
            .to_integer() as usize;
        out.push(Costume { id, name_id })
    }
}

fn read_soul_hack(row: &RowRef, status_hash: Label, achievement_hash: Label) -> Option<SoulHack> {
    let status = row[status_hash].as_single().unwrap().to_integer() as usize;
    let status = NonZeroUsize::new(status)?;

    let achievement = row[achievement_hash].as_single().unwrap().to_integer() as usize;
    let achievement = NonZeroUsize::new(achievement)?;

    Some(SoulHack {
        status_flag: status,
        achievement_flag: achievement,
    })
}
