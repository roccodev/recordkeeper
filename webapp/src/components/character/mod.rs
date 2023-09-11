use game_data::lang::{Filterable, Id};
use ybc::{Notification, Select, Tile};
use yew::prelude::*;

use crate::{
    components::character::{appearance::Appearance, class::ClassEditor, stats::CharacterStats},
    data::Data,
    lang::Text,
};

mod appearance;
mod class;
pub mod party;
mod slot;
mod stats;

#[derive(Properties, PartialEq, Clone, Copy)]
pub struct CharacterProps {
    pub char_id: usize,
}

#[derive(Properties, PartialEq)]
pub struct StateSelectorProps<F: Filterable + PartialEq + Id + 'static> {
    /// State to update. Value is the object's ID (e.g. character ID, class ID...)
    pub state: UseStateHandle<usize>,
    pub values: &'static [F],
}

#[derive(Properties, PartialEq)]
pub struct UpdateSelectorProps<F: Filterable + PartialEq + Id + 'static> {
    pub values: &'static [F],
    pub current: usize,
    pub update: Callback<usize>,
}

#[function_component]
pub fn CharacterEditor(props: &CharacterProps) -> Html {
    html! {
        <>
            <Tile classes={classes!("notification")}>
                <CharacterStats ..*props />
                <Appearance char_idx={props.char_id.checked_sub(1).unwrap()} />
            </Tile>
            <Notification>
                <ClassEditor char_id={props.char_id} />
            </Notification>
        </>
    }
}

#[function_component]
pub fn Selector<F: Filterable + PartialEq + Id + 'static>(props: &StateSelectorProps<F>) -> Html {
    let state = props.state.clone();
    let update = Callback::from(move |i| state.set(i));

    html!(<UpdateSelector<F> update={update} values={props.values} current={*props.state} />)
}

#[function_component]
pub fn UpdateSelector<F: Filterable + PartialEq + Id + 'static>(
    props: &UpdateSelectorProps<F>,
) -> Html {
    let data = use_context::<Data>().unwrap();
    let update = {
        let update = props.update.clone();
        Callback::from(move |s: String| update.emit(s.parse::<usize>().unwrap()))
    };

    html! {
        <Select name="class" update={update} value={props.current.to_string()}>
            {for props.values.iter().map(|c| {
                let entry = c.get_filter(data.lang());
                html! {
                    <option value={c.id().to_string()} selected={c.id() == props.current}>
                        {match entry {
                            Some(entry) => entry.text().into(),
                            None => html!(<Text path="unnamed" args={vec![("id".into(), c.id().into())]} />)
                        }}
                    </option>
                }
            })}
        </Select>
    }
}
