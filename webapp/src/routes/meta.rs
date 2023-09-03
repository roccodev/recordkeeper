use recordkeeper::flags::FlagType;
use ybc::{Container, Control, Field, Select, Tile, Title};
use yew::prelude::*;

use crate::{
    components::edit::{FlagEditor, NumberInput},
    components::meta::{
        misc::Settings,
        time::{PlayTime, Timestamps},
    },
    lang::Text,
    save::SaveContext,
};

#[function_component]
pub fn SaveMeta() -> Html {
    html! {
        <Container>
            <Tile>
                <Tile classes={classes!("is-parent")}>
                    <PlayTime />
                </Tile>
                <Tile classes={classes!("is-parent")}>
                    <Timestamps />
                </Tile>
            </Tile>
            <Tile>
                <Tile classes={classes!("is-parent")}>
                    <ScenarioFlag />
                </Tile>
                <Tile classes={classes!("is-parent")}>
                    <Settings />
                </Tile>
            </Tile>
        </Container>
    }
}

#[function_component]
fn ScenarioFlag() -> Html {
    let save = use_context::<SaveContext>().unwrap();
    let save = save.get();
    let save = save.get().save();

    let scenario_editor = FlagEditor {
        flag_type: FlagType::Short,
        flag_index: 1,
    };

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
                <Control>
                    <Select name="scenario-ch" value="ch1" update={Callback::from(move |_| ())}>
                        <option value="ch1">{"Chapter 1"}</option>
                    </Select>
                </Control>
            </Field>

            <Field>
                <label class="label"><Text path="scenario_flag_event" /></label>
                <Control>
                    <Select name="scenario-ev" value="ev001" update={Callback::from(move |_| ())}>
                        <option value="ev001">{"ev001"}</option>
                    </Select>
                </Control>
            </Field>
        </Tile>
    }
}
