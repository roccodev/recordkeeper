use std::borrow::Cow;

use game_data::{
    dlc::map::{AchievementName, AchievementSearch},
    lang::{FilterEntry, Nameable, TextEntry},
    LanguageData,
};
use ybc::{Button, Buttons, Field, Notification, Tile};
use yew::prelude::*;

use crate::{
    components::edit::{Editor, FlagEditor},
    data::Data,
    lang::Text,
    save::SaveContext,
};

#[derive(Properties, PartialEq)]
pub struct RegionProps {
    region: usize,
}

#[derive(Properties, PartialEq)]
pub struct AchievementProps {
    region: usize,
    row: usize,
    column: usize,
    disabled: bool,
}

#[function_component]
pub fn MapPage() -> Html {
    let data = use_context::<Data>().unwrap();

    html! {
        <RegionAchievements region={3} />
    }
}

#[function_component]
pub fn RegionAchievements(props: &RegionProps) -> Html {
    let data = use_context::<Data>().unwrap();

    let achievements = data.game().dlc.map.achievements(props.region);

    html! {
        <>
            {for achievements.iter().enumerate().map(|(r, ach)| html! {
                <>
                    <span>{ach.get_name_str(data.lang())}</span>
                    <Buttons classes={classes!("has-addons")}>
                        {for (0..ach.searches.len())
                            .map(|c| html!(<Achievement region={props.region} row={r} column={c} disabled={false} />))
                        }
                    </Buttons>
                </>
            })}
        </>
    }
}

#[function_component]
fn Achievement(props: &AchievementProps) -> Html {
    let data = use_context::<Data>().unwrap();
    let save_context = use_context::<SaveContext>().unwrap();
    let save = save_context.get();

    let achievements = data.game().dlc.map.achievements(props.region);
    let achievement = &achievements[props.row].searches[props.column];

    let flag_editor = FlagEditor::from(achievement.flag);
    let flag_value = flag_editor.get(save.get_save());

    // Same tri-state button as quest purposes
    let button_class = classes!(
        "button",
        match u8::try_from(flag_value).unwrap() {
            0 => "",           // not started
            1 => "is-dark",    // in progress
            2 => "is-primary", // completed
            _ => unreachable!(),
        }
    );

    let save_context = save_context.clone();
    let callback = Callback::from(move |_: MouseEvent| {
        save_context.edit(move |save| {
            flag_editor.set(save, if flag_value == 0 { u8::MAX.into() } else { 0 })
        })
    });

    html! {
        <Button classes={button_class} onclick={callback}>
            <span>
                {achievement_lang(data.lang(), achievement)}
            </span>
        </Button>
    }
}

fn achievement_lang(lang: &LanguageData, achievement: &AchievementSearch) -> Html {
    let text = match &achievement.name {
        AchievementName::Enemy { name_id } => lang
            .enemies
            .enemies
            .get(*name_id as usize)
            .map(TextEntry::text)
            .map(Into::into),
        AchievementName::Location { name_id } => lang
            .field
            .locations
            .get(*name_id as usize)
            .map(FilterEntry::text)
            .map(Into::into),
        AchievementName::ComSpot { name_id } => lang
            .field
            .com_spots
            .get(*name_id as usize)
            .map(TextEntry::text)
            .map(Into::into),
        AchievementName::Unknown { x, y, z } => Some(format!("({x:.0}, {y:.0}, {z:.0})").into()),
        _ => None,
    }
    .unwrap_or_else(|| Cow::Owned(format!("{:?}", achievement.name)));
    html! (<>{text}</>)
}
