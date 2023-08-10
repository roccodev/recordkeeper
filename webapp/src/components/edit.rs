use std::{fmt::Display, str::FromStr};

use recordkeeper::SaveData;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

use crate::save::{EditAction, SaveContext};

/// Helper structs that query and edit a field or a portion
/// of the save file.
///
/// To easily make new editors, use the [`editor!`] and [`flag_editor!`]
/// macros.
pub trait Editor: Copy + 'static {
    /// The value type to get and set.
    type Target;

    /// Gets the current value of the observed target from an instance
    /// of the save file.
    fn get(&self, save: &SaveData) -> Self::Target;

    /// Updates the value of the observed target for an instance of the
    /// save file.
    fn set(&self, save: &mut SaveData, new: Self::Target);

    /// Checks whether the given value is valid for the target.
    ///
    /// On failure, an error message can be returned as the error value.
    fn validate(_value: &Self::Target) -> Result<(), String> {
        Ok(())
    }
}

#[rustfmt::skip]
macro_rules! flag_editor {
    ($name:ident, $flag_type:expr, $flag_index:expr) => {
        $crate::components::edit::editor!(
            $name,
            u32,
            get |save| save.flags.get($flag_type, $flag_index).unwrap(),
            set |save, new_value| save.flags.set($flag_type, $flag_index, new_value),
            assert |value| $flag_type.is_valid(*value).then_some(()).ok_or_else(|| String::from("value too big"))
        );
    };
}

macro_rules! editor {
    ($name:ident, $value:tt, get $get_fn:expr, set $set_fn:expr) => {
        $crate::components::edit::editor!($name, $value, get $get_fn, set $set_fn, assert |_| Ok(()));
    };
    ($name:ident, $value:tt, get $get_fn:expr, set $set_fn:expr, assert $check_fn:expr) => {
        #[derive(Copy, Clone, PartialEq)]
        struct $name;

        impl $crate::components::edit::Editor for $name {
            type Target = $value;

            fn get(&self, save: &recordkeeper::SaveData) -> Self::Target {
                // required for type inference
                let getter: &dyn Fn(&recordkeeper::SaveData) -> Self::Target = &$get_fn;
                (getter)(save)
            }

            fn set(&self, save: &mut recordkeeper::SaveData, new: Self::Target) {
                let setter: &dyn Fn(&mut recordkeeper::SaveData, Self::Target) = &$set_fn;
                (setter)(save, new)
            }

            fn validate(value: &Self::Target) -> Result<(), String> {
                let checker: &dyn Fn(&Self::Target) -> Result<(), String> = &$check_fn;
                (checker)(value)
            }
        }
    };
}

pub(crate) use editor;
pub(crate) use flag_editor;

#[derive(Properties, PartialEq)]
pub struct EditorProps<E: PartialEq> {
    pub editor: E,
}

/// General-purpose number input that automatically saves changes to
/// the save file.
#[function_component]
pub fn NumberInput<E: Editor + PartialEq>(props: &EditorProps<E>) -> Html
where
    <E as Editor>::Target: Display + FromStr,
{
    let save_context = use_context::<SaveContext>().unwrap();
    let current_value = {
        let save = save_context.get();
        props.editor.get(save.get().save())
    };
    let editor = props.editor;

    let change_listener = Callback::from(move |e: Event| {
        let target: Option<EventTarget> = e.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

        if let Some(input) = input {
            match <E as Editor>::Target::from_str(&input.value())
                .map_err(|_| ())
                .and_then(|v| E::validate(&v).map_err(|_| ()).map(|_| v))
            {
                Ok(v) => {
                    save_context
                        .submit_action(EditAction::Edit(Box::new(move |save| editor.set(save, v))));
                }
                Err(_) => {
                    // Invalid number, out of range, etc.
                    e.prevent_default();
                }
            }
        }
    });

    html! {
        <input class="input" type="number" value={current_value.to_string()} onchange={change_listener} />
    }
}
