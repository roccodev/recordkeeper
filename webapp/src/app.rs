use crate::components::nav::Navbar;
use crate::components::sidebar::Sidebar;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <>
          <Sidebar />
          <Navbar />
        </>
    }
}

pub fn render_app() {
    yew::Renderer::<App>::new().render();
}
