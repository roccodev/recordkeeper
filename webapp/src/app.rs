use crate::components::nav::Navbar;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <>
          <Navbar />
        </>
    }
}

pub fn render_app() {
    yew::Renderer::<App>::new().render();
}
