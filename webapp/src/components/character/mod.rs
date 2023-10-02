use game_data::{
    character::Class,
    lang::{Filterable, Id},
};
use recordkeeper::{character::class::CharacterClass, flags::BitFlags, SaveData};
use strum::{EnumIter, IntoEnumIterator};
use web_sys::HtmlSelectElement;
use ybc::{Control, Field, Notification, Tile};
use yew::prelude::*;

use crate::{
    components::character::{appearance::Appearance, class::ClassEditor, stats::CharacterStats},
    components::{
        character::class::ClassAccessor,
        edit::{editor, CheckboxInput},
    },
    data::Data,
    lang::Text,
};

mod appearance;
pub mod class;
pub mod formation;
pub mod party;
pub mod slot;
mod stats;

#[rustfmt::skip]
editor!(
    CharacterSetEditor, 
    bool,
    get |editor, save| editor.set.get(&save).get(editor.char_idx).unwrap() != 0,
    set |editor, save, new| editor.set.get_mut(save).set(editor.char_idx, u8::from(new).into()),
    capture set: CharacterSet, char_idx: usize
);

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

#[derive(EnumIter, Clone, Copy, PartialEq)]
enum CharacterSet {
    Selectable,
    Permanent,
    Temporary,
}

#[derive(Clone, Copy, PartialEq)]
pub enum CharacterAccessor {
    Save { idx: usize },
    Formation { formation: usize, id: u16 },
}

#[function_component]
pub fn CharacterEditor(props: &CharacterProps) -> Html {
    let data = use_context::<Data>().unwrap();
    let char_idx = props.char_id.checked_sub(1).unwrap();
    let class_id = use_state(|| 1); // TODO use selected class

    let accessor = CharacterAccessor::Save { idx: char_idx };

    html! {
        <>
            <Notification>
                <Field classes={classes!("is-grouped")}>
                    {for CharacterSet::iter().map(|set| html! {
                        <Control>
                            <CheckboxInput<CharacterSetEditor> editor={CharacterSetEditor { set, char_idx }}>
                                {" "}{set.lang()}
                            </CheckboxInput<CharacterSetEditor>>
                        </Control>
                    })}
                </Field>
                <Tile classes={classes!("notification")}>
                    <CharacterStats ..*props />
                    <Appearance accessor={accessor} char_id={props.char_id} />
                </Tile>
            </Notification>
            <Notification>
                <Tile>
                    <Field>
                        <label class="label"><Text path="character_class" /></label>
                        <Control>
                            <Selector<Class> state={class_id.clone()} values={data.game().characters.classes()} />
                        </Control>
                    </Field>
                </Tile>
                <ClassEditor accessor={accessor.into_class(*class_id)} stats={true} />
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
    let select_ref = use_node_ref();

    let update = {
        let update = props.update.clone();
        Callback::from(move |s: String| {
            update.emit(s.parse::<usize>().unwrap());
        })
        .reform(|ev: web_sys::Event| {
            let select: HtmlSelectElement = ev
                .target_dyn_into()
                .expect("event target should be a select");
            select.value()
        })
    };

    let selected_index = props
        .values
        .iter()
        .position(|c| c.id() == props.current)
        .unwrap_or_default();

    // Firefox workaround. For some reason, sometimes the
    // select element fails to update its selectedIndex property
    // when the DOM is refreshed
    {
        let select_ref = select_ref.clone();
        use_effect(move || {
            let select: HtmlSelectElement = select_ref.cast().unwrap();
            select.set_selected_index(selected_index.try_into().unwrap());
        });
    }

    html! {
        <div class="select">
            <select ref={select_ref} name="selector" onchange={update} value={props.current.to_string()}>
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
            </select>
        </div>
    }
}

impl CharacterSet {
    fn get<'s>(&self, save: &'s SaveData) -> &'s BitFlags<1, 2> {
        match self {
            CharacterSet::Selectable => &save.character_sets.selectable_characters,
            CharacterSet::Permanent => &save.character_sets.permanent_characters,
            CharacterSet::Temporary => &save.character_sets.temporary_characters,
        }
    }

    fn get_mut<'s>(&self, save: &'s mut SaveData) -> &'s mut BitFlags<1, 2> {
        match self {
            CharacterSet::Selectable => &mut save.character_sets.selectable_characters,
            CharacterSet::Permanent => &mut save.character_sets.permanent_characters,
            CharacterSet::Temporary => &mut save.character_sets.temporary_characters,
        }
    }

    fn lang(&self) -> Html {
        let id = match self {
            CharacterSet::Selectable => "selectable",
            CharacterSet::Permanent => "permanent",
            CharacterSet::Temporary => "temp",
        };
        html!(<Text path={format!("character_set_{id}")} />)
    }
}

impl CharacterAccessor {
    pub fn get_costume_id(&self, save: &SaveData) -> u16 {
        match self {
            CharacterAccessor::Save { idx } => save.characters[*idx].costume_id,
            CharacterAccessor::Formation { formation, id } => {
                save.party_formations[*formation]
                    .character(*id)
                    .unwrap()
                    .costume_id
            }
        }
    }

    pub fn set_costume_id(&self, save: &mut SaveData, costume: u16) {
        match self {
            CharacterAccessor::Save { idx } => save.characters[*idx].costume_id = costume,
            CharacterAccessor::Formation { formation, id } => {
                save.party_formations[*formation]
                    .character_mut(*id)
                    .costume_id = costume
            }
        }
    }

    pub fn get_attachment(&self, save: &SaveData) -> u8 {
        match self {
            CharacterAccessor::Save { idx } => save.characters[*idx].attachment,
            CharacterAccessor::Formation { formation, id } => {
                save.party_formations[*formation]
                    .character(*id)
                    .unwrap()
                    .attachment
            }
        }
    }

    pub fn set_attachment(&self, save: &mut SaveData, attachment: u8) {
        match self {
            CharacterAccessor::Save { idx } => save.characters[*idx].attachment = attachment,
            CharacterAccessor::Formation { formation, id } => {
                save.party_formations[*formation]
                    .character_mut(*id)
                    .attachment = attachment
            }
        }
    }

    pub fn get_selected_class(&self, save: &SaveData) -> u16 {
        match self {
            CharacterAccessor::Save { idx } => save.characters[*idx].selected_class as u16,
            CharacterAccessor::Formation { formation, id } => {
                save.party_formations[*formation]
                    .character(*id)
                    .unwrap()
                    .current_class
            }
        }
    }

    pub fn into_class(self, class_id: usize) -> ClassAccessor {
        match self {
            CharacterAccessor::Save { idx } => ClassAccessor::Character {
                char: idx,
                class: class_id,
            },
            CharacterAccessor::Formation { formation, id } => ClassAccessor::Formation {
                formation,
                char: id,
            },
        }
    }
}
