use std::{fmt::Display, str::FromStr};

use recordkeeper::SaveData;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

use crate::save::{EditAction, SaveContext};

/// Helper structs that query and edit a field or a portion
/// of the save file.
///
/// To easily make new editors, use the [`editor!`] macro.
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
    fn validate(&self, _value: &Self::Target) -> Result<(), String> {
        Ok(())
    }
}

#[rustfmt::skip]
editor!(
    pub FlagEditor,
    u32,
    get |editor, save| { save.flags.get(editor.flag_type, editor.flag_index).unwrap() },
    set |editor, save, new_value| { save.flags.set(editor.flag_type, editor.flag_index, new_value) },
    assert |editor, value| { editor.flag_type.is_valid(*value).then_some(()).ok_or_else(|| String::from("value too big")) },
    capture flag_type: recordkeeper::flags::FlagType, flag_index: usize
);

macro_rules! editor {
    ($vis:tt $name:ident, $value:tt, get $get_fn:expr, set $set_fn:expr) => {
        $crate::components::edit::editor!($vis $name, $value, get $get_fn, set $set_fn, assert |_| Ok(()), capture);
    };
    ($vis:tt $name:ident, $value:tt, get $get_fn:expr, set $set_fn:expr, assert $check_fn:expr, capture $($field: ident: $ty: ty),*) => {
        #[derive(Copy, Clone, PartialEq)]
        $vis struct $name {
            $(pub $field: $ty),*
        }

        impl $crate::components::edit::Editor for $name {
            type Target = $value;

            fn get(&self, save: &recordkeeper::SaveData) -> Self::Target {
                // required for type inference
                let getter: &dyn Fn(&Self, &recordkeeper::SaveData) -> Self::Target = &$get_fn;
                (getter)(self, save)
            }

            fn set(&self, save: &mut recordkeeper::SaveData, new: Self::Target) {
                let setter: &dyn Fn(&Self, &mut recordkeeper::SaveData, Self::Target) = &$set_fn;
                (setter)(self, save, new)
            }

            fn validate(&self, value: &Self::Target) -> Result<(), String> {
                let checker: &dyn Fn(&Self, &Self::Target) -> Result<(), String> = &$check_fn;
                (checker)(self, value)
            }
        }
    };
}

pub(crate) use editor;

#[derive(Properties, PartialEq, Clone, Copy)]
pub struct NumberEditorProps<E: Editor + PartialEq>
where
    <E as Editor>::Target: PartialEq + Copy,
{
    pub editor: E,
    #[prop_or_default]
    pub min: Option<<E as Editor>::Target>,
    #[prop_or_default]
    pub max: Option<<E as Editor>::Target>,
}

/// General-purpose number input that automatically saves changes to
/// the save file.
#[function_component]
pub fn NumberInput<E: Editor + PartialEq>(props: &NumberEditorProps<E>) -> Html
where
    <E as Editor>::Target: PartialEq + PartialOrd + Display + FromStr + Copy,
{
    let save_context = use_context::<SaveContext>().unwrap();
    let current_value = {
        let save = save_context.get();
        props.editor.get(save.get().save())
    };
    let value_display = current_value.to_string();
    let editor = props.editor;

    let props = *props;
    let change_listener = Callback::from(move |e: Event| {
        let target: Option<EventTarget> = e.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        if let Some(input) = input {
            match <E as Editor>::Target::from_str(&input.value())
                .map_err(|_| ())
                .and_then(|v| editor.validate(&v).map_err(|_| ()).map(|_| v))
                .ok()
                .and_then(|v| props.check_range(v).then_some(v))
            {
                Some(v) => {
                    save_context.edit(move |save| editor.set(save, v));
                }
                None => {
                    // Invalid number, out of range, etc.
                    e.prevent_default();
                    input.set_value(&value_display);
                }
            }
        }
    });

    html! {
        <input
            class="input" type="number"
            value={current_value.to_string()}
            oninput={change_listener.reform(|e: InputEvent| e.dyn_into().unwrap())}
        />
    }
}

impl<E: Editor + PartialEq> NumberEditorProps<E>
where
    <E as Editor>::Target: PartialEq + PartialOrd + Copy,
{
    fn check_range(&self, val: <E as Editor>::Target) -> bool {
        if matches!(&self.min, Some(min) if &val < min) {
            return false;
        }
        if matches!(&self.max, Some(max) if &val > max) {
            return false;
        }
        true
    }
}
