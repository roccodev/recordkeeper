use crate::{lang::Text, save::SaveContext};
use recordkeeper::{PlayTime as SavePlayTime, SaveTimestamp};
use std::str::FromStr;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use ybc::{Control, Field, Tile, Title};
use yew::prelude::*;

#[function_component]
pub fn PlayTime() -> Html {
    let save_context = use_context::<SaveContext>().unwrap();
    let save = save_context.get();
    let save = save.get_save();

    let (hours, mins, secs) = save.play_time.to_hours_mins_secs();

    let update_time = {
        let save_context = save_context.clone();
        Callback::from(move |(hours, mins, secs)| {
            save_context.edit(move |save| {
                save.play_time = SavePlayTime::from_seconds(secs + mins * 60 + hours * 3600)
            });
        })
    };

    html! {
        <Tile classes={classes!("is-child", "notification")}>
            <Title><Text path="meta_playtime" /></Title>

            <Field>
                <label class="label"><Text path="hours" /></label>
                <Control>
                    <input class="input" type="number" min="0" max="9999" value={hours.to_string()} oninput={
                        let update_time = update_time.clone();
                        Callback::from(move |e: InputEvent| {
                            let value = number_from_event(e, hours);
                            update_time.emit((value, mins, secs))
                        })
                    } />
                </Control>
            </Field>

            <Field>
                <label class="label"><Text path="minutes" /></label>
                <Control>
                    <input class="input" type="number" min="0" max="59" value={mins.to_string()} oninput={
                        let update_time = update_time.clone();
                        Callback::from(move |e: InputEvent| {
                            let value = number_from_event(e, mins);
                            update_time.emit((hours, value, secs))
                        })
                    } />
                </Control>
            </Field>

            <Field>
                <label class="label"><Text path="seconds" /></label>
                <Control>
                    <input class="input" type="number" min="0" max="59" value={secs.to_string()} oninput={Callback::from(move |e: InputEvent| {
                        let value = number_from_event(e, secs);
                        update_time.emit((hours, mins, value))
                    })} />
                </Control>
            </Field>
        </Tile>
    }
}

#[function_component]
pub fn Timestamps() -> Html {
    let save_context = use_context::<SaveContext>().unwrap();
    let save = save_context.get();
    let save = save.get_save();

    let timestamp = save.timestamp;

    let update_timestamp = {
        let save_context = save_context.clone();
        Callback::from(move |timestamp| save_context.edit(move |save| save.timestamp = timestamp))
    };

    html! {
        <Tile classes={classes!("is-child", "notification")}>
            <Title><Text path="meta_savetime" /></Title>

            <Field>
                <label class="label"><Text path="date" /></label>
                <Control>
                    <input class="input" type="date" value={timestamp.to_iso_date()} oninput={
                        let update_timestamp = update_timestamp.clone();
                        Callback::from(move |e: InputEvent| update_timestamp.emit(date_from_event(e, timestamp)))
                    } />
                </Control>
            </Field>

            <Field>
                <label class="label"><Text path="time" /></label>
                <Control>
                    <input class="input" type="time" value={timestamp.to_iso_time()} oninput={Callback::from(move |e: InputEvent| {
                        update_timestamp.emit(time_from_event(e, timestamp))
                    })} />
                </Control>
            </Field>
        </Tile>
    }
}

fn number_from_event(event: InputEvent, prev_value: u32) -> u32 {
    let target: Option<EventTarget> = event.target();
    let Some(input) = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) else {
        return prev_value;
    };
    match u32::from_str(&input.value()) {
        Ok(n) => n,
        Err(_) => {
            event.prevent_default();
            input.set_value(&prev_value.to_string());
            prev_value
        }
    }
}

fn date_from_event(event: InputEvent, prev_time: SaveTimestamp) -> SaveTimestamp {
    let target: Option<EventTarget> = event.target();
    let Some(input) = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) else {
        return prev_time;
    };
    let value = input.value();
    let mut split = value.split('-');
    let Ok(year) = split.next().unwrap().parse() else {
        return prev_time;
    };
    let Ok(month) = split.next().unwrap().parse() else {
        return prev_time;
    };
    let Ok(day) = split.next().unwrap().parse() else {
        return prev_time;
    };
    SaveTimestamp::from_date_time(year, month, day, prev_time.hour(), prev_time.minute())
}

fn time_from_event(event: InputEvent, prev_time: SaveTimestamp) -> SaveTimestamp {
    let target: Option<EventTarget> = event.target();
    let Some(input) = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok()) else {
        return prev_time;
    };
    let value = input.value();
    let mut split = value.split(':');
    let Ok(hour) = split.next().unwrap().parse() else {
        return prev_time;
    };
    let Ok(minute) = split.next().unwrap().parse() else {
        return prev_time;
    };
    SaveTimestamp::from_date_time(
        prev_time.year(),
        prev_time.month(),
        prev_time.day(),
        hour,
        minute,
    )
}
