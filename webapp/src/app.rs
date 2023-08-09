use crate::components::nav::Navbar;
use crate::components::sidebar::Sidebar;
use crate::dialog::DialogRenderer;
use crate::lang::{Lang, LangManager};
use crate::save::SaveProvider;

use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let lang = LangManager::DEFAULT_LANG;
    let lang = use_memo(|lang| LangManager::load(lang.clone()), lang);

    html! {
        <ContextProvider<Lang> context={lang}>
            <DialogRenderer>
                <SaveProvider>
                    <Sidebar />
                    <Navbar />
                </SaveProvider>
            </DialogRenderer>
        </ContextProvider<Lang>>
    }
}

pub fn render_app() {
    yew::Renderer::<App>::new().render();
}
