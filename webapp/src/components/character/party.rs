use game_data::character::Character;
use ybc::{Button, Control, Field, Icon};
use yew::prelude::*;
use yew_feather::{Minus, Plus};

use crate::{components::character::UpdateSelector, data::Data, lang::Text, save::SaveContext};

#[function_component]
pub fn PartyEditor() -> Html {
    let save_context = use_context::<SaveContext>().unwrap();
    let data = use_context::<Data>().unwrap();
    let save = save_context.get();

    let party = &save.get().save().party_characters;
    let len = party.len();
    let characters = data.game().characters.characters();

    let push = {
        let save_context = save_context.clone();
        Callback::from(move |_: MouseEvent| {
            // Add an extra character = slot index. There are fewer slots than
            // characters so it's a fine choice.
            save_context
                .try_edit(move |save| Ok(save.party_characters.try_push((len + 1).try_into()?)?))
        })
    };
    let pop = {
        let save_context = save_context.clone();
        Callback::from(move |_: MouseEvent| {
            save_context.try_edit(move |save| Ok(save.party_characters.try_pop().map(|_| ())?))
        })
    };

    html! {
        <Field>
            <label class="label"><Text path="character_party" /></label>

            <Field classes={classes!("is-grouped", "is-grouped-multiline")}>
                {for (0..len).map(|i| {
                    let update = {
                        let save_context = save_context.clone();
                        Callback::from(move |new: usize| save_context.edit(move |save| save.party_characters.set(i, new.try_into().unwrap())))
                    };
                    html! {
                        <Control classes={classes!("recordkeeper-party-select")}>
                            <UpdateSelector<Character>
                                current={*party.get(i).unwrap() as usize}
                                update={update}
                                values={characters}
                            />
                        </Control>
                    }
                })}

                <Control>
                    <Button disabled={len == 0} onclick={pop}>
                        <Icon>
                            <Minus />
                        </Icon>
                    </Button>
                </Control>

                <Control>
                    <Button disabled={len >= party.capacity()} onclick={push}>
                        <Icon>
                            <Plus />
                        </Icon>
                    </Button>
                </Control>
            </Field>
        </Field>
    }
}
