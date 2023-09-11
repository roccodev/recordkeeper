use game_data::character::Character;
use ybc::{Container, Control, Field, Tile};
use yew::prelude::*;

use crate::{
    components::character::{party::PartyEditor, CharacterEditor, Selector},
    data::Data,
    lang::Text,
};

#[function_component]
pub fn Characters() -> Html {
    let char_id = use_state(|| 1);
    let data = use_context::<Data>().unwrap();

    html! {
        <Container>
            <Tile classes={classes!("mb-2")}>
                <Tile>
                    <Field>
                        <label class="label"><Text path="character_character" /></label>
                        <Control>
                            <Selector<Character> state={char_id.clone()} values={data.game().characters.characters()} />
                        </Control>
                    </Field>
                </Tile>
                <Tile classes={classes!("is-10", "is-justify-content-right")}>
                    <PartyEditor />
                </Tile>
            </Tile>
            <div>
                <CharacterEditor char_id={*char_id} />
            </div>
        </Container>
    }
}
