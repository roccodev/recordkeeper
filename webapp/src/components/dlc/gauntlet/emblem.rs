use std::num::NonZeroU32;

use game_data::{lang::Nameable, IdInt};
use yew::prelude::*;

use crate::{
    components::edit::{editor, CheckboxInput},
    data::Data,
};

editor!(
    PurchaseEditor,
    bool,
    get |editor, save| save.challenge_battle.emblem(editor.id).unlocked,
    set |editor, save, new| save.challenge_battle.emblem_mut(editor.id).unlocked = new,
    capture id: NonZeroU32
);

#[derive(Properties, PartialEq, Clone)]
pub struct EmblemProps {
    pub id: IdInt,
}

#[function_component]
pub fn EmblemRow(props: &EmblemProps) -> Html {
    let data = use_context::<Data>().unwrap();

    let id = props.id;
    let emblem = data
        .game()
        .dlc
        .challenge
        .get_emblem(id)
        .expect("emblem not found");

    html! {
        <>
            <tr>
                <th>{emblem.id.to_string()}</th>
                <td>{emblem.get_name_str(data.lang())}</td>
                {for (0..emblem.levels).map(|offset| html! {
                    <td><CheckboxInput<PurchaseEditor> editor={PurchaseEditor { id: (emblem.id + offset).try_into().unwrap() }} /></td>
                })}
            </tr>
        </>
    }
}
