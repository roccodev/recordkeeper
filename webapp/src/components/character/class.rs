use game_data::{
    character::{Art, Class, Skill},
    item::GemCategory,
};
use recordkeeper::{
    character::class::{
        CharacterClass, CHARACTER_CLASS_ACCESSORY_MAX, CHARACTER_CLASS_ART_MAX,
        CHARACTER_CLASS_SKILL_MAX,
    },
    SaveData,
};
use ybc::{Control, Field, Tile};
use yew::prelude::*;

use crate::{
    components::{
        character::slot::{AccessoryInput, SlotInput},
        edit::{editor, NumberInput},
    },
    data::Data,
    lang::Text,
};

#[derive(Properties, PartialEq)]
pub struct ClassEditorProps {
    pub accessor: ClassAccessor,
    pub stats: bool,
}

#[derive(Properties, PartialEq)]
pub struct ClassProps {
    pub char_id: usize,
    pub class_id: usize,
}

#[rustfmt::skip]
editor!(
    CpEditor,
    u32,
    get |editor, save| editor.class.class_data(save).cp,
    set |editor, save, new| editor.class.class_data_mut(save).cp = new,
    capture class: ClassAccessor
);

#[rustfmt::skip]
editor!(
    UnlockPointsEditor,
    u16,
    get |editor, save| editor.class.class_data(save).unlock_points,
    set |editor, save, new| editor.class.class_data_mut(save).unlock_points = new,
    capture class: ClassAccessor
);

#[rustfmt::skip]
editor!(
    RankEditor,
    u8,
    get |editor, save| editor.class.class_data(save).level,
    set |editor, save, new| editor.class.class_data_mut(save).level = new,
    assert |_, v| (1..=20).contains(v).then_some(()).ok_or_else(String::new),
    capture class: ClassAccessor
);

#[rustfmt::skip]
editor!(
    pub ArtEditor,
    Option<u16>,
    get |editor, save| editor.class.class_data(save).art_slot(editor.slot_idx).get(),
    set |editor, save, new| editor.class.class_data_mut(save).art_slot_mut(editor.slot_idx).set(new),
    capture class: ClassAccessor, slot_idx: usize
);

#[rustfmt::skip]
editor!(
    pub GemEditor,
    Option<u8>,
    get |editor, save| editor.class.class_data(save).gem_slot(editor.slot_idx).get(),
    set |editor, save, new| editor.class.class_data_mut(save).gem_slot_mut(editor.slot_idx).set(new),
    capture class: ClassAccessor, slot_idx: usize
);

#[rustfmt::skip]
editor!(
    pub SkillEditor,
    Option<u16>,
    get |editor, save| editor.class.class_data(save).skill_slot(editor.slot_idx).get(),
    set |editor, save, new| editor.class.class_data_mut(save).skill_slot_mut(editor.slot_idx).set(new),
    capture class: ClassAccessor, slot_idx: usize
);

#[derive(Clone, Copy, PartialEq)]
pub enum ClassAccessor {
    Character { char: usize, class: usize },
    Formation { formation: usize, char: u16 },
}

#[function_component]
pub fn ClassEditor(props: &ClassEditorProps) -> Html {
    let data = use_context::<Data>().unwrap();

    let arts = data.game().characters.arts();
    let skills = data.game().characters.skills();
    let gem_categories = data.game().items.gem_categories();

    let art_mapper = Callback::from(art_to_id);
    let skill_mapper = Callback::from(skill_to_id);
    let gem_mapper = Callback::from(gem_category_to_id);

    let accessor = props.accessor;

    html! {
        <>
            <Tile classes={classes!("is-parent")}>
                {props.stats.then(|| html! {
                    <Tile>
                        <Field classes={classes!("mr-2")}>
                            <label class="label"><Text path="character_class_cp" /></label>
                            <Control>
                                <NumberInput<CpEditor> editor={CpEditor { class: accessor }} />
                            </Control>
                        </Field>
                        <Field classes={classes!("mr-2")}>
                            <label class="label"><Text path="character_class_unlock" /></label>
                            <Control>
                                <NumberInput<UnlockPointsEditor> editor={UnlockPointsEditor { class: accessor }} />
                            </Control>
                        </Field>
                        <Field>
                            <label class="label"><Text path="character_class_rank" /></label>
                            <Control>
                                <NumberInput<RankEditor> editor={RankEditor { class: accessor }} />
                            </Control>
                        </Field>
                    </Tile>
                })}
            </Tile>
            <Tile classes={classes!("is-parent")}>
                <Tile classes={classes!("is-vertical", "mr-2")}>
                    {for (0..CHARACTER_CLASS_ART_MAX).map(|i| html! {
                        <Field>
                            <label class="label"><Text path={"character_art"} args={vec![("id".into(), i.into())]} /></label>
                            <SlotInput<ArtEditor, Art, u16>
                                editor={ArtEditor {class: accessor, slot_idx: i}}
                                possible_values={arts}
                                id_mapper={art_mapper.clone()}
                            />
                        </Field>
                    })}
                </Tile>
                <Tile classes={classes!("is-vertical", "mr-2")}>
                    {for (0..CHARACTER_CLASS_SKILL_MAX-1).map(|i| html! {
                        <Field>
                            <label class="label"><Text path={"character_skill"} args={vec![("id".into(), i.into())]} /></label>
                            <SlotInput<SkillEditor, Skill, u16>
                                editor={SkillEditor {class: accessor, slot_idx: i}}
                                possible_values={skills}
                                id_mapper={skill_mapper.clone()}
                            />
                        </Field>
                    })}
                </Tile>
                <Tile classes={classes!("is-vertical", "mr-2")}>
                    {for (0..3).map(|i| html! {
                        <Field>
                            <label class="label"><Text path={"character_gem"} args={vec![("id".into(), i.into())]} /></label>
                            <SlotInput<GemEditor, GemCategory, u8>
                                editor={GemEditor {class: accessor, slot_idx: i}}
                                possible_values={gem_categories}
                                id_mapper={gem_mapper.clone()}
                            />
                        </Field>
                    })}
                </Tile>
                <Tile classes={classes!("is-vertical", "mr-2")}>
                    {for (0..CHARACTER_CLASS_ACCESSORY_MAX).map(|i| html! {
                        <Field>
                            <label class="label"><Text path={"character_accessory"} args={vec![("id".into(), i.into())]} /></label>
                            <AccessoryInput char={accessor} slot_idx={i} />
                        </Field>
                    })}
                </Tile>
            </Tile>
        </>
    }
}

pub fn art_to_id(art: &Art) -> Option<u16> {
    Some(art.id.try_into().unwrap())
}

pub fn skill_to_id(skill: &Skill) -> Option<u16> {
    Some(skill.id.try_into().unwrap())
}

fn gem_category_to_id(gem: &GemCategory) -> Option<u8> {
    gem.id.try_into().ok().and_then(|id: u8| id.checked_sub(1))
}

impl ClassAccessor {
    pub fn class_data<'s>(&self, save: &'s SaveData) -> &'s CharacterClass {
        match self {
            ClassAccessor::Character { char, class } => save.characters[*char].class_data(*class),
            ClassAccessor::Formation { formation, char } => {
                &save.party_formations[*formation]
                    .character(*char)
                    .unwrap()
                    .class
            }
        }
    }

    pub fn class_data_mut<'s>(&self, save: &'s mut SaveData) -> &'s mut CharacterClass {
        match self {
            ClassAccessor::Character { char, class } => {
                save.characters[*char].class_data_mut(*class)
            }
            ClassAccessor::Formation { formation, char } => {
                &mut save.party_formations[*formation].character_mut(*char).class
            }
        }
    }
}
