use crate::components::nav::Navbar;
use crate::components::sidebar::Sidebar;
use crate::dialog::DialogRenderer;
use crate::lang::{Lang, LangManager};
use crate::routes::Route;
use crate::save::SaveProvider;

use crate::data;
use crate::data::{Data, DataAction, DataManager};
use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use unic_langid::LanguageIdentifier;
use yew::prelude::*;
use yew_router::history::{AnyHistory, MemoryHistory};
use yew_router::{Router, Switch};

#[derive(Serialize, Deserialize, Clone, Default)]
struct Preferences {
    ui_lang: Option<String>,
    game_lang: Option<String>,
}

#[function_component]
fn App() -> Html {
    let data = use_reducer_eq(DataManager::load);

    let prefs_src: Preferences = LocalStorage::get("rkp_preferences").unwrap_or_default();
    let prefs = prefs_src.clone();

    let lang = prefs
        .ui_lang
        .map(|s| s.parse().unwrap())
        .unwrap_or_else(LangManager::get_preferred_language);
    let lang_state = use_state(|| lang);
    let lang = (*lang_state).clone();
    let lang = use_memo(|lang| LangManager::load(lang.clone()), lang);

    let game_lang = prefs
        .game_lang
        .unwrap_or_else(|| data::DEFAULT_GAME_LANG.to_string());
    let game_lang_state = use_state(|| game_lang);
    let for_future = data.clone();
    let game_lang = (*game_lang_state).clone();
    wasm_bindgen_futures::spawn_local(async move {
        let lang = data::load_lang(&game_lang).await.expect("extra lang load");
        for_future.dispatch(DataAction::ChangeLanguage(lang, game_lang));
    });

    let router_history = use_state(|| AnyHistory::from(MemoryHistory::new()));

    let prefs_callback = Callback::from(move |prefs: Preferences| {
        LocalStorage::set("rkp_preferences", prefs).unwrap()
    });

    let ui_lang_callback = {
        let prefs_callback = prefs_callback.clone();
        let prefs_src = prefs_src.clone();
        Callback::from(move |lang_id: String| {
            let id: LanguageIdentifier = lang_id.parse().unwrap();
            prefs_callback.emit(Preferences {
                ui_lang: Some(lang_id),
                ..prefs_src.clone()
            });
            lang_state.set(id);
        })
    };

    let game_lang_callback = Callback::from(move |id: String| {
        prefs_callback.emit(Preferences {
            game_lang: Some(id.clone()),
            ..prefs_src.clone()
        });
        game_lang_state.set(id);
    });

    html! {
        <ContextProvider<Lang> context={lang}>
            <ContextProvider<Data> context={data}>
                <DialogRenderer>
                    <SaveProvider>
                        <Router history={(*router_history).clone()} basename="/">
                            <Sidebar />
                            <Navbar game_lang_callback={game_lang_callback} ui_lang_callback={ui_lang_callback} />
                            <Switch<Route> render={crate::routes::render} />
                        </Router>
                    </SaveProvider>
                </DialogRenderer>
            </ContextProvider<Data>>
        </ContextProvider<Lang>>
    }
}

pub fn render_app() {
    yew::Renderer::<App>::new().render();
}
