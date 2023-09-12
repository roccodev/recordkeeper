use game_data::ouroboros::Ouroboros;
use ybc::{Container, Control, Field, Tile};
use yew::prelude::*;

use crate::{
    components::{character::Selector, ouroboros::OuroborosEditor},
    data::Data,
    lang::Text,
};

#[function_component]
pub fn OuroborosPage() -> Html {
    let char_id = use_state(|| 1);
    let data = use_context::<Data>().unwrap();

    html! {
        <Container>
            <Tile classes={classes!("mb-2")}>
                <Field>
                    <label class="label"><Text path="ouroboros_character" /></label>
                    <Control>
                        <Selector<Ouroboros> state={char_id.clone()} values={data.game().ouroboros.as_slice()} />
                    </Control>
                </Field>
            </Tile>
            <div>
                <OuroborosEditor char_id={*char_id} />
            </div>
        </Container>
    }
}
