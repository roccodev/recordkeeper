use game_data::{lang::Nameable, quest::QuestStatus};
use recordkeeper::flags::FlagType;
use ybc::{Button, Control, Icon};
use yew::prelude::*;
use yew_feather::Info;

use crate::{
    components::{
        edit::{EnumInput, FlagEditor},
        quest::purpose::PurposeModal,
    },
    data::Data,
    lang::Text,
    save::SaveContext,
    ToHtml,
};

use super::edit::Editor;

mod purpose;

#[derive(Properties, PartialEq, Clone)]
pub struct QuestEditorProps {
    pub id: usize,
}

#[derive(Clone, Copy, PartialEq)]
struct StatusEditor(FlagEditor);

#[function_component]
pub fn QuestRow(props: &QuestEditorProps) -> Html {
    let data = use_context::<Data>().unwrap();
    let purpose_modal = use_state(|| false);
    let lang = data.to_lang();

    let quest = data.game().quests.get(props.id).expect("quest not found");
    let status_editor = StatusEditor(FlagEditor {
        flag_type: FlagType::TwoBits,
        flag_index: quest.flag,
    });

    let purpose_state = purpose_modal.clone();
    let purpose_callback = Callback::from(move |_: MouseEvent| {
        purpose_state.set(true);
    });
    let purpose_state = purpose_modal.clone();
    let purpose_close_callback = Callback::from(move |_| {
        purpose_state.set(false);
    });

    html! {
        <>
            <PurposeModal quest_id={(*purpose_modal).then_some(props.id)} close_callback={purpose_close_callback} />
            <tr>
                <th>{props.id.to_string()}</th>
                <td>{quest.get_name_str(&lang)}</td>
                <td>
                    <EnumInput<StatusEditor> editor={status_editor} />
                </td>
                <td>
                    <Control>
                        <Button onclick={purpose_callback}>
                            <Icon>
                                <Info />
                            </Icon>
                        </Button>
                    </Control>
                </td>
            </tr>
        </>
    }
}

impl Editor for StatusEditor {
    type Target = QuestStatus;

    fn get(&self, save: &recordkeeper::SaveData) -> Self::Target {
        QuestStatus::from_repr(self.0.get(save) as usize).expect("unknown status")
    }

    fn set(&self, save: &mut recordkeeper::SaveData, new: Self::Target) {
        self.0.set(save, new as u32);
    }
}

impl ToHtml for QuestStatus {
    fn to_html(&self) -> Html {
        let id = match self {
            QuestStatus::Unstarted => "unstarted",
            QuestStatus::InProgress => "progress",
            QuestStatus::CompletedA => "complete_a",
            QuestStatus::CompletedB => "complete_b",
        };
        html!(<Text path={format!("quest_status_{id}")} />)
    }
}
