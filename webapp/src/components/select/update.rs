use game_data::lang::{Filterable, Id};
use yew::prelude::*;

use crate::{
    components::{edit::Editor, select::HtmlSelect},
    data::Data,
    lang::Text,
    save::SaveContext,
};

#[derive(Properties, PartialEq)]
pub struct UpdateSelectorProps<F: Filterable + PartialEq + Id + 'static> {
    pub values: &'static [F],
    pub current: usize,
    pub update: Callback<usize>,
}

#[derive(Properties, PartialEq)]
pub struct StateSelectorProps<F: Filterable + PartialEq + Id + 'static> {
    /// State to update. Value is the object's ID (e.g. character ID, class ID...)
    pub state: UseStateHandle<usize>,
    pub values: &'static [F],
}

#[derive(Properties, PartialEq)]
pub struct EditorSelectorProps<E: PartialEq, F: Filterable + PartialEq + Id + 'static> {
    pub editor: E,
    pub values: &'static [F],
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
        Callback::from(move |s: String| {
            update.emit(s.parse::<usize>().unwrap());
        })
    };

    let selected_index = props
        .values
        .iter()
        .position(|c| c.id() == props.current)
        .unwrap_or_default();

    html! {
        <HtmlSelect on_change={update} value={props.current.to_string()} selected_idx={selected_index}>
            {for props.values.iter().enumerate().map(|(i, c)| {
                let entry = c.get_filter(data.lang());
                html! {
                    <option value={c.id().to_string()} selected={i == selected_index}>
                        {match entry {
                            Some(entry) => entry.text().into(),
                            None => html!(<Text path="unnamed" args={vec![("id".into(), c.id().into())]} />)
                        }}
                    </option>
                }
            })}
        </HtmlSelect>
    }
}

#[function_component]
pub fn EditorSelector<E, F>(props: &EditorSelectorProps<E, F>) -> Html
where
    E: PartialEq + Editor<Target = usize>,
    F: Filterable + PartialEq + Id + 'static,
{
    let save_context = use_context::<SaveContext>().unwrap();
    let editor = props.editor;

    let update = {
        let save = save_context.clone();
        Callback::from(move |i| save.edit(move |save| editor.set(save, i)))
    };

    html!(<UpdateSelector<F> update={update} values={props.values} current={editor.get(save_context.get().get_save())} />)
}
