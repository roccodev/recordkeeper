use game_data::{
    dlc::community::CommunityTask,
    lang::{Nameable, TextEntry},
    manual::Flag,
};
use recordkeeper::{chrono::ChronologicalOrder, dlc::CommunityChrono};
use ybc::{Button, Buttons, Modal, Table};
use yew::prelude::*;
use yew_feather::{ArrowDown, ArrowUp};

use crate::{
    components::edit::{Editor, FlagEditor, NumberInput},
    data::Data,
    lang::Text,
    save::SaveContext,
};

#[derive(Properties, PartialEq)]
pub struct OrderModalProps {
    pub open: bool,
    #[prop_or_default]
    pub close_callback: Callback<()>,
}

#[derive(Properties, PartialEq)]
pub struct TaskProgressProps {
    pub task: CommunityTask,
}

#[derive(Properties, PartialEq)]
struct NpcEntryProps {
    npc: u32,
    flag: Flag,
    prev_flag: Option<Flag>,
    next_flag: Option<Flag>,
}

#[function_component]
pub fn CommunityOrderModal(props: &OrderModalProps) -> Html {
    let data = use_context::<Data>().unwrap();
    let save = use_context::<SaveContext>().unwrap();

    let close_fn = props.close_callback.clone();
    let close_callback = Callback::from(move |_: MouseEvent| {
        close_fn.emit(());
    });

    if !props.open {
        return html!();
    }

    let community = &data.game().dlc.community;
    let mut order: Vec<_> = {
        let save = save.get();
        let save = save.get_save();
        community
            .npc_challenges()
            .map(|(npc, comm)| {
                (
                    npc,
                    comm.order_flag,
                    FlagEditor::from(comm.order_flag).get(save),
                )
            })
            .filter(|(_, _, flag)| *flag != 0)
            .collect()
    };
    order.sort_unstable_by_key(|(_, _, flag)| *flag);

    html! {
        <Modal id="card" classes={classes!("is-active")}>
            <div class="modal-card">
                <header class="modal-card-head">
                    <p class="modal-card-title"><Text path="dlc4_comm_order" /></p>
                    <button class="delete" aria-label="close" onclick={close_callback}></button>
                </header>
                <section class="modal-card-body">
                    <Table classes="is-fullwidth">
                        <thead>
                            <tr>
                                <th></th>
                                <th></th>
                            </tr>
                        </thead>
                        <tbody>
                            {for order.iter().enumerate().map(|(i, (npc, flag, _))| {
                                let prev_flag = i.checked_sub(1).map(|i| order[i].1);
                                let next_flag = i.checked_add(1).and_then(|i| order.get(i)).map(|c| c.1);
                                html!(<NpcEntry npc={npc} flag={*flag} prev_flag={prev_flag} next_flag={next_flag} />)
                            })}
                        </tbody>
                    </Table>
                </section>
            </div>
        </Modal>
    }
}

#[function_component]
pub fn TaskProgress(props: &TaskProgressProps) -> Html {
    match props.task {
        CommunityTask::Talk { flag, max, .. } => html! {
            <NumberInput<FlagEditor> editor={FlagEditor::from(flag)} max={max} />
        },
        CommunityTask::Condition {
            progress_flag: Some(flag),
            ..
        } => html! {
            <NumberInput<FlagEditor> editor={FlagEditor::from(flag)} />
        },
        _ => html!(),
    }
}

#[function_component]
fn NpcEntry(props: &NpcEntryProps) -> Html {
    let data = use_context::<Data>().unwrap();
    let save = use_context::<SaveContext>().unwrap();
    let self_flag = props.flag.index;

    let swap = |flag: Option<Flag>| {
        let save = save.clone();
        match flag {
            Some(flag) => Callback::from(move |_: MouseEvent| {
                save.edit(move |save| CommunityChrono::new(save).swap(self_flag, flag.index))
            }),
            None => Callback::noop(),
        }
    };

    html! {
        <tr>
            <td>
                {data.game().npcs.get(props.npc).get_name(data.lang()).map(TextEntry::text)}
            </td>
            <td class={classes!("is-flex", "is-justify-content-end")}>
                <Buttons>
                    <Button disabled={props.prev_flag.is_none()} onclick={swap(props.prev_flag)}>
                        <ArrowUp />
                    </Button>
                    <Button disabled={props.next_flag.is_none()} onclick={swap(props.next_flag)}>
                        <ArrowDown />
                    </Button>
                </Buttons>
            </td>
        </tr>
    }
}
