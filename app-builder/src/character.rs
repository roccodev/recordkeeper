use std::num::NonZeroUsize;

use bdat::{label_hash, Label, ModernTable, TableAccessor};
use game_data::character::{
    Art, Attachment, Character, CharacterData, CharacterLang, Class, Costume, Skill, SoulHack,
};

use crate::lang::{filter_table_from_bdat, hash_table_from_bdat};
use crate::{dlc, BdatRegistry, LangBdatRegistry, ModernRow};

const UI_NAME_HASHES: [Label; 6] = [
    label_hash!("UIName1"),
    label_hash!("UIName2"),
    label_hash!("UIName3"),
    label_hash!("UIName4"),
    label_hash!("UIName5"),
    label_hash!("UIName6"),
];

pub fn read_data(bdat: &BdatRegistry) -> CharacterData {
    let characters = bdat.table(label_hash!("CHR_PC"));
    let arts = bdat.table(label_hash!("BTL_Arts_PC"));
    let skills = bdat.table(label_hash!("BTL_Skill_PC"));
    let classes = bdat.table(label_hash!("BTL_Talent"));
    let attachments = bdat.table(label_hash!("MNU_Attachment"));
    let costumes_table = bdat.table(label_hash!("RSC_PcCostumeOpen"));

    let characters = read_id_name_pairs(characters).map(|(id, name)| {
        let pow_augment = characters.row(id).get(label_hash!("PowAugment")).as_str();
        let pow_augment = (!pow_augment.is_empty()).then(|| ()).and_then(|_| {
            let table = bdat.get_table(label_hash!(pow_augment))?;
            Some(dlc::pow_augment::read_for_character(table))
        });
        Character {
            id,
            name_id: name,
            pow_augment,
        }
    });
    let arts = read_id_name_pairs(arts).map(|(id, name)| Art {
        id,
        name_id: name,
        soul_hack: read_soul_hack(
            &arts.row(id),
            Label::Hash(0xA2275574),
            label_hash!("EnArtsAchieve"),
        ),
    });
    let skills = read_id_name_pairs(skills).map(|(id, name)| Skill {
        id,
        name_id: name,
        soul_hack: read_soul_hack(
            &skills.row(id),
            Label::Hash(0xA6E42F10),
            label_hash!("EnSkillAchieve"),
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
    let characters = bdat.table(label_hash!("msg_player_name"));
    let arts = bdat.table(label_hash!("msg_btl_arts_name"));
    let skills = bdat.table(label_hash!("msg_btl_skill_name"));
    let classes = bdat.table(label_hash!("msg_btl_talent_name"));
    let misc = bdat.table(label_hash!("msg_mnu_char_ms"));
    let npcs = bdat.table(label_hash!("msg_npc_name"));

    CharacterLang {
        characters: filter_table_from_bdat(characters),
        arts: filter_table_from_bdat(arts),
        skills: filter_table_from_bdat(skills),
        classes: filter_table_from_bdat(classes),
        misc: filter_table_from_bdat(misc),
        npcs: hash_table_from_bdat(npcs),
    }
}

fn read_id_name_pairs<'a>(table: &'a ModernTable) -> impl Iterator<Item = (usize, usize)> + 'a {
    table.rows().map(|row| {
        let name = row.get(label_hash!("Name")).to_integer() as usize;
        (row.id(), name)
    })
}

fn read_costume(table: &ModernTable, char_id: usize, out: &mut Vec<Costume>) {
    for row in table.rows() {
        let id = row.id();
        let name_id = row.get(&UI_NAME_HASHES[char_id]).to_integer() as usize;
        out.push(Costume { id, name_id })
    }
}

fn read_soul_hack(
    row: &ModernRow,
    status_hash: Label,
    achievement_hash: Label,
) -> Option<SoulHack> {
    let status = row.get(status_hash).to_integer() as usize;
    let status = NonZeroUsize::new(status)?;

    let achievement = row.get(achievement_hash).to_integer() as usize;
    let achievement = NonZeroUsize::new(achievement)?;

    Some(SoulHack {
        status_flag: status,
        achievement_flag: achievement,
    })
}
