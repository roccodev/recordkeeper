use bdat::{
    hash::PreHashedMap, label_hash, BdatFile, Label, ModernCell, ModernTable, RowRef, SwitchEndian,
    TableAccessor,
};
use game_data::{GameData, LanguageData};
use gimmick::GimmickData;
use std::{borrow::Borrow, collections::HashMap, fs::File, io::BufReader, path::Path};

mod character;
mod dlc;
mod enemy;
mod enhance;
mod field;
mod formation;
mod gimmick;
mod item;
mod lang;
mod manual;
mod ouroboros;
mod quest;
mod scenario;

pub type ModernRow<'a, 'b> = RowRef<'a, 'b, ModernCell<'a, 'b>>;

pub struct BdatRegistry<'b> {
    game_tables: HashMap<Label, ModernTable<'b>>,
    gimmicks: PreHashedMap<u32, GimmickData>,
}

pub struct LangBdatRegistry<'b> {
    game: BdatRegistry<'b>,
    tables: HashMap<Label, ModernTable<'b>>,
}

fn main() {
    let bdat_path = std::env::args().nth(1).unwrap();
    let out_path = std::env::args().nth(2).unwrap();
    std::fs::create_dir_all(&out_path).unwrap();

    let mut bdat = BdatRegistry::load(&bdat_path);

    let game_data = read_game_data(&bdat);
    let out_game_data = File::create(Path::new(&out_path).join("game_data.bin")).unwrap();
    game_data::save_game_data(&game_data, out_game_data).unwrap();

    let lang_dirs = std::fs::read_dir(&bdat_path)
        .unwrap()
        .flatten()
        .filter(|dir| dir.path().is_dir())
        .collect::<Vec<_>>();

    for lang in lang_dirs {
        let lang = lang.file_name().into_string().unwrap();
        let lang_bdat = LangBdatRegistry::load(bdat, &bdat_path, &lang);
        let lang_data = read_lang_data(&lang_bdat);
        let out_lang_data =
            File::create(Path::new(&out_path).join(format!("lang_{lang}.bin"))).unwrap();
        game_data::save_lang_data(&lang_data, out_lang_data).unwrap();
        bdat = lang_bdat.game;
    }
}

fn read_game_data(bdat: &BdatRegistry) -> GameData {
    GameData {
        items: item::load_items(bdat),
        enhance: enhance::load_enhance(bdat),
        dlc: dlc::read_dlc_game(bdat),
        manual: manual::read_manual_data(),
        events: scenario::read_scenario_events(bdat),
        quests: quest::read_quests(bdat),
        characters: character::read_data(bdat),
        ouroboros: ouroboros::read_ouroboros(bdat),
        field: field::read_data(bdat),
        enemies: enemy::read_data(bdat),
        formation: formation::read_data(bdat),
    }
}

fn read_lang_data(bdat: &LangBdatRegistry) -> LanguageData {
    LanguageData {
        items: item::load_item_lang(bdat),
        enhance: enhance::load_enhance_lang(bdat),
        dlc: dlc::read_dlc_lang(bdat),
        quests: quest::read_quest_lang(bdat),
        characters: character::read_lang(bdat),
        field: field::read_lang(bdat),
        enemies: enemy::read_lang(bdat),
        formation: formation::read_lang(bdat),
    }
}

impl<'b> BdatRegistry<'b> {
    fn load(base_path: impl AsRef<Path>) -> Self {
        let mut game_tables = HashMap::default();
        let base_path = base_path.as_ref();

        for file in ["fld", "qst", "btl", "sys", "dlc", "mnu", "map"] {
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

        let gimmicks = game_tables[&label_hash!("SYS_GimmickLocation_dlc04")].clone();
        let gimmicks = gimmicks
            .rows()
            .map(GimmickData::new)
            .map(|gmk| (gmk.gimmick_id, gmk))
            .collect();

        Self {
            game_tables,
            gimmicks,
        }
    }

    pub fn get_table(&self, label: impl Borrow<Label>) -> Option<&ModernTable<'b>> {
        self.game_tables.get(label.borrow())
    }

    pub fn table(&self, label: impl Borrow<Label>) -> &ModernTable<'b> {
        self.get_table(label).expect("table not found")
    }
}

impl<'b> LangBdatRegistry<'b> {
    fn load(game: BdatRegistry<'b>, base_path: impl AsRef<Path>, lang_id: &str) -> Self {
        let mut all_tables = HashMap::default();
        let base_path = base_path.as_ref();

        for file in ["field", "quest", "battle", "system", "dlc", "menu"] {
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

        Self {
            game,
            tables: all_tables,
        }
    }

    pub fn table(&self, label: impl Borrow<Label>) -> &ModernTable<'b> {
        let label = label.borrow();
        self.tables
            .get(label)
            .or_else(|| self.game.game_tables.get(label))
            .expect("no table found")
    }
}
