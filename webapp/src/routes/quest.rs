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
const ROWS_PER_PAGE: usize = 12;

#[function_component]
pub fn Quests() -> Html {
    let save = use_context::<SaveContext>().unwrap();
    let data = use_context::<Data>().unwrap();

    let quests = &data.game().quests;
    let is_dlc4 = save.get().get().save().is_dlc4();
    let start = quests.start(is_dlc4);
    let end = quests.end(is_dlc4);

    let page = use_state(|| 0);
    let page_organizer =
        PageOrganizer::<PAGES_PER_VIEW>::new(ROWS_PER_PAGE, *page, end + 1 - start);

    html! {
        <Container>
            <Tile>
                {for page_organizer.current_bounds.into_iter().map(|(s, e)| html! {
                    <Tile>
                        <TablePage start={start + s} end={start + e} />
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
