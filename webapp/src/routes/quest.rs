use crate::components::page::{PageControls, PageOrganizer};
use crate::components::quest::QuestRow;
use crate::data::Data;
use crate::lang::Text;
use crate::save::SaveContext;
use ybc::{Container, Table, Tile};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct TableProps {
    pub start: usize,
    pub end: usize,
}

const PAGES_PER_VIEW: usize = 2;
const ROWS_PER_PAGE: usize = 20;

#[function_component]
pub fn Quests() -> Html {
    let save = use_context::<SaveContext>().unwrap();
    let data = use_context::<Data>().unwrap();

    let page = use_state(|| 0);
    let page_organizer =
        PageOrganizer::<PAGES_PER_VIEW>::new(ROWS_PER_PAGE, *page, data.game().quests.len() - 1);

    html! {
        <Container>
            <Tile>
                {for page_organizer.current_bounds.into_iter().map(|(start, end)| html! {
                    <Tile>
                        <TablePage start={start + 1} end={end} />
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
                    <th><Text path="quest_id" /></th>
                    <th><Text path="quest_name" /></th>
                    <th><Text path="quest_status" /></th>
                    <th><Text path="quest_actions" /></th>
                </tr>
            </thead>

            <tbody>
                {for (props.start..=props.end).map(|index| {
                    html!(<QuestRow id={index} />)
                })}
            </tbody>
        </Table>
    }
}
