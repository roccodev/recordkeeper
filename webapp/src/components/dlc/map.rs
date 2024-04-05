use std::borrow::Cow;

use game_data::{
    dlc::map::{AchievementName, AchievementSearch, MapAchievementProgress},
    lang::{FilterEntry, TextEntry},
    LanguageData,
};
use ybc::{Table, Tile};
use yew::prelude::*;

use crate::{
    components::{
        edit::{Editor, EnumInput, FlagEditor},
        page::{PageControls, PageOrganizer},
    },
    data::Data,
    lang::Text,
    ToHtml,
};

#[derive(Properties, PartialEq)]
pub struct RegionProps {
    pub region: usize,
    pub category: u32,
}

#[derive(Properties, PartialEq)]
pub struct AchievementProps {
    region: usize,
    row: usize,
    column: usize,
    disabled: bool,
}

const PAGES_PER_VIEW: usize = 2;
const ROWS_PER_PAGE: usize = 12;

#[derive(PartialEq, Copy, Clone)]
pub struct AchievementStatusEditor(pub FlagEditor);

#[function_component]
pub fn RegionAchievements(props: &RegionProps) -> Html {
    let data = use_context::<Data>().unwrap();
    let page = use_state(|| 0);

    let achievements = data.game().dlc.map.achievements(props.region);
    let Some(row) = achievements.iter().position(|a| a.ty == props.category) else {
        return html!();
    };
    let achievement = &achievements[row];

    let page_organizer =
        PageOrganizer::<PAGES_PER_VIEW>::new(ROWS_PER_PAGE, *page, achievement.searches.len());

    html! {
        <>
            <Tile classes="mb-2">
                {for page_organizer.bounds().map(|(s, e)| html! {
                    <Tile classes="is-align-items-start">
                        <Table classes={classes!("is-fullwidth")}>
                            <thead>
                                <tr>
                                    <th><Text path="dlc4_map_ach_flag" /></th>
                                    <th><Text path="dlc4_map_ach_desc" /></th>
                                    <th><Text path="dlc4_map_ach_status" /></th>
                                </tr>
                            </thead>

                            <tbody>
                                {for (s..=e).map(|index| {
                                    html! {
                                        <Achievement
                                            region={props.region}
                                            row={row}
                                            column={index}
                                            disabled={false}
                                        />
                                    }
                                })}
                            </tbody>
                        </Table>
                    </Tile>
                })}
            </Tile>

            <PageControls<PAGES_PER_VIEW> organizer={page_organizer} state={page} />
        </>
    }
}

#[function_component]
fn Achievement(props: &AchievementProps) -> Html {
    let data = use_context::<Data>().unwrap();

    let achievements = data.game().dlc.map.achievements(props.region);
    let achievement = &achievements[props.row].searches[props.column];

    let flag_editor = AchievementStatusEditor(FlagEditor::from(achievement.flag));

    html! {
        <tr>
            <th>{achievement.flag.index}</th>
            <td>{achievement_lang(data.lang(), achievement)}</td>
            <td>
                <EnumInput<AchievementStatusEditor> editor={flag_editor} />
            </td>
        </tr>
    }
}

fn achievement_lang(lang: &LanguageData, achievement: &AchievementSearch) -> Html {
    let text = match &achievement.name {
        AchievementName::Enemy { name_id } => lang
            .enemies
            .enemies
            .get(*name_id)
            .map(TextEntry::text)
            .map(Into::into),
        AchievementName::Npc { name_id } => lang
            .npcs
            .get_npc_name(*name_id)
            .map(FilterEntry::text)
            .map(Into::into),
        AchievementName::Location { name_id } => lang
            .field
            .locations
            .get(*name_id)
            .map(FilterEntry::text)
            .map(Into::into),
        AchievementName::ComSpot { name_id } => lang
            .field
            .com_spots
            .get(*name_id)
            .map(TextEntry::text)
            .map(Into::into),
        AchievementName::Architecture { ty, x, y, z } => {
            return html! {
                <>
                    <Text path={format!("dlc4_map_arch_{}", ty.lang_id())} />
                    {": "}{format!("({x:.0}, {y:.0}, {z:.0})")}
                </>
            }
        }
        AchievementName::Unknown { x, y, z } => Some(format!("({x:.0}, {y:.0}, {z:.0})").into()),
    }
    .unwrap_or_else(|| Cow::Owned(format!("{:?}", achievement.name)));
    html! (<>{text}</>)
}

impl Editor for AchievementStatusEditor {
    type Target = MapAchievementProgress;

    fn get(&self, save: &recordkeeper::SaveData) -> Self::Target {
        MapAchievementProgress::from_repr(self.0.get(save) as usize).expect("unknown status")
    }

    fn set(&self, save: &mut recordkeeper::SaveData, new: Self::Target) {
        self.0.set(save, new as u32);
    }
}

impl ToHtml for MapAchievementProgress {
    fn to_html(&self) -> Html {
        let id = match self {
            MapAchievementProgress::Hidden => "hidden",
            MapAchievementProgress::Visible => "visible",
            MapAchievementProgress::Completed => "complete",
        };
        html!(<Text path={format!("dlc4_map_status_{id}")} />)
    }
}
