use game_data::character::{Art, Skill};
use recordkeeper::{dlc::ChallengeDifficulty, enemy::Difficulty};
use strum::{EnumIter, IntoEnumIterator};
use ybc::{Container, Control, Field, Select, Table, Tabs, Tile};
use yew::prelude::*;

use crate::{
    components::{
        dlc::challenge::ChallengeRow,
        page::{PageControls, PageOrganizer},
    },
    data::Data,
    lang::Text,
    ToHtml,
};

const PAGES_PER_VIEW: usize = 1;
const ROWS_PER_PAGE: usize = 10;

#[derive(Properties, PartialEq)]
struct TableProps {
    pub start: usize,
    pub end: usize,
    pub difficulty: ChallengeDifficulty,
}

#[derive(Properties, PartialEq)]
struct DifficultyProps {
    state: UseStateHandle<ChallengeDifficulty>,
}

#[function_component]
pub fn ChallengePage() -> Html {
    let data = use_context::<Data>().unwrap();
    let challenges = &data.game().dlc.challenge.challenges;

    let page = use_state(|| 0);
    let difficulty = use_state(|| ChallengeDifficulty::Normal);
    let page_organizer =
        PageOrganizer::<PAGES_PER_VIEW>::new(ROWS_PER_PAGE, *page, challenges.len());

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
                        <TablePage start={1 + s} end={1 + e} difficulty={*difficulty} />
                    </Tile>
                })}
            </Tile>

            <PageControls<PAGES_PER_VIEW> organizer={page_organizer} state={page} />
        </Container>
    }
}

#[function_component]
fn TablePage(props: &TableProps) -> Html {
    html! {
        <Table classes={classes!("is-fullwidth")}>
            <thead>
                <tr>
                    <th><Text path="challenge_id" /></th>
                    <th><Text path="challenge_name" /></th>
                    <th><Text path="challenge_bonus" /></th>
                    <th><Text path="challenge_rank" /></th>
                    <th><Text path="challenge_time" /></th>
                    <th><Text path="challenge_clear_count" /></th>
                </tr>
            </thead>

            <tbody>
                {for (props.start..=props.end).map(|index| {
                    html!(<ChallengeRow id={index} difficulty={props.difficulty} />)
                })}
            </tbody>
        </Table>
    }
}

#[function_component]
fn DifficultySelector(props: &DifficultyProps) -> Html {
    let update = {
        let state = props.state.clone();
        Callback::from(move |diff: String| {
            let diff = ChallengeDifficulty::from_repr(diff.parse().unwrap()).unwrap();
            state.set(diff);
        })
    };

    html! {
        <Select name="difficulty" update={update} value={(*props.state as u32).to_string()}>
            {for ChallengeDifficulty::iter().map(|difficulty| {
                html! {
                    <option value={(difficulty as u32).to_string()} selected={*props.state == difficulty}>
                        {Difficulty::from(difficulty).to_html()}
                    </option>
                }
            })}
        </Select>
    }
}
