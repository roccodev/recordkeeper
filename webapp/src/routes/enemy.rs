use recordkeeper::enemy::Difficulty;
use strum::IntoEnumIterator;
use ybc::{Container, Control, Field, Select, Table, Tile};
use yew::prelude::*;

use crate::{
    components::{
        enemy::UniqueMonsterRow,
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
    pub difficulty: Difficulty,
}

#[derive(Properties, PartialEq)]
struct DifficultyProps {
    state: UseStateHandle<Difficulty>,
}

#[function_component]
pub fn UniqueMonsters() -> Html {
    let data = use_context::<Data>().unwrap();

    let enemies = &data.game().enemies.unique_monsters;

    let page = use_state(|| 0);
    let difficulty = use_state(|| Difficulty::Normal);
    let page_organizer = PageOrganizer::<PAGES_PER_VIEW>::new(ROWS_PER_PAGE, *page, enemies.len());

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
                    <th><Text path="enemy_id" /></th>
                    <th><Text path="enemy_name" /></th>
                    <th><Text path="enemy_seen" /></th>
                    <th><Text path="enemy_defeat" /></th>
                    <th><Text path="enemy_rematch" /></th>
                    <th><Text path="enemy_time" /></th>
                    <th><Text path="enemy_rematch_time" /></th>
                </tr>
            </thead>

            <tbody>
                {for (props.start..=props.end).map(|index| {
                    html!(<UniqueMonsterRow id={index} difficulty={props.difficulty} />)
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
            let diff = Difficulty::from_repr(diff.parse().unwrap()).unwrap();
            state.set(diff);
        })
    };

    html! {
        <Select name="difficulty" update={update} value={(*props.state as u32).to_string()}>
            {for Difficulty::iter().map(|difficulty| {
                html! {
                    <option value={(difficulty as u32).to_string()} selected={*props.state == difficulty}>
                        {difficulty.to_html()}
                    </option>
                }
            })}
        </Select>
    }
}
