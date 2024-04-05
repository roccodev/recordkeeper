use game_data::{dlc::community::CommunityStatus, lang::Nameable, npc::Npc};
use recordkeeper::{chrono::ChronologicalOrder, dlc::CommunityChrono, SaveData};
use ybc::{Button, Control, Field, Table, Tile};
use yew::prelude::*;

use crate::{
    components::{
        dlc::community::CommunityOrderModal,
        edit::{Editor, EnumInput, FlagEditor},
        page::{PageControls, PageOrganizer},
    },
    data::Data,
    lang::Text,
    ToHtml,
};

#[derive(Properties, PartialEq)]
pub struct NpcProps {
    npc: &'static Npc,
}

#[derive(Clone, Copy, PartialEq)]
struct StatusEditor {
    progress: FlagEditor,
    order: FlagEditor,
}

const PAGES_PER_VIEW: usize = 2;
const ROWS_PER_PAGE: usize = 12;

#[function_component]
pub fn CommunityPage() -> Html {
    let data = use_context::<Data>().unwrap();
    let order_open = use_state(|| false);
    let order_change = |v: bool| {
        let order = order_open.clone();
        Callback::from(move |_| {
            order.set(v);
        })
    };

    let page = use_state(|| 0);
    let sorted = data
        .lang()
        .dlc
        .community
        .npc_sort
        .list(&data.game().dlc.community.npcs);
    let page_organizer = PageOrganizer::<PAGES_PER_VIEW>::new(ROWS_PER_PAGE, *page, sorted.len());

    html! {
        <>
            <CommunityOrderModal open={*order_open} close_callback={order_change(false)} />
            <Field classes={classes!("is-grouped", "is-align-items-end")}>
                <Control classes="is-flex-grow-1">
                </Control>
                <Control>
                    <Button onclick={order_change(true).reform(|_: MouseEvent| ())}>
                        <Text path="dlc4_comm_order" />
                    </Button>
                </Control>
            </Field>
            <Tile classes="mb-2">
                {for page_organizer.bounds().map(|(s, e)| html! {
                    <Tile classes="is-align-items-start">
                        <Table classes={classes!("is-fullwidth")}>
                            <thead>
                                <tr>
                                    <th><Text path="dlc4_comm_npc" /></th>
                                    <th><Text path="dlc4_comm_status" /></th>
                                </tr>
                            </thead>
                            <tbody>
                                {for (s..=e).map(|index| {
                                    html!(<NpcRow npc={sorted[index]}/>)
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
fn NpcRow(props: &NpcProps) -> Html {
    let data = use_context::<Data>().unwrap();
    let challenges = &data.game().dlc.community.challenge(props.npc.id);
    let editor = StatusEditor {
        progress: FlagEditor::from(challenges.progress_flag),
        order: FlagEditor::from(challenges.order_flag),
    };
    html! {
        <tr>
            <td>{props.npc.get_name_str(data.lang())}</td>
            <td><EnumInput<StatusEditor> editor={editor} /></td>
        </tr>
    }
}

impl Editor for StatusEditor {
    type Target = CommunityStatus;

    fn get(&self, save: &SaveData) -> Self::Target {
        CommunityStatus::from_repr(self.progress.get(save)).expect("unknown status")
    }

    fn set(&self, save: &mut SaveData, new: Self::Target) {
        let mut chrono_editor = CommunityChrono::new(save);
        if new == CommunityStatus::Unregistered {
            chrono_editor.delete(self.order.flag_index as usize);
        } else {
            chrono_editor.insert(self.order.flag_index as usize);
        }
        self.progress.set(save, new as u32);
    }
}

impl ToHtml for CommunityStatus {
    fn to_html(&self) -> Html {
        let lang = match self {
            CommunityStatus::Unregistered => "none",
            CommunityStatus::Registered => "regist",
            CommunityStatus::ChallengeComplete => "complete_task",
            CommunityStatus::Complete => "complete",
        };
        html!(<Text path ={format!("dlc4_comm_status_{lang}")} />)
    }
}
