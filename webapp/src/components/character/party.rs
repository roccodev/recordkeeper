use game_data::character::Character;
use recordkeeper::{character::PARTY_MAX, util::FixVec, SaveData};
use ybc::{Button, Control, Field, Icon};
use yew::prelude::*;
use yew_feather::{Minus, Plus};

use crate::{components::select::UpdateSelector, data::Data, lang::Text, save::SaveContext};

#[derive(PartialEq, Properties)]
pub struct PartyEditorProps<const N: usize, E>
where
    E: PartyVecEditor<N> + PartialEq,
{
    pub editor: E,
}

pub trait PartyVecEditor<const N: usize> {
    fn get<'s>(&self, save: &'s SaveData) -> &'s FixVec<u16, N>;
    fn get_mut<'s>(&self, save: &'s mut SaveData) -> &'s mut FixVec<u16, N>;
}

#[derive(Copy, Clone, PartialEq)]
pub struct SavePartyEditor;

#[derive(Copy, Clone, PartialEq)]
pub struct FormationPartyEditor {
    pub formation: usize,
}

#[function_component]
pub fn PartyEditor<const N: usize, E>(props: &PartyEditorProps<N, E>) -> Html
where
    E: PartyVecEditor<N> + PartialEq + Clone + 'static,
{
    let save_context = use_context::<SaveContext>().unwrap();
    let data = use_context::<Data>().unwrap();
    let save = save_context.get();

    let party = &props.editor.get(save.get_save());
    let len = party.len();
    let characters = data.game().characters.characters();

    let push = {
        let save_context = save_context.clone();
        let editor = props.editor.clone();
        Callback::from(move |_: MouseEvent| {
            let editor = editor.clone();
            // Add an extra character = slot index. There are fewer slots than
            // characters so it's a fine choice.
            save_context
                .try_edit(move |save| Ok(editor.get_mut(save).try_push((len + 1).try_into()?)?))
        })
    };
    let pop = {
        let save_context = save_context.clone();
        let editor = props.editor.clone();
        Callback::from(move |_: MouseEvent| {
            let editor = editor.clone();
            save_context.try_edit(move |save| Ok(editor.get_mut(save).try_pop().map(|_| ())?))
        })
    };

    html! {
        <Field>
            <label class="label"><Text path="character_party" /></label>

            <Field classes={classes!("is-grouped", "is-grouped-multiline")}>
                {for (0..len).map(|i| {
                    let update = {
                        let save_context = save_context.clone();
                        let editor = props.editor.clone();
                        Callback::from(move |new: usize| {
                            let editor = editor.clone();
                            save_context.edit(move |save| editor.get_mut(save).set(i, new.try_into().unwrap()))
                        })
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

impl PartyVecEditor<PARTY_MAX> for SavePartyEditor {
    fn get<'s>(&self, save: &'s SaveData) -> &'s FixVec<u16, PARTY_MAX> {
        &save.party_characters
    }

    fn get_mut<'s>(&self, save: &'s mut SaveData) -> &'s mut FixVec<u16, PARTY_MAX> {
        &mut save.party_characters
    }
}

impl PartyVecEditor<PARTY_MAX> for FormationPartyEditor {
    fn get<'s>(&self, save: &'s SaveData) -> &'s FixVec<u16, PARTY_MAX> {
        &save.party_formations[self.formation].party
    }

    fn get_mut<'s>(&self, save: &'s mut SaveData) -> &'s mut FixVec<u16, PARTY_MAX> {
        &mut save.party_formations[self.formation].party
    }
}
