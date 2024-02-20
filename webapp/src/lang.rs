use std::{borrow::Cow, rc::Rc};

use fluent::{FluentArgs, FluentBundle, FluentResource, FluentValue};
use serde::Deserialize;
use unic_langid::{langid, LanguageIdentifier};

use yew::prelude::*;

static LANG_META: &str = include_str!("../lang/lang.json");

pub type Lang = Rc<LangManager>;
type Bundle = FluentBundle<FluentResource>;

mod imported {
    // Created by build.rs
    include!(concat!(env!("CARGO_TARGET_DIR"), "/lang_in.rs"));
}

use imported::LANGUAGE_FILES;

pub struct LangManager {
    bundle: Bundle,
    pub ui_meta: Vec<LangMeta>,
    pub game_meta: Vec<LangMeta>,
    lang_id: usize,
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

#[derive(Deserialize, Clone)]
pub struct LangMeta {
    pub name: String,
    pub file: String,
    pub flag: String,
    pub lang_id: Option<String>,
}

#[function_component]
pub fn Text(props: &TextProps) -> Html {
    let lang = use_context::<Lang>().unwrap();
    let key = props.path.as_cow().to_string(); // TODO
    let args = props.args.iter().cloned().collect();
    let translated = lang.translate_with_args(key, Some(&args));
    html! {translated}
}

impl LangManager {
    pub const DEFAULT_LANG: LanguageIdentifier = langid!("en-US");

    pub fn load(lang_id: LanguageIdentifier) -> Self {
        let lang_id = LANGUAGE_FILES.iter().position(|l| l.0 == lang_id).unwrap();
        let bundle = Self::create_bundle(lang_id);
        let meta: MetaFile = serde_json::from_str(LANG_META).unwrap();

        Self {
            bundle,
            ui_meta: meta.ui,
            game_meta: meta.game,
            lang_id,
        }
    }

    pub fn translate(&self, key: impl Into<Cow<'static, str>>) -> Cow<str> {
        self.translate_with_args(key, None)
    }

    pub fn translate_with_args<'bundle>(
        &'bundle self,
        key: impl Into<Cow<'static, str>>,
        args: Option<&'bundle FluentArgs<'_>>,
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
        &self.ui_meta[self.lang_id]
    }

    pub fn game_meta(&self, lang_id: &str) -> &LangMeta {
        self.game_meta
            .iter()
            .find(|m| m.file == lang_id)
            .expect("data lang not registered")
    }

    /// Returns the user's supported language if it is supported.
    ///
    /// If the user's language is not supported, [`Self::DEFAULT_LANG`]
    /// is returned instead.
    ///
    /// The language preference is read from `navigator.languages`.
    pub fn get_preferred_language() -> LanguageIdentifier {
        let win = web_sys::window().unwrap();

        for lang in win.navigator().languages() {
            let Some(lang): Option<LanguageIdentifier> =
                lang.as_string().and_then(|s| s.parse().ok())
            else {
                continue;
            };
            if LANGUAGE_FILES.iter().any(|l| l.0 == lang) {
                return lang;
            }
        }

        Self::DEFAULT_LANG
    }

    fn create_bundle(lang_id: usize) -> Bundle {
        let language = &LANGUAGE_FILES[lang_id].0;
        let locales = if language != &Self::DEFAULT_LANG {
            vec![LANGUAGE_FILES[lang_id].0.clone(), Self::DEFAULT_LANG]
        } else {
            vec![Self::DEFAULT_LANG]
        };

        let fallback_res = FluentResource::try_new(
            LANGUAGE_FILES
                .iter()
                .find(|(lang, _)| lang == &Self::DEFAULT_LANG)
                .expect("fallback lang file not found")
                .1
                .to_string(),
        )
        .expect("invalid fallback lang file");

        let mut bundle = FluentBundle::new(locales);
        bundle.add_resource(fallback_res).unwrap();

        if language != &Self::DEFAULT_LANG {
            let resource_text = LANGUAGE_FILES[lang_id].1.to_string();
            let res = FluentResource::try_new(resource_text).expect("invalid lang file");
            bundle.add_resource_overriding(res);
        }

        bundle
    }
}

impl PartialEq for LangManager {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&self.bundle, &other.bundle)
    }
}
