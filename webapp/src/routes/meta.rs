use recordkeeper::flags::FlagType;
use ybc::{Container, Control, Field, Select, Tile, Title};
use yew::prelude::*;

use crate::{
    components::edit::{FlagEditor, NumberInput},
    components::meta::misc::Settings,
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
fn PlayTime() -> Html {
    let save = use_context::<SaveContext>().unwrap();
    let save = save.get();
    let save = save.get().save();
    let (hours, mins, secs) = save.play_time.to_hours_mins_secs();

    html! {
        <Tile classes={classes!("is-child", "notification")}>
            <Title><Text path="meta_playtime" /></Title>

            <Field>
                <label class="label"><Text path="hours" /></label>
                <Control>
                    <input class="input" type="number" min="0" max="9999" value={hours.to_string()} />
                </Control>
            </Field>

            <Field>
                <label class="label"><Text path="minutes" /></label>
                <Control>
                    <input class="input" type="number" min="0" max="59" value={mins.to_string()} />
                </Control>
            </Field>

            <Field>
                <label class="label"><Text path="seconds" /></label>
                <Control>
                    <input class="input" type="number" min="0" max="59" value={secs.to_string()} />
                </Control>
            </Field>
        </Tile>
    }
}

#[function_component]
fn Timestamps() -> Html {
    let save = use_context::<SaveContext>().unwrap();
    let save = save.get();
    let save = save.get().save();
    let date = save.timestamp.to_iso_date();
    let time = save.timestamp.to_iso_time();

    html! {
        <Tile classes={classes!("is-child", "notification")}>
            <Title><Text path="meta_savetime" /></Title>

            <Field>
                <label class="label"><Text path="date" /></label>
                <Control>
                    <input class="input" type="date" value={date} />
                </Control>
            </Field>

            <Field>
                <label class="label"><Text path="time" /></label>
                <Control>
                    <input class="input" type="time" value={time} />
                </Control>
            </Field>
        </Tile>
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
