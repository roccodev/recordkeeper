use game_data::character::{Art, Class, Skill};
use recordkeeper::character::{CHARACTER_CLASS_ART_MAX, CHARACTER_CLASS_SKILL_MAX};
use ybc::{Control, Field, Tile};
use yew::prelude::*;

use crate::{
    components::{
        character::{slot::SlotInput, Selector},
        edit::{editor, NumberInput},
    },
    data::Data,
    lang::Text,
};

use super::CharacterProps;

#[derive(Properties, PartialEq)]
pub struct ClassProps {
    pub char_id: usize,
    pub class_id: usize,
}

#[rustfmt::skip]
editor!(
    CpEditor,
    u32,
    get |editor, save| save.characters[editor.char_idx].class_data(editor.class_id).cp,
    set |editor, save, new| save.characters[editor.char_idx].class_data_mut(editor.class_id).cp = new,
    capture char_idx: usize, class_id: usize
);

#[rustfmt::skip]
editor!(
    UnlockPointsEditor,
    u16,
    get |editor, save| save.characters[editor.char_idx].class_data(editor.class_id).unlock_points,
    set |editor, save, new| save.characters[editor.char_idx].class_data_mut(editor.class_id).unlock_points = new,
    capture char_idx: usize, class_id: usize
);

#[rustfmt::skip]
editor!(
    RankEditor,
    u8,
    get |editor, save| save.characters[editor.char_idx].class_data(editor.class_id).level,
    set |editor, save, new| save.characters[editor.char_idx].class_data_mut(editor.class_id).level = new,
    assert |_, v| (1..=20).contains(v).then_some(()).ok_or_else(String::new),
    capture char_idx: usize, class_id: usize
);

#[rustfmt::skip]
editor!(
    pub ArtEditor,
    Option<u16>,
    get |editor, save| save.characters[editor.char_idx].class_data(editor.class_id).art_slot(editor.slot_idx).get(),
    set |editor, save, new| save.characters[editor.char_idx].class_data_mut(editor.class_id).art_slot_mut(editor.slot_idx).set(new),
    capture char_idx: usize, class_id: usize, slot_idx: usize
);

#[rustfmt::skip]
editor!(
    pub GemEditor,
    Option<u8>,
    get |editor, save| save.characters[editor.char_idx].class_data(editor.class_id).gem_slot(editor.slot_idx).get(),
    set |editor, save, new| save.characters[editor.char_idx].class_data_mut(editor.class_id).gem_slot_mut(editor.slot_idx).set(new),
    capture char_idx: usize, class_id: usize, slot_idx: usize
);

#[rustfmt::skip]
editor!(
    pub SkillEditor,
    Option<u16>,
    get |editor, save| save.characters[editor.char_idx].class_data(editor.class_id).skill_slot(editor.slot_idx).get(),
    set |editor, save, new| save.characters[editor.char_idx].class_data_mut(editor.class_id).skill_slot_mut(editor.slot_idx).set(new),
    capture char_idx: usize, class_id: usize, slot_idx: usize
);

#[function_component]
pub fn ClassEditor(props: &CharacterProps) -> Html {
    let data = use_context::<Data>().unwrap();
    let class_id = use_state(|| 1); // TODO use selected class
    let char_idx = props.char_id.checked_sub(1).unwrap();

    let arts = data.game().characters.arts();
    let skills = data.game().characters.skills();

    let art_mapper = Callback::from(art_to_id);
    let skill_mapper = Callback::from(skill_to_id);

    html! {
        <>
            <Tile classes={classes!("is-parent")}>
                <Tile>
                    <Field>
                        <label class="label"><Text path="character_class" /></label>
                        <Control>
                            <Selector<Class> state={class_id.clone()} values={data.game().characters.classes()} />
                        </Control>
                    </Field>
                </Tile>
                <Tile>
                    <Field>
                        <label class="label"><Text path="character_class_cp" /></label>
                        <Control>
                            <NumberInput<CpEditor> editor={CpEditor { char_idx: char_idx, class_id: *class_id }} />
                        </Control>
                    </Field>
                    <Field>
                        <label class="label"><Text path="character_class_unlock" /></label>
                        <Control>
                            <NumberInput<UnlockPointsEditor> editor={UnlockPointsEditor { char_idx: char_idx, class_id: *class_id }} />
                        </Control>
                    </Field>
                    <Field>
                        <label class="label"><Text path="character_class_rank" /></label>
                        <Control>
                            <NumberInput<RankEditor> editor={RankEditor { char_idx: char_idx, class_id: *class_id }} />
                        </Control>
                    </Field>
                </Tile>
            </Tile>
            <Tile classes={classes!("is-parent")}>
                <Tile classes={classes!("is-child")}>
                    {for (0..CHARACTER_CLASS_ART_MAX).map(|i| html! {
                        <Field>
                            <label class="label"><Text path={"character_art"} args={vec![("id".into(), i.into())]} /></label>
                            <SlotInput<ArtEditor, Art>
                                editor={ArtEditor {char_idx: char_idx, class_id: *class_id, slot_idx: i}}
                                possible_values={arts}
                                id_mapper={art_mapper.clone()}
                            />
                        </Field>
                    })}
                </Tile>
                <Tile classes={classes!("is-child")}>
                    {for (0..CHARACTER_CLASS_SKILL_MAX).map(|i| html! {
                        <Field>
                            <label class="label"><Text path={"character_skill"} args={vec![("id".into(), i.into())]} /></label>
                            <SlotInput<SkillEditor, Skill>
                                editor={SkillEditor {char_idx: char_idx, class_id: *class_id, slot_idx: i}}
                                possible_values={skills}
                                id_mapper={skill_mapper.clone()}
                            />
                        </Field>
                    })}
                </Tile>
                <Tile classes={classes!("is-child")}>
                    {"Gems"}
                </Tile>
                <Tile classes={classes!("is-child")}>
                    {"Accessories"}
                </Tile>
            </Tile>
        </>
    }
}

fn art_to_id(art: &Art) -> Option<u16> {
    Some(art.id.try_into().unwrap())
}

fn skill_to_id(skill: &Skill) -> Option<u16> {
    Some(skill.id.try_into().unwrap())
}
