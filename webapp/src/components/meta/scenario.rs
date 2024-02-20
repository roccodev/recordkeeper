use std::borrow::Cow;

use ybc::{Control, Field, Tile, Title};
use yew::prelude::*;

use crate::{
    components::edit::{Editor, FlagEditor, NumberInput},
    data::Data,
    lang::Text,
    save::SaveContext,
};

#[function_component]
pub fn ScenarioFlag() -> Html {
    let data = use_context::<Data>().unwrap();
    let save = use_context::<SaveContext>().unwrap();
    let save = save.get();
    let save = save.get_save();

    let scenario_editor = FlagEditor::from(data.game().manual.flags.scenario);
    let chapter = data
        .game()
        .events
        .get_chapter_by_scenario(
            scenario_editor.get(save).try_into().unwrap(),
            save.is_dlc4(),
        )
        .map(|i| Cow::Owned(i.to_string()))
        .unwrap_or_else(|| Cow::Borrowed("???"));

    html! {
        <Tile classes={classes!("is-child", "notification")}>
            <Title><Text path="meta_scenario_flag" /></Title>

            <Field>
                <label class="label"><Text path="scenario_flag_flag" /></label>
                <Control>
                    <NumberInput<FlagEditor> editor={scenario_editor} />
                </Control>
            </Field>

            <Field>
                <label class="label"><Text path="scenario_flag_chapter" /></label>
                <p>
                    <Text path="scenario_flag_chapter_id" args={vec![("id".into(), chapter.into())]} />
                </p>
            </Field>
        </Tile>
    }
}
