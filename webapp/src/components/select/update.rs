use std::slice;

use game_data::{
    lang::{Filterable, Id, SortKey},
    IdInt,
};
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
    pub current: IdInt,
    pub update: Callback<IdInt>,
    #[prop_or_default]
    pub sort_key: Option<SortKey>,
}

#[derive(Properties, PartialEq)]
pub struct StateSelectorProps<F: Filterable + PartialEq + Id + 'static> {
    /// State to update. Value is the object's ID (e.g. character ID, class ID...)
    pub state: UseStateHandle<IdInt>,
    pub values: &'static [F],
    #[prop_or_default]
    pub sort_key: Option<SortKey>,
}

#[derive(Properties, PartialEq)]
pub struct EditorSelectorProps<E: PartialEq, F: Filterable + PartialEq + Id + 'static> {
    pub editor: E,
    pub values: &'static [F],
}

enum OptSort<'a, T> {
    Owned(Vec<&'a T>),
    Borrowed(&'a [T]),
}

enum OptSortIter<'a, T> {
    Owned(slice::Iter<'a, &'a T>),
    Borrowed(slice::Iter<'a, T>),
}

#[function_component]
pub fn Selector<F: Filterable + PartialEq + Id + 'static>(props: &StateSelectorProps<F>) -> Html {
    let state = props.state.clone();
    let update = Callback::from(move |i| state.set(i));

    html!(<UpdateSelector<F> update={update} values={props.values} current={*props.state} sort_key={props.sort_key.clone()} />)
}

#[function_component]
pub fn UpdateSelector<F: Filterable + PartialEq + Id + 'static>(
    props: &UpdateSelectorProps<F>,
) -> Html {
    let data = use_context::<Data>().unwrap();

    let update = {
        let update = props.update.clone();
        Callback::from(move |s: String| {
            update.emit(s.parse::<IdInt>().unwrap());
        })
    };

    let values = if let Some(sort_key) = &props.sort_key {
        OptSort::Owned(sort_key.list(props.values))
    } else {
        OptSort::Borrowed(props.values)
    };

    let selected_index = values
        .iter()
        .position(|c| c.id() == props.current)
        .unwrap_or_default();

    html! {
        <HtmlSelect on_change={update} value={props.current.to_string()} selected_idx={selected_index}>
            {for values.iter().enumerate().map(|(i, c)| {
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
    E: PartialEq + Editor<Target = IdInt>,
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

impl<'a, T> OptSort<'a, T> {
    fn iter(&'a self) -> impl Iterator<Item = &'a T> {
        match self {
            OptSort::Owned(v) => OptSortIter::Owned(v.iter()),
            OptSort::Borrowed(s) => OptSortIter::Borrowed(s.iter()),
        }
    }
}

impl<'a, T> Iterator for OptSortIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            OptSortIter::Owned(i) => i.next().copied(),
            OptSortIter::Borrowed(i) => i.next(),
        }
    }
}
