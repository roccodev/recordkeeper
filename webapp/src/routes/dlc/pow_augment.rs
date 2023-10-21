use game_data::character::Character;
use ybc::{Container, Control, Field, Notification, Tile};
use yew::prelude::*;

use crate::{
    components::{character::Selector, dlc::pow_augment::PowAugmentEditor},
    data::Data,
    lang::Text,
    save::SaveContext,
};

#[function_component]
pub fn PowAugmentPage() -> Html {
    let data = use_context::<Data>().unwrap();
    let save_context = use_context::<SaveContext>().unwrap();

    let dlc4 = save_context.get().get().save().is_dlc4();
    let char_id_state = use_state(|| {
        if dlc4 {
            36 // Matthew
        } else {
            32 // Ino
        }
    });
    let char_id = *char_id_state;

    let pow_augment = data
        .game()
        .characters
        .get_character(char_id)
        .and_then(|c| c.pow_augment.as_ref())
        .expect("no pow augment");

    html! {
        <Container>
            <Tile classes={classes!("mb-2")}>
                <Field>
                    <label class="label"><Text path="pow_augment_character" /></label>
                    <Control>
                        <Selector<Character> state={char_id_state.clone()} values={data.game().characters.pow_augment_characters.as_ref()} />
                    </Control>
                </Field>
            </Tile>
            <Notification>
                <PowAugmentEditor char_id={u8::try_from(char_id).unwrap()} pow_augment={pow_augment} />
            </Notification>
        </Container>
    }
}
