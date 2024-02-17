use game_data::{GameData, LanguageData};
use gloo_net::http::Request;
use log::{info, warn};
use std::error::Error;
use std::io::Cursor;
use std::rc::Rc;
use yew::{Reducible, UseReducerHandle};

pub static DEFAULT_GAME_LANG: &str = "gb";
static RES_GAME_DATA: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/res/game_data.bin"));
static RES_DEFAULT_LANG: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/res/lang_",
    "gb",
    ".bin"
));

pub struct DataManager {
    game_data: &'static GameData,
    current_language_data: Rc<LanguageData>,
    lang_id: String,
}

pub type Data = UseReducerHandle<DataManager>;

pub enum DataAction {
    ChangeLanguage(LanguageData, String),
}

impl DataManager {
    pub fn load() -> Self {
        let game_data =
            game_data::load_game_data(Cursor::new(RES_GAME_DATA)).expect("game data load");
        let lang_data = load_default_lang().expect("lang data load");

        info!("Loaded game data.");

        Self {
            game_data: Box::leak(Box::new(game_data)),
            current_language_data: Rc::new(lang_data),
            lang_id: DEFAULT_GAME_LANG.to_string(),
        }
    }

    pub fn game(&self) -> &'static GameData {
        &self.game_data
    }

    pub fn lang(&self) -> &LanguageData {
        &self.current_language_data
    }

    pub fn to_lang(&self) -> Rc<LanguageData> {
        Rc::clone(&self.current_language_data)
    }

    pub fn lang_id(&self) -> &str {
        &self.lang_id
    }
}

pub async fn load_lang(lang_id: &str) -> Result<LanguageData, Box<dyn Error>> {
    if lang_id == DEFAULT_GAME_LANG {
        return load_default_lang();
    }

    let http_res = Request::get(&format!("/res/lang_{lang_id}.bin"))
        .send()
        .await?;
    if !http_res.ok() && http_res.status() != 304 {
        warn!("Failed to load language data for '{lang_id}'. Response: {http_res:?}");
        return load_default_lang();
    }
    game_data::load_lang_data(Cursor::new(http_res.binary().await?))
}

fn load_default_lang() -> Result<LanguageData, Box<dyn Error>> {
    game_data::load_lang_data(Cursor::new(RES_DEFAULT_LANG))
}

impl PartialEq for DataManager {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.game_data, other.game_data) && self.lang_id.eq(&other.lang_id)
    }
}

impl Reducible for DataManager {
    type Action = DataAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let DataAction::ChangeLanguage(new_lang, lang_id) = action;
        if self.lang_id == lang_id {
            return self;
        }
        Rc::new(Self {
            game_data: self.game_data,
            current_language_data: Rc::new(new_lang),
            lang_id,
        })
    }
}
