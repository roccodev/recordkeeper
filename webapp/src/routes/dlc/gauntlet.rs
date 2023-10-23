use recordkeeper::dlc::ChallengeDifficulty;
use strum::{EnumIter, IntoEnumIterator};
use ybc::{Container, Control, Field, Table, Tabs, Tile};
use yew::prelude::*;

use crate::{
    components::{
        dlc::gauntlet::GauntletRow,
        page::{PageControls, PageOrganizer},
    },
    data::Data,
    lang::Text,
};

use super::challenge::{DifficultySelector, TableProps};

const PAGES_PER_VIEW: usize = 1;
const ROWS_PER_PAGE: usize = 10;

#[derive(Clone, PartialEq, Copy, EnumIter)]
pub enum GauntletTab {
    Records,
    Emblems,
    SaveState,
}

#[function_component]
pub fn GauntletPage() -> Html {
    let tab = use_state(|| GauntletTab::Records);
    let data = use_context::<Data>().unwrap();

    let update_tab = |t| {
        let tab_state = tab.clone();
        Callback::from(move |_: MouseEvent| {
            tab_state.set(t);
        })
    };

    html! {
        <>
            <Tabs classes={classes!("is-boxed", "is-centered")}>
                {for GauntletTab::iter().map(|t| {
                    let classes = if t == *tab { classes!("is-active") } else { classes!() };
                    html!(<li class={classes}><a onclick={update_tab(t)}><Text path={t.lang()} /></a></li>)
                })}
            </Tabs>

            {match *tab {
                GauntletTab::Records => html!(<Records />),
                GauntletTab::Emblems => html!(),
                GauntletTab::SaveState => html!(),
            }}
        </>
    }
}

#[function_component]
pub fn Records() -> Html {
    let data = use_context::<Data>().unwrap();

    let gauntlets = &data.game().dlc.challenge.gauntlets;

    let page = use_state(|| 0);
    let difficulty = use_state(|| ChallengeDifficulty::Normal);
    let page_organizer =
        PageOrganizer::<PAGES_PER_VIEW>::new(ROWS_PER_PAGE, *page, gauntlets.len());

    html! {
        <Container>
            <Field>
                <label class="label"><Text path="difficulty" /></label>
                <Control>
                    <DifficultySelector state={difficulty.clone()} />
                </Control>
            </Field>

            <Tile>
                {for page_organizer.current_bounds.into_iter().map(|(s, e)| html! {
                    <Tile>
                        <RecordPage start={1 + s} end={1 + e} difficulty={*difficulty} />
                    </Tile>
                })}
            </Tile>

            <PageControls<PAGES_PER_VIEW> organizer={page_organizer} state={page} />
        </Container>
    }
}

#[function_component]
fn RecordPage(props: &TableProps) -> Html {
    html! {
        <Table classes={classes!("is-fullwidth")}>
            <thead>
                <tr>
                    <th><Text path="challenge_id" /></th>
                    <th><Text path="challenge_name" /></th>
                    <th><Text path="challenge_rank" /></th>
                    <th><Text path="gauntlet_score" /></th>
                    <th><Text path="gauntlet_stage" /></th>
                    <th><Text path="challenge_time" /></th>
                    <th><Text path="gauntlet_play_count" /></th>
                    <th><Text path="challenge_clear_count" /></th>
                    <th><Text path="challenge_clear" /></th>
                    <th><Text path="challenge_new" /></th>
                    <th><Text path="challenge_bonus" /></th>
                    <th><Text path="gauntlet_reward_b" /></th>
                    <th><Text path="gauntlet_reward_a" /></th>
                </tr>
            </thead>

            <tbody>
                {for (props.start..=props.end).map(|index| {
                    html!(<GauntletRow id={index} difficulty={props.difficulty} />)
                })}
            </tbody>
        </Table>
    }
}

impl GauntletTab {
    fn lang(&self) -> String {
        let id = match self {
            Self::Records => "records",
            Self::Emblems => "emblems",
            Self::SaveState => "state",
        };
        format!("gauntlet_tab_{id}")
    }
}
