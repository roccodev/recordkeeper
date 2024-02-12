use ybc::{Container, Content, Icon};
use yew::prelude::*;
use yew_feather::FilePlus;

use crate::{components::upload::UploadButton, lang::Text};

#[function_component]
pub fn Home() -> Html {
    html! {
        <Container>
            <Content>
                <p class="title">{"Recordkeeper"}</p>
                <p class="subtitle"><Text path="home_subtitle" /></p>
                <div class={classes!("is-flex", "is-align-items-center")}>
                    <span><Text path="home_upload" /></span>
                    {" "}
                    <UploadButton>
                        <Icon><FilePlus /></Icon>
                        <span><Text path="open" /></span>
                    </UploadButton>
                </div>
            </Content>
        </Container>
    }
}
