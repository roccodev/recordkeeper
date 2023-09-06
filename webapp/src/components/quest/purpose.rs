use game_data::quest::QuestPurpose;
use recordkeeper::flags::FlagType;
use ybc::{Modal, Table};
use yew::prelude::*;

use crate::{
    components::{
        edit::{EnumInput, FlagEditor},
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
pub struct PurposeProps {
    pub purpose: QuestPurpose,
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
        <Modal id="card" classes={classes!("is-active")}>
            <div class="modal-card">
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
        </Modal>
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
        </tr>
    }
}
