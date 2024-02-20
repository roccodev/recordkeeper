use recordkeeper::{flags::BitFlags, SaveData};
use strum::{EnumIter, IntoEnumIterator};
use ybc::{Control, Field, Notification, Tile};
use yew::prelude::*;

use crate::{
    components::character::{appearance::Appearance, class::ClassEditor, stats::CharacterStats},
    components::{
        character::class::ClassAccessor,
        edit::{editor, CheckboxInput},
    },
    lang::Text,
    save::SaveContext,
};

mod appearance;
pub mod class;
pub mod formation;
pub mod party;
pub mod slot;
mod stats;
mod util;

#[rustfmt::skip]
editor!(
    CharacterSetEditor, 
    bool,
    get |editor, save| editor.set.get(save).get(editor.char_idx).unwrap() != 0,
    set |editor, save, new| editor.set.get_mut(save).set(editor.char_idx, u8::from(new).into()),
    capture set: CharacterSet, char_idx: usize
);

#[derive(Properties, PartialEq, Clone, Copy)]
pub struct CharacterProps {
    pub char_id: usize,
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
    let save = use_context::<SaveContext>().unwrap();

    let char_idx = props.char_id.checked_sub(1).unwrap();
    let class_id = save.get().get_save().characters[char_idx].selected_class;

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
                <ClassEditor accessor={accessor.into_class(class_id as usize)} stats={true} />
            </Notification>
        </>
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
