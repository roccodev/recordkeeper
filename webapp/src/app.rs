use crate::components::nav::Navbar;
use crate::components::sidebar::Sidebar;
use crate::dialog::DialogRenderer;
use crate::lang::{Lang, LangManager};
use crate::routes::Route;
use crate::save::SaveProvider;

use crate::data;
use crate::data::{Data, DataAction, DataManager};
use yew::prelude::*;
use yew_router::history::{AnyHistory, MemoryHistory};
use yew_router::{Router, Switch};

#[function_component]
fn App() -> Html {
    let data = use_reducer_eq(DataManager::load);

    let lang = LangManager::DEFAULT_LANG;
    let lang = use_memo(|lang| LangManager::load(lang.clone()), lang);

    let game_lang_state = use_state(|| data::DEFAULT_GAME_LANG.to_string());
    let for_future = data.clone();
    let game_lang = (*game_lang_state).clone();
    wasm_bindgen_futures::spawn_local(async move {
        let lang = data::load_lang(&game_lang).await.expect("extra lang load");
        for_future.dispatch(DataAction::ChangeLanguage(lang, game_lang));
    });

    let router_history = use_state(|| AnyHistory::from(MemoryHistory::new()));

    html! {
        <ContextProvider<Lang> context={lang}>
            <ContextProvider<Data> context={data}>
                <DialogRenderer>
                    <SaveProvider>
                        <Router history={(*router_history).clone()} basename="/">
                            <Sidebar />
                            <Navbar game_lang_state={game_lang_state} />
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
