use ybc::{Container, Tile};
use yew::prelude::*;

use crate::components::meta::{
    misc::Settings,
    scenario::ScenarioFlag,
    time::{PlayTime, Timestamps},
};

#[function_component]
pub fn SaveMeta() -> Html {
    html! {
        <Container>
            <Tile>
                <Tile classes={classes!("is-parent")}>
                    <PlayTime />
                </Tile>
                <Tile classes={classes!("is-parent")}>
                    <Timestamps />
                </Tile>
            </Tile>
            <Tile>
                <Tile classes={classes!("is-parent")}>
                    <ScenarioFlag />
                </Tile>
                <Tile classes={classes!("is-parent")}>
                    <Settings />
                </Tile>
            </Tile>
        </Container>
    }
}
