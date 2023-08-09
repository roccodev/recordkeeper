use crate::components::nav::Navbar;
use crate::components::sidebar::Sidebar;
use crate::dialog::DialogRenderer;
use crate::lang::{Lang, LangManager};
use crate::routes::Route;
use crate::save::SaveProvider;

use yew::prelude::*;
use yew_router::history::{AnyHistory, MemoryHistory};
use yew_router::{Router, Switch};

#[function_component]
fn App() -> Html {
    let lang = LangManager::DEFAULT_LANG;
    let lang = use_memo(|lang| LangManager::load(lang.clone()), lang);

    let router_history = use_state(|| AnyHistory::from(MemoryHistory::new()));

    html! {
        <ContextProvider<Lang> context={lang}>
            <DialogRenderer>
                <SaveProvider>
                    <Router history={(*router_history).clone()} basename="/">
                        <Sidebar />
                        <Navbar />
                        <Switch<Route> render={crate::routes::render} />
                    </Router>
                </SaveProvider>
            </DialogRenderer>
        </ContextProvider<Lang>>
    }
}

pub fn render_app() {
    yew::Renderer::<App>::new().render();
}
