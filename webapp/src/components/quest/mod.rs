use game_data::{lang::Nameable, quest::QuestStatus};
use recordkeeper::flags::FlagType;
use ybc::{Button, Control, Icon};
use yew::prelude::*;
use yew_feather::Info;

use crate::{
    components::edit::{EnumInput, FlagEditor},
    data::Data,
    lang::Text,
    save::SaveContext,
    ToHtml,
};

use super::edit::Editor;

#[derive(Properties, PartialEq, Clone)]
pub struct QuestEditorProps {
    pub id: usize,
}

#[derive(Clone, Copy, PartialEq)]
struct StatusEditor(FlagEditor);

#[function_component]
pub fn QuestRow(props: &QuestEditorProps) -> Html {
    let data = use_context::<Data>().unwrap();
    let lang = data.to_lang();
    let save_context = use_context::<SaveContext>().unwrap();
    let save = save_context.get();

    let quest = data.game().quests.get(props.id).expect("quest not found");
    let status_editor = StatusEditor(FlagEditor {
        flag_type: FlagType::TwoBits,
        flag_index: quest.flag,
    });

    html! {
        <>
            <tr>
                <th>{props.id.to_string()}</th>
                <td>{quest.get_name_str(&lang)}</td>
                <td>
                    <EnumInput<StatusEditor> editor={status_editor} />
                </td>
                <td>
                    <Control>
                        <Button>
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
