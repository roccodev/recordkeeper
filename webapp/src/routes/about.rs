use ybc::{Container, Content};
use yew::prelude::*;

#[function_component]
pub fn About() -> Html {
    html! {
        <Container>
            <Content>
                <h1>{"Recordkeeper"}</h1>
                <p>{"Recordkeeper is a save editing tool for Xenoblade Chronicles 3 and Future Redeemed."}</p>
                <p>{"The web interface runs on WebAssembly, and is styled using the Bulma CSS library."}</p>
            </Content>
        </Container>
    }
}
