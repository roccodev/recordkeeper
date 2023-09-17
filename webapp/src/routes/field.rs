use ybc::{Container, Tile};
use yew::prelude::*;

use crate::components::field::player::{PlayerLoc, ShipLoc};

#[derive(Clone, PartialEq, Copy)]
pub enum FieldTab {
    Player,
    Locations,
    Colonies,
}

#[function_component]
pub fn FieldPage() -> Html {
    let tab = use_state(|| FieldTab::Player);

    html! {
        {match *tab {
            FieldTab::Player => html!(<TabPlayer />),
            FieldTab::Locations => html!(),
            FieldTab::Colonies => html!(),
        }}
    }
}

#[function_component]
fn TabPlayer() -> Html {
    html! {
        <Container>
            <Tile>
                <Tile classes={classes!("is-parent")}>
                    <PlayerLoc />
                </Tile>
                <Tile classes={classes!("is-parent")}>
                    <ShipLoc />
                </Tile>
            </Tile>
            <Tile>
            </Tile>
        </Container>
    }
}
