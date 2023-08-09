use ybc::{Checkbox, Container, Control, Field, Select, Tile, Title};
use yew::prelude::*;

use crate::{lang::Text, save::SaveContext};

#[function_component]
pub fn SaveMeta() -> Html {
    let save = use_context::<SaveContext>().unwrap();
    let save = save.get();

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
    html! {
        <Tile classes={classes!("is-child", "notification")}>
            <Title><Text path="meta_playtime" /></Title>

            <Field>
                <label class="label"><Text path="hours" /></label>
                <Control>
                    <input class="input" type="number" min="0" max="9999" />
                </Control>
            </Field>

            <Field>
                <label class="label"><Text path="minutes" /></label>
                <Control>
                    <input class="input" type="number" min="0" max="59" />
                </Control>
            </Field>

            <Field>
                <label class="label"><Text path="seconds" /></label>
                <Control>
                    <input class="input" type="number" min="0" max="59" />
                </Control>
            </Field>
        </Tile>
    }
}

#[function_component]
fn Timestamps() -> Html {
    html! {
        <Tile classes={classes!("is-child", "notification")}>
            <Title><Text path="meta_savetime" /></Title>

            <Field>
                <label class="label"><Text path="date" /></label>
                <Control>
                    <input class="input" type="date" />
                </Control>
            </Field>

            <Field>
                <label class="label"><Text path="time" /></label>
                <Control>
                    <input class="input" type="time" />
                </Control>
            </Field>
        </Tile>
    }
}

#[function_component]
fn ScenarioFlag() -> Html {
    html! {
        <Tile classes={classes!("is-child", "notification")}>
            <Title><Text path="meta_scenario_flag" /></Title>

            <Field>
                <label class="label"><Text path="scenario_flag_flag" /></label>
                <Control>
                    <input class="input" type="number" />
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

#[function_component]
fn Settings() -> Html {
    html! {
        <Tile classes={classes!("is-child", "notification")}>
            <Title><Text path="meta_settings" /></Title>

            <Field>
                <label class="label"><Text path="difficulty" /></label>
                <Control>
                    <Select name="difficulty" value="normal" update={Callback::from(move |_| ())}>
                        <option value="normal">{"Normal"}</option>
                    </Select>
                </Control>
            </Field>

            <Field>
                <Control>
                    <Checkbox name="ngp" checked={false} update={Callback::from(move |_| ())}>
                        {" "}<Text path="meta_ngp" />
                    </Checkbox>
                </Control>
            </Field>
        </Tile>
    }
}
