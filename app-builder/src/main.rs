use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

use bdat::{Label, SwitchEndian, Table};

mod enhance;
mod item;
mod lang;

macro_rules! const_hash {
    ($name:literal) => {{
        const HASH: u32 = ::bdat::hash::murmur3_str($name);
        ::bdat::Label::Hash(HASH)
    }};
}

pub(crate) use const_hash;
use game_data::{GameData, LanguageData};

pub struct BdatRegistry<'b> {
    game_tables: HashMap<Label, Table<'b>>,
}

pub struct LangBdatRegistry<'b>(HashMap<Label, Table<'b>>);

const LANG_IDS: &[&str] = &["gb"];

fn main() {
    let bdat_path = std::env::args().nth(1).unwrap();
    let out_path = std::env::args().nth(2).unwrap();
    std::fs::create_dir_all(&out_path).unwrap();

    let bdat = BdatRegistry::load(&bdat_path);

    let game_data = read_game_data(&bdat);
    let out_game_data = File::create(Path::new(&out_path).join("game_data.bin")).unwrap();
    game_data::save_game_data(&game_data, out_game_data).unwrap();

    for lang in LANG_IDS {
        let bdat = LangBdatRegistry::load(&bdat_path, lang);
        let lang_data = read_lang_data(&bdat);
        let out_lang_data =
            File::create(Path::new(&out_path).join(format!("lang_{lang}.bin"))).unwrap();
        game_data::save_lang_data(&lang_data, out_lang_data).unwrap();
    }
}

fn read_game_data(bdat: &BdatRegistry) -> GameData {
    GameData {
        items: item::load_items(bdat),
        enhance: enhance::load_enhance(bdat),
    }
}

fn read_lang_data(bdat: &LangBdatRegistry) -> LanguageData {
    LanguageData {
        items: item::load_item_lang(bdat),
        enhance: enhance::load_enhance_lang(bdat),
    }
}

impl<'b> BdatRegistry<'b> {
    fn load(base_path: impl AsRef<Path>) -> Self {
        let mut game_tables = HashMap::default();
        let base_path = base_path.as_ref();

        for file in ["fld", "qst", "btl", "sys"] {
            let reader =
                BufReader::new(File::open(base_path.join(format!("{file}.bdat"))).unwrap());
            let tables = bdat::modern::from_reader::<_, SwitchEndian>(reader)
                .unwrap()
                .get_tables()
                .unwrap();
            for table in tables {
                game_tables.insert(table.name().clone(), table);
            }
        }

        Self { game_tables }
    }

    pub fn table(&self, label: &Label) -> &Table<'b> {
        &self.game_tables[label]
    }
}

impl<'b> LangBdatRegistry<'b> {
    fn load(base_path: impl AsRef<Path>, lang_id: &str) -> Self {
        let mut all_tables = HashMap::default();
        let base_path = base_path.as_ref();

        for file in ["field", "quest", "battle", "system"] {
            let reader = BufReader::new(
                File::open(base_path.join(format!("{lang_id}/game/{file}.bdat"))).unwrap(),
            );
            let tables = bdat::modern::from_reader::<_, SwitchEndian>(reader)
                .unwrap()
                .get_tables()
                .unwrap();
            for table in tables {
                all_tables.insert(table.name().clone(), table);
            }
        }

        Self(all_tables)
    }

    pub fn table(&self, label: &Label) -> &Table<'b> {
        &self.0[label]
    }
}
