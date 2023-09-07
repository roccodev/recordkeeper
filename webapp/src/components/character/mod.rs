use game_data::lang::{Filterable, Id};
use ybc::{Notification, Select, Tile};
use yew::prelude::*;

use crate::{
    components::character::{class::ClassEditor, stats::CharacterStats},
    data::Data,
};

mod class;
mod stats;

#[derive(Properties, PartialEq, Clone, Copy)]
pub struct CharacterProps {
    pub char_id: usize,
}

#[derive(Properties, PartialEq)]
pub struct SelectorProps<F: Filterable + PartialEq + 'static> {
    pub state: UseStateHandle<usize>,
    pub values: &'static [F],
}

#[function_component]
pub fn CharacterEditor(props: &CharacterProps) -> Html {
    html! {
        <>
            <Tile classes={classes!("notification")}>
                <CharacterStats ..*props />
                <Tile classes={classes!("is-parent")}>
                    {"Appearance"}
                </Tile>
            </Tile>
            <Notification>
                <ClassEditor char_id={props.char_id} />
            </Notification>
        </>
    }
}

#[function_component]
pub fn Selector<F: Filterable + PartialEq + 'static>(props: &SelectorProps<F>) -> Html {
    let data = use_context::<Data>().unwrap();
    let state = props.state.clone();
    let update = Callback::from(move |s: String| state.set(s.parse::<usize>().unwrap()));

    html! {
        <Select name="class" update={update} value={(*props.state).to_string()}>
            {for props.values.iter().filter_map(|c| {
                let entry = c.get_filter(data.lang())?;
                Some(html! {
                    <option value={entry.id().to_string()} selected={entry.id() == *props.state}>
                        {entry.text()}
                    </option>
                })
            })}
        </Select>
    }
}
