use crate::components::nav::Navbar;
use crate::components::sidebar::Sidebar;
use crate::lang::{Lang, LangManager};

use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let lang = LangManager::DEFAULT_LANG;
    let lang = use_memo(|lang| LangManager::load(lang.clone()), lang);

    html! {
        <ContextProvider<Lang> context={lang}>
            <Sidebar />
            <Navbar />
        </ContextProvider<Lang>>
    }
}

pub fn render_app() {
    yew::Renderer::<App>::new().render();
}
