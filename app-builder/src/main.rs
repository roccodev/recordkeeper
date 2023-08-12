use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

use bdat::{Label, SwitchEndian, Table};

mod item;

macro_rules! const_hash {
    ($name:literal) => {{
        const HASH: u32 = ::bdat::hash::murmur3_str($name);
        ::bdat::Label::Hash(HASH)
    }};
}

pub(crate) use const_hash;
use game_data::GameData;

pub struct BdatRegistry<'b> {
    game_tables: HashMap<Label, Table<'b>>,
}

fn main() {
    let bdat_path = std::env::args().nth(1).unwrap();
    let out_path = std::env::args().nth(2).unwrap();
    std::fs::create_dir_all(&out_path).unwrap();

    let bdat = BdatRegistry::load(&bdat_path);

    let game_data = read_game_data(&bdat);
    let out_game_data = File::create(Path::new(&out_path).join("game_data.bin")).unwrap();
    game_data::save_game_data(&game_data, out_game_data).unwrap();
}

fn read_game_data(bdat: &BdatRegistry) -> GameData {
    GameData {
        items: item::load_items(bdat),
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
