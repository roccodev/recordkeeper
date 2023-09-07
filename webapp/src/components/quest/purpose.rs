use std::borrow::Cow;

use game_data::{
    lang::Nameable,
    quest::{PurposeTask, QuestPurpose},
};
use recordkeeper::flags::FlagType;
use ybc::{Button, Buttons, Modal, Table};
use yew::prelude::*;

use crate::{
    components::{
        edit::{Editor, EnumInput, FlagEditor},
        quest::StatusEditor,
    },
    data::Data,
    lang::Text,
    save::SaveContext,
};

#[derive(Properties, PartialEq)]
pub struct PurposeModalProps {
    pub quest_id: Option<usize>,
    #[prop_or_default]
    pub close_callback: Callback<()>,
}

#[derive(Properties, PartialEq)]
struct PurposeProps {
    pub purpose: QuestPurpose,
}

#[derive(Properties, PartialEq)]
struct TaskProps {
    pub task: PurposeTask,
}

#[function_component]
pub fn PurposeModal(props: &PurposeModalProps) -> Html {
    let data = use_context::<Data>().unwrap();

    let quest = match props.quest_id {
        Some(slot) => slot,
        None => return html!(),
    };
    let quest = data.game().quests.get(quest).expect("quest not found");

    let close_fn = props.close_callback.clone();
    let close_callback = Callback::from(move |_: MouseEvent| {
        close_fn.emit(());
    });

    html! {
        <div class={classes!("modal", "is-active")}>
            <div class="modal-background"></div>
            <div class={classes!("modal-content", "recordkeeper-task-modal")}>
                <div class={classes!("modal-card", "recordkeeper-task-modal")}>
                    <header class="modal-card-head">
                        <p class="modal-card-title"><Text path="quest_purpose" /></p>
                        <button class="delete" aria-label="close" onclick={close_callback}></button>
                    </header>
                    <section class="modal-card-body">
                        <Table classes={classes!("is-fullwidth")}>
                            <thead>
                                <tr>
                                    <th><Text path="quest_purpose_id" /></th>
                                    <th><Text path="quest_purpose_status" /></th>
                                    <th><Text path="quest_purpose_tasks_a" /></th>
                                    <th><Text path="quest_purpose_tasks_b" /></th>
                                </tr>
                            </thead>
                            <tbody>
                                {for quest.purposes.iter().map(|purpose| {
                                    html!(<PurposeRow purpose={*purpose} />)
                                })}
                            </tbody>
                        </Table>
                    </section>
                </div>
            </div>
        </div>
    }
}

#[function_component]
fn PurposeRow(props: &PurposeProps) -> Html {
    let status_editor = StatusEditor(FlagEditor {
        flag_type: FlagType::TwoBits,
        flag_index: props.purpose.flag,
    });

    html! {
        <tr>
            <th>{props.purpose.id}</th>
            <td><EnumInput<StatusEditor> editor={status_editor} /></td>
            <td>
                <Buttons classes={classes!("has-addons")}>
                    {for props.purpose.tasks.iter()
                        .filter_map(|t| t.and_then(|t| (t.branch == 1).then_some(t)))
                        .map(|t| html!(<TaskButton task={t} />))
                    }
                </Buttons>
            </td>
            <td>
                <Buttons classes={classes!("has-addons")}>
                    {for props.purpose.tasks.iter()
                        .filter_map(|t| t.and_then(|t| (t.branch == 2).then_some(t)))
                        .map(|t| html!(<TaskButton task={t} />))
                    }
                </Buttons>
            </td>
        </tr>
    }
}

#[function_component]
fn TaskButton(props: &TaskProps) -> Html {
    let data = use_context::<Data>().unwrap();
    let save_context = use_context::<SaveContext>().unwrap();
    let save = save_context.get();

    let flag_editor = FlagEditor {
        flag_type: FlagType::Byte,
        flag_index: props.task.flag,
    };

    let flag_value = flag_editor.get(save.get().save());
    let button_class = classes!(
        "button",
        match u8::try_from(flag_value).unwrap() {
            0 => "",                 // not started
            u8::MAX => "is-success", // completed
            _ => "is-warning",       // in progress
        }
    );

    let type_lang = format!("quest_task_{}", props.task.task_type.lang_id());

    let save_context = save_context.clone();
    let callback = Callback::from(move |_: MouseEvent| {
        save_context.edit(move |save| {
            flag_editor.set(save, if flag_value == 0 { u8::MAX.into() } else { 0 })
        })
    });

    html! {
            <Button classes={button_class} onclick={callback}>
                <span class="recordkeeper-task-name">
                    {props.task.get_name_str(data.lang()).map(Cow::from)
                        .unwrap_or_else(|| format!("#{}", props.task.id).into())}
                </span>
                {"\u{00a0}"}
                <span>
                    {" ("}<Text path={type_lang} />{")"}
                </span>
            </Button>
    }
}
