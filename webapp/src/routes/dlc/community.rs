use game_data::{dlc::community::CommunityTask, npc::Npc};
use recordkeeper::{chrono::ChronologicalOrder, dlc::CommunityChrono, SaveData};
use ybc::{Button, Control, Field, Table};
use yew::prelude::*;

use crate::{
    components::{
        dlc::community::CommunityOrderModal,
        edit::{CheckboxInput, Editor, FlagEditor},
        select::Selector,
    },
    data::Data,
    lang::Text,
};

#[derive(Properties, PartialEq)]
pub struct NpcProps {
    npc_id: u32,
}

#[derive(Properties, PartialEq)]
pub struct TaskProps {
    npc_id: u32,
    task: usize,
}

#[derive(Clone, Copy, PartialEq)]
struct StatusEditor {
    progress: FlagEditor,
    order: FlagEditor,
    target: u32,
}

#[function_component]
pub fn CommunityPage() -> Html {
    let data = use_context::<Data>().unwrap();
    let npc = use_state(|| data.game().dlc.community.npcs[0].id as usize);
    let order_open = use_state(|| false);
    let order_change = |v: bool| {
        let order = order_open.clone();
        Callback::from(move |_| {
            order.set(v);
        })
    };

    let npc_id = *npc as u32;
    let comm_npc = data.game().dlc.community.challenge(npc_id);

    html! {
        <>
            <CommunityOrderModal open={*order_open} close_callback={order_change(false)} />
            <Field classes={classes!("is-grouped", "is-align-items-end")}>
                <Control classes="is-flex-grow-1">
                    <Field>
                        <label class="label"><Text path="dlc4_comm_npc" /></label>
                        <Control>
                            <Selector<Npc>
                                state={npc.clone()}
                                values={&*data.game().dlc.community.npcs}
                                sort_key={data.lang().dlc.community.npc_sort.clone()}
                            />
                        </Control>
                    </Field>
                </Control>
                <Control>
                    <Button onclick={order_change(true).reform(|_: MouseEvent| ())}>
                        <Text path="dlc4_comm_order" />
                    </Button>
                </Control>
            </Field>
            <Table classes={classes!("is-fullwidth")}>
                <thead>
                    <tr>
                        <th><Text path="dlc4_comm_status" /></th>
                        <th><Text path="dlc4_comm_type" /></th>
                        <th><Text path="dlc4_comm_desc" /></th>
                        <th><Text path="dlc4_comm_progress" /></th>
                    </tr>
                </thead>
                <tbody>
                    {for (0..comm_npc.tasks.len()).map(|task| html!(<TaskView npc_id={npc_id} task={task} />))}
                </tbody>
            </Table>
        </>
    }
}

#[function_component]
fn TaskView(props: &TaskProps) -> Html {
    let data = use_context::<Data>().unwrap();
    let challenges = &data.game().dlc.community.challenge(props.npc_id);
    let task = &challenges.tasks[props.task];
    let editor = StatusEditor {
        progress: FlagEditor::from(challenges.progress_flag),
        order: FlagEditor::from(challenges.order_flag),
        target: props.task as u32,
    };
    let type_fmt = format!(
        "dlc4_comm_task_{}",
        match task {
            CommunityTask::Talk { .. } => "talk",
            CommunityTask::Quest { .. } => "quest",
            CommunityTask::Condition { .. } => "condition",
        }
    );
    html! {
        <tr>
            <td><CheckboxInput<StatusEditor> editor={editor} /></td>
            <td><Text path={type_fmt}/></td>
            <td>{task.get_desc(data.game(), data.lang()).unwrap_or("")}</td>
            <td></td>
        </tr>
    }
}

impl Editor for StatusEditor {
    type Target = bool;

    fn get(&self, save: &SaveData) -> Self::Target {
        if self.order.get(save) == 0 {
            // Progress flag = 0 means the first task is complete, while
            // order flag = 0 means the community entry is absent (i.e. first task not complete)
            return false;
        }
        self.progress.get(save) >= self.target
    }

    fn set(&self, save: &mut SaveData, new: Self::Target) {
        let mut chrono_editor = CommunityChrono::new(save);
        if new {
            chrono_editor.insert(self.order.flag_index);
            let old_val = self.progress.get(save);
            if old_val < self.target {
                self.progress.set(save, self.target);
            }
        } else {
            if self.target == 0 {
                // Delete order entry if no task is completed
                chrono_editor.delete(self.order.flag_index);
            }
            // Undo progress thus far
            let new_val = self.target.saturating_sub(1);
            self.progress.set(save, new_val);
        }
    }
}
