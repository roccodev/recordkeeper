use std::{fmt::Display, str::FromStr};

use game_data::{
    character::SoulHack,
    enemy::SoulLearnable,
    lang::{Id, Nameable},
};
use recordkeeper::{enemy::Achievement, flags::FlagType};
use ybc::{Button, Container, Control, Field, Table};
use yew::prelude::*;

use crate::{
    components::edit::{editor, CheckboxInput, Editor, FlagEditor, StringInput, ToBool},
    data::Data,
    lang::Text,
    save::SaveContext,
};

#[derive(Properties, PartialEq)]
pub struct SoulHackProps<T: PartialEq + 'static> {
    pub values: &'static [T],
}

#[derive(Properties, PartialEq)]
struct SoulProps<T: PartialEq + 'static> {
    value: &'static T,
    soul_hack: SoulHack,
}

#[rustfmt::skip]
editor!(
    AchievementEditor,
    ParseAchievement,
    get |editor, save| ParseAchievement(save.soul_hack_achievements.get(editor.id)),
    set |editor, save, new| save.soul_hack_achievements.set(editor.id, new.0),
    capture id: usize
);

#[derive(Clone, PartialEq, Eq)]
struct ParseAchievement(Achievement);

#[function_component]
pub fn SoulHackTable<T>(props: &SoulHackProps<T>) -> Html
where
    T: PartialEq + SoulLearnable + Id + Nameable + 'static,
{
    html! {
        <Container>
            <Table classes={classes!("is-fullwidth")}>
                <thead>
                    <tr>
                        <th><Text path="enemy_soul_hack_id" /></th>
                        <th><Text path="enemy_soul_hack_name" /></th>
                        <th><Text path="enemy_soul_hack_unlocked" /></th>
                        <th><Text path="enemy_soul_hack_progress" /></th>
                    </tr>
                </thead>

                <tbody>
                    {for props.values.iter().filter_map(|e| e.get_soul_hack().map(|s| (e, s))).map(|(entry, soul_hack)| {
                        html!(<SoulHackRow<T> soul_hack={soul_hack} value={entry} />)
                    })}
                </tbody>
            </Table>
        </Container>
    }
}

#[function_component]
fn SoulHackRow<T>(props: &SoulProps<T>) -> Html
where
    T: PartialEq + Id + Nameable + 'static,
{
    let data = use_context::<Data>().unwrap();
    let save_context = use_context::<SaveContext>().unwrap();

    let unlocked = ToBool(FlagEditor {
        flag_index: props.soul_hack.status_flag.get(),
        flag_type: FlagType::TwoBits,
    });

    let progress_editor = AchievementEditor {
        id: props.soul_hack.achievement_flag.get(),
    };

    let is_upgraded =
        progress_editor.get(save_context.get().get_save()).0 == Achievement::Completed;

    let upgrade_callback = {
        let save_context = save_context.clone();
        Callback::from(move |_: MouseEvent| {
            save_context.edit(move |save| {
                if is_upgraded {
                    // Reset progress
                    progress_editor.set(save, Achievement::InProgress(0).into());
                } else {
                    // Set completed
                    progress_editor.set(save, Achievement::Completed.into());
                }
            })
        })
    };

    let upgrade_classes = if is_upgraded {
        classes!("button", "is-primary")
    } else {
        classes!("button")
    };

    html! {
        <tr>
            <th>{props.value.id()}</th>
            <td>{props.value.get_name_str(data.lang())}</td>
            <td><CheckboxInput<ToBool<FlagEditor>> editor={unlocked} /></td>
            <td>
                <Field classes={"has-addons"}>
                    <Control>
                        <StringInput<ParseAchievement, AchievementEditor>
                            input_type="number"
                            editor={progress_editor}
                            disabled={Callback::from(|ach: ParseAchievement| ach.0 == Achievement::Completed)}
                        />
                    </Control>
                    <Control>
                        <Button onclick={upgrade_callback} classes={upgrade_classes}>
                            <Text path="enemy_soul_hack_upgraded" />
                        </Button>
                    </Control>
                </Field>
            </td>
        </tr>
    }
}

impl FromStr for ParseAchievement {
    type Err = <u32 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        u32::from_str(s).map(|i| ParseAchievement(Achievement::from(i)))
    }
}

impl Display for ParseAchievement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Achievement::Completed => f.write_str("--"),
            Achievement::InProgress(p) => p.fmt(f),
        }
    }
}

impl From<Achievement> for ParseAchievement {
    fn from(value: Achievement) -> Self {
        Self(value)
    }
}
