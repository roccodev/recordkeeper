use game_data::character::Character;
use ybc::{Container, Control, Field, Tile};
use yew::prelude::*;

use crate::{
    components::character::{CharacterEditor, Selector},
    data::Data,
    lang::Text,
};

#[function_component]
pub fn Characters() -> Html {
    let char_id = use_state(|| 1);
    let data = use_context::<Data>().unwrap();

    html! {
        <Container>
            <Field>
                <label class="label"><Text path="character_character" /></label>
                <Control>
                    <Selector<Character> state={char_id.clone()} values={data.game().characters.characters()} />
                </Control>
            </Field>
            <CharacterEditor char_id={*char_id} />
        </Container>
    }
}
