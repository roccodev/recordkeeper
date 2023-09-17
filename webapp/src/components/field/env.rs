use recordkeeper::SaveFlag;
use ybc::{Control, Field, Tile, Title};
use yew::prelude::*;

use super::MetaFlagEditor;
use crate::{
    components::edit::{editor, CheckboxInput, NumberInput},
    lang::Text,
};

#[rustfmt::skip]
editor!(
    WeatherEditor,
    u16,
    get |_, save| save.weather,
    set |_, save, new| save.weather = new
);

#[rustfmt::skip]
editor!(
    TimeHourEditor,
    u16,
    get |_, save| save.map_time.hour,
    set |_, save, new| save.map_time.hour = new
);

#[rustfmt::skip]
editor!(
    TimeMinuteEditor,
    u16,
    get |_, save| save.map_time.minute,
    set |_, save, new| save.map_time.minute = new
);

#[function_component]
pub fn Environment() -> Html {
    html! {
        <Tile classes={classes!("is-child", "notification")}>
            <Title><Text path="field_environment" /></Title>

            <Field>
                <label class="label"><Text path="field_time_hour" /></label>
                <Control>
                    <NumberInput<TimeHourEditor> editor={TimeHourEditor {}} max={23} />
                </Control>
            </Field>

            <Field>
                <label class="label"><Text path="field_time_minute" /></label>
                <Control>
                    <NumberInput<TimeMinuteEditor> editor={TimeMinuteEditor {}} max={59} />
                </Control>
            </Field>

            <Field>
                <label class="label"><Text path="field_weather" /></label>
                <Control>
                    <NumberInput<WeatherEditor> editor={WeatherEditor {}} />
                </Control>
            </Field>

            <Field>
                <Control>
                    <CheckboxInput<MetaFlagEditor> editor={MetaFlagEditor { flag: SaveFlag::TimeLocked }}>
                        {" "}<Text path="field_time_lock" />
                    </CheckboxInput<MetaFlagEditor>>
                </Control>
            </Field>

            <Field>
                <Control>
                    <CheckboxInput<MetaFlagEditor> editor={MetaFlagEditor { flag: SaveFlag::WeatherLocked }}>
                        {" "}<Text path="field_weather_lock" />
                    </CheckboxInput<MetaFlagEditor>>
                </Control>
            </Field>
        </Tile>
    }
}
