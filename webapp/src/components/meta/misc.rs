use recordkeeper::flags::FlagType;
use ybc::{Control, Field, Select, Tile, Title};
use yew::prelude::*;

use crate::components::edit::{CheckboxInput, FlagEditor, ToBool};
use crate::lang::Text;
use crate::save::SaveContext;

#[function_component]
pub fn Settings() -> Html {
    let save = use_context::<SaveContext>().unwrap();
    let save = save.get();
    let save = save.get().save();

    // TODO: load from json
    let ngp_editor = ToBool(FlagEditor {
        flag_type: FlagType::Bit,
        flag_index: 23894,
    });
    let difficulty = save.flags.get(FlagType::TwoBits, 4554).unwrap() as usize;

    html! {
        <Tile classes={classes!("is-child", "notification")}>
            <Title><Text path="meta_settings" /></Title>

            <Field>
                <label class="label"><Text path="difficulty" /></label>
                <Control>
                    <Select name="difficulty" value={difficulty.to_string()} update={Callback::from(move |_| ())}>
                        {for ["normal", "easy", "hard", "veryhard"].into_iter().enumerate().map(|(i, key)| html! {
                            <option value={i.to_string()} selected={i == difficulty}><Text path={format!("difficulty_{key}")}/></option>
                        })}
                    </Select>
                </Control>
            </Field>

            <Field>
                <Control>
                    <CheckboxInput<ToBool<FlagEditor>> editor={ngp_editor}>
                        {" "}<Text path="meta_ngp" />
                    </CheckboxInput<ToBool<FlagEditor>>>
                </Control>
            </Field>
        </Tile>
    }
}
