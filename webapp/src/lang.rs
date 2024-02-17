use std::borrow::Cow;
use std::rc::Rc;

use fluent::FluentArgs;
use fluent::FluentBundle;
use fluent::FluentResource;
use fluent::FluentValue;
use serde::Deserialize;
use unic_langid::langid;
use unic_langid::LanguageIdentifier;

use yew::prelude::*;

const LANGUAGE_FILES: [(LanguageIdentifier, &str); 1] =
    [(langid!("en-US"), include_str!("../lang/en_us.ftl"))];
static LANG_META: &str = include_str!("../lang/lang.json");

pub type Lang = Rc<LangManager>;

pub struct LangManager {
    bundle: FluentBundle<FluentResource>,
    pub ui_meta: Vec<LangMeta>,
    pub game_meta: Vec<LangMeta>,
    cur_ui: usize,
}

#[derive(Properties, PartialEq)]
pub struct TextProps {
    pub path: AttrValue,
    #[prop_or_default]
    pub args: Vec<(Cow<'static, str>, FluentValue<'static>)>,
}

#[derive(Deserialize)]
struct MetaFile {
    ui: Vec<LangMeta>,
    game: Vec<LangMeta>,
}

#[derive(Deserialize)]
pub struct LangMeta {
    pub name: String,
    pub file: String,
    pub flag: String,
}

#[function_component]
pub fn Text(props: &TextProps) -> Html {
    let lang = use_context::<Lang>().unwrap();
    let key = props.path.as_cow().to_string(); // TODO
    let args = FluentArgs::from(props.args.iter().cloned().collect());
    let translated = lang.translate_with_args(key, Some(&args));
    html! {translated}
}

impl LangManager {
    pub const DEFAULT_LANG: LanguageIdentifier = langid!("en-US");

    pub fn load(lang_id: LanguageIdentifier) -> Self {
        let locales = if lang_id != Self::DEFAULT_LANG {
            vec![lang_id.clone(), Self::DEFAULT_LANG]
        } else {
            vec![Self::DEFAULT_LANG]
        };

        let resource_text = LANGUAGE_FILES
            .iter()
            .find(|(lang, _)| lang == &lang_id)
            .expect("lang file not found")
            .1
            .to_string();

        let mut bundle = FluentBundle::new(locales);
        bundle
            .add_resource(FluentResource::try_new(resource_text).expect("invalid lang file"))
            .unwrap();

        let meta: MetaFile = serde_json::from_str(LANG_META).unwrap();

        Self {
            bundle,
            ui_meta: meta.ui,
            game_meta: meta.game,
            cur_ui: 0,
        }
    }

    pub fn translate(&self, key: impl Into<Cow<'static, str>>) -> Cow<str> {
        self.translate_with_args(key, None)
    }

    pub fn translate_with_args<'args, 'bundle>(
        &'bundle self,
        key: impl Into<Cow<'static, str>>,
        args: Option<&'bundle FluentArgs<'args>>,
    ) -> Cow<str> {
        let key = key.into();
        let message = match self.bundle.get_message(&key) {
            Some(msg) => msg,
            None => return key,
        };
        let value = message.value().expect("no lang value");
        let mut errors = vec![];
        self.bundle.format_pattern(value, args, &mut errors)
    }

    pub fn ui_meta(&self) -> &LangMeta {
        &self.ui_meta[self.cur_ui]
    }

    pub fn game_meta(&self, lang_id: &str) -> &LangMeta {
        self.game_meta
            .iter()
            .find(|m| m.file == lang_id)
            .expect("data lang not registered")
    }
}

impl PartialEq for LangManager {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&self.bundle, &other.bundle)
    }
}
