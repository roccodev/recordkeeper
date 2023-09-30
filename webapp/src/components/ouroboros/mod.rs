use game_data::{
    character::{Art, Skill},
    ouroboros::Ouroboros,
    GameData,
};
use recordkeeper::{
    character::{OUROBOROS_ART_MAX, OUROBOROS_SKILL_MAX},
    flags::FlagType,
    SaveData,
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
        ouroboros::tree::OuroTree,
    },
    data::Data,
    lang::Text,
};

use super::{
    character::{
        formation::{FormationOuroArt, FormationOuroSkill},
        CharacterProps,
    },
    edit::{editor, Editor},
};

mod tree;

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
    pub SaveArt,
    Option<u16>,
    get |editor, save| save.ouroboros[editor.char_idx].art_slot(editor.slot_idx).get(),
    set |editor, save, new| save.ouroboros[editor.char_idx].art_slot_mut(editor.slot_idx).set(new),
    capture char_idx: usize, slot_idx: usize
);

#[rustfmt::skip]
editor!(
    pub SaveSkill,
    Option<u16>,
    get |editor, save| save.ouroboros[editor.char_idx].linked_skill_slot(editor.slot_idx).get(),
    set |editor, save, new| save.ouroboros[editor.char_idx].linked_skill_slot_mut(editor.slot_idx).set(new),
    capture char_idx: usize, slot_idx: usize
);

#[derive(Clone, Copy, PartialEq)]
pub enum ArtEditor {
    Save(SaveArt),
    Formation(FormationOuroArt),
}

#[derive(Clone, Copy, PartialEq)]
pub enum SkillEditor {
    Save(SaveSkill),
    Formation(FormationOuroSkill),
}

/// Configuration for the ouroboros GUI editor.
///
/// Internal editors can be customized to edit other parts of the save file, while
/// still retaining the same graphical interface.
#[derive(PartialEq, Clone)]
pub struct OuroEditorConfig {
    pub sp: bool,
    pub flags: bool,
    pub tree: bool,
    pub art: Callback<usize, ArtEditor>,
    pub skill: Callback<usize, SkillEditor>,
}

#[derive(PartialEq, Properties)]
pub struct OuroborosProps {
    pub char_id: usize,
    pub config: OuroEditorConfig,
}

#[function_component]
pub fn OuroborosEditor(props: &OuroborosProps) -> Html {
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
                {if props.config.flags {
                    html! {
                        <Tile classes={classes!("is-align-items-center")}>
                            <Field classes={classes!("mr-2", "is-grouped", "is-grouped-multiline")}>
                                <Control>
                                    <CheckboxInput<ToBool<FlagEditor>> editor={ToBool(get_enable_flag(data.game(), ouroboros))}>
                                        {" "}<Text path="ouroboros_enable" />
                                    </CheckboxInput<ToBool<FlagEditor>>>
                                </Control>
                                <Control>
                                    <CheckboxInput<ToBool<FlagEditor>> editor={share_slot_flag}>
                                        {" "}<Text path="ouroboros_share_slot" />
                                    </CheckboxInput<ToBool<FlagEditor>>>
                                </Control>
                            </Field>
                        </Tile>
                    }
                } else { html!() }}
                {if props.config.sp {
                    html! {
                        <Tile>
                            <Field classes={classes!("mr-2")}>
                                <label class="label"><Text path="ouroboros_sp" /></label>
                                <Control>
                                    <NumberInput<SpEditor> editor={SpEditor { char_idx }} />
                                </Control>
                            </Field>
                        </Tile>
                }} else { html!() }}
                <Tile>
                    {for (0..OUROBOROS_SKILL_MAX).map(|i| html! {
                        <Field classes={classes!("mr-2")}>
                            <label class="label"><Text path={"ouroboros_skill"} args={vec![("id".into(), i.into())]} /></label>
                            <SlotInput<SkillEditor, Skill, u16>
                                editor={props.config.skill.emit(i)}
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
                                editor={props.config.art.emit(i)}
                                possible_values={arts}
                                id_mapper={art_mapper.clone()}
                            />
                        </Field>
                    })}
                </Tile>
            </Tile>
            {if props.config.tree {
                html! {
                    <Notification>
                        <OuroTree ouroboros={ouroboros} />
                    </Notification>
                }
            } else { html!() }}
        </>
    }
}

fn get_enable_flag(data: &GameData, ouroboros: &Ouroboros) -> FlagEditor {
    if ouroboros.id == 1 {
        return data.manual.flags.ouro_enable_noah.into();
    }
    let flag = data.manual.flags.ouro_enable;
    return FlagEditor {
        flag_type: FlagType::from_bits(flag.bits),
        flag_index: flag.index + ouroboros.id - 2,
    };
}

impl Editor for ArtEditor {
    type Target = Option<u16>;

    fn get(&self, save: &SaveData) -> Self::Target {
        match self {
            Self::Save(e) => e.get(save),
            Self::Formation(e) => e.get(save),
        }
    }

    fn set(&self, save: &mut SaveData, new: Self::Target) {
        match self {
            Self::Save(e) => e.set(save, new),
            Self::Formation(e) => e.set(save, new),
        }
    }
}

impl Editor for SkillEditor {
    type Target = Option<u16>;

    fn get(&self, save: &SaveData) -> Self::Target {
        match self {
            Self::Save(e) => e.get(save),
            Self::Formation(e) => e.get(save),
        }
    }

    fn set(&self, save: &mut SaveData, new: Self::Target) {
        match self {
            Self::Save(e) => e.set(save, new),
            Self::Formation(e) => e.set(save, new),
        }
    }
}
