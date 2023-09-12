use game_data::character::{Art, Skill};
use recordkeeper::{
    character::{OUROBOROS_ART_MAX, OUROBOROS_SKILL_MAX},
    flags::FlagType,
};
use ybc::{Control, Field, Notification, Tile};
use yew::prelude::*;

use crate::{
    components::{
        character::{
            class::{art_to_id, skill_to_id},
            slot::SlotInput,
        },
        edit::{CheckboxInput, FlagEditor, NumberInput, ToBool},
    },
    data::Data,
    lang::Text,
};

use super::{character::CharacterProps, edit::editor};

#[rustfmt::skip]
editor!(
    SpEditor,
    u32,
    get |editor, save| save.ouroboros[editor.char_idx].sp,
    set |editor, save, new_value| save.ouroboros[editor.char_idx].sp = new_value,
    capture char_idx: usize
);

#[rustfmt::skip]
editor!(
    pub ArtEditor,
    Option<u16>,
    get |editor, save| save.ouroboros[editor.char_idx].art_slot(editor.slot_idx).get(),
    set |editor, save, new| save.ouroboros[editor.char_idx].art_slot_mut(editor.slot_idx).set(new),
    capture char_idx: usize, slot_idx: usize
);

#[rustfmt::skip]
editor!(
    pub SkillEditor,
    Option<u16>,
    get |editor, save| save.ouroboros[editor.char_idx].linked_skill_slot(editor.slot_idx).get(),
    set |editor, save, new| save.ouroboros[editor.char_idx].linked_skill_slot_mut(editor.slot_idx).set(new),
    capture char_idx: usize, slot_idx: usize
);

#[function_component]
pub fn OuroborosEditor(props: &CharacterProps) -> Html {
    let data = use_context::<Data>().unwrap();
    let ouroboros = data.game().ouroboros.get(props.char_id).unwrap();

    let arts = data.game().characters.arts();
    let skills = data.game().characters.skills();
    let art_mapper = Callback::from(art_to_id);
    let skill_mapper = Callback::from(skill_to_id);

    let char_idx = props.char_id.checked_sub(1).unwrap();
    let share_slot_flag = ToBool(FlagEditor {
        flag_type: FlagType::Bit,
        flag_index: ouroboros.share_slot_flag,
    });

    html! {
        <>
            <Tile classes={classes!("notification", "is-vertical")}>
                <Tile classes={classes!("is-align-items-center")}>
                    <Field classes={classes!("mr-2")}>
                        <label class="label"><Text path="ouroboros_sp" /></label>
                        <Control>
                            <NumberInput<SpEditor> editor={SpEditor { char_idx }} />
                        </Control>
                    </Field>
                    <Field classes={classes!("mr-2")}>
                        <Control>
                            <CheckboxInput<ToBool<FlagEditor>> editor={share_slot_flag}>
                                {" "}<Text path="ouroboros_share_slot" />
                            </CheckboxInput<ToBool<FlagEditor>>>
                        </Control>
                    </Field>
                </Tile>
                <Tile>
                    {for (0..OUROBOROS_SKILL_MAX).map(|i| html! {
                        <Field classes={classes!("mr-2")}>
                            <label class="label"><Text path={"ouroboros_skill"} args={vec![("id".into(), i.into())]} /></label>
                            <SlotInput<SkillEditor, Skill, u16>
                                editor={SkillEditor {char_idx: char_idx, slot_idx: i}}
                                possible_values={skills}
                                id_mapper={skill_mapper.clone()}
                            />
                        </Field>
                    })}
                </Tile>
                <Tile>
                    {for (0..OUROBOROS_ART_MAX).map(|i| html! {
                        <Field classes={classes!("mr-2")}>
                            <label class="label"><Text path={"ouroboros_art"} args={vec![("id".into(), i.into())]} /></label>
                            <SlotInput<ArtEditor, Art, u16>
                                editor={ArtEditor {char_idx: char_idx, slot_idx: i}}
                                possible_values={arts}
                                id_mapper={art_mapper.clone()}
                            />
                        </Field>
                    })}
                </Tile>
            </Tile>
            <Notification>
                {"Skill Tree"}
            </Notification>
        </>
    }
}
