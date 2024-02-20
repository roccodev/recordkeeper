use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use game_data::manual::Flag;
use recordkeeper::{flags::FlagType, SaveData};
use strum::IntoEnumIterator;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use ybc::{Button, Checkbox, Control, Field, Icon};
use yew::prelude::*;
use yew_feather::X;

use crate::{
    components::select::{HtmlSelect, SearchSelect},
    data::Data,
    save::SaveContext,
    ToHtml,
};

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
    ($vis:vis $name:ident, $value:ty, get $get_fn:expr, set $set_fn:expr) => {
        $crate::components::edit::editor!($vis $name, $value, get $get_fn, set $set_fn, assert |_, _| Ok(()), capture);
    };
    ($vis:vis $name:ident, $value:ty, get $get_fn:expr, set $set_fn:expr, capture $($field: ident: $ty: ty),*) => {
        $crate::components::edit::editor!($vis $name, $value, get $get_fn, set $set_fn, assert |_, _| Ok(()), capture $($field: $ty),*);
    };
    ($vis:vis $name:ident, $value:ty, get $get_fn:expr, set $set_fn:expr, assert $check_fn:expr, capture $($field: ident: $ty: ty),*) => {
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

use super::select::{HtmlName, Options};

#[derive(Clone, Copy, PartialEq)]
pub struct ToBool<E: Editor>(pub E)
where
    E::Target: FlagConvert;

pub trait FlagConvert {
    fn to_bool(self) -> bool;
    fn from_bool(b: bool) -> Self;
}

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

#[derive(Properties, PartialEq, Clone, Copy)]
pub struct EditorProps<E: Editor + PartialEq>
where
    <E as Editor>::Target: PartialEq,
{
    pub editor: E,
}

#[derive(Properties, PartialEq, Clone)]
pub struct StringInputProps<E: Editor + PartialEq>
where
    <E as Editor>::Target: PartialEq,
{
    pub editor: E,
    #[prop_or_default]
    pub input_type: AttrValue,
    #[prop_or_default]
    pub filter: Option<Callback<E::Target, bool>>,
    #[prop_or_default]
    pub disabled: Option<Callback<E::Target, bool>>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct CheckboxInputProps<E: Editor + PartialEq>
where
    <E as Editor>::Target: PartialEq,
{
    pub editor: E,
    #[prop_or_default]
    pub children: Children,
}

#[derive(Properties, PartialEq)]
pub struct SearchInputProps<E: PartialEq, I: PartialEq + 'static> {
    pub editor: E,
    pub options: Options<I>,
}

/// General-purpose number input that automatically saves changes to
/// the save file.
#[function_component]
pub fn NumberInput<E: Editor + PartialEq>(props: &NumberEditorProps<E>) -> Html
where
    <E as Editor>::Target: Eq + Ord + Display + FromStr + Copy,
{
    let props = *props;
    let editor = props.editor;

    html! {
        <StringInput<E::Target, E>
            editor={editor}
            input_type="number"
            filter={Callback::from(move |v| props.check_range(v))}
        />
    }
}

/// Select dropdown for enum-like types
#[function_component]
pub fn EnumInput<E: Editor + PartialEq>(props: &EditorProps<E>) -> Html
where
    <E as Editor>::Target: PartialEq + IntoEnumIterator + ToHtml,
{
    let save_context = use_context::<SaveContext>().unwrap();
    let current_value = {
        let save = save_context.get();
        props.editor.get(save.get_save())
    };
    let current_index = <E as Editor>::Target::iter()
        .position(|e| e == current_value)
        .unwrap();

    let editor = props.editor;
    let callback = Callback::from(move |val: String| {
        let value = val.parse::<usize>().unwrap();
        let value = <E as Editor>::Target::iter().nth(value).unwrap();
        save_context.edit(move |save| editor.set(save, value));
    });

    html! {
        <HtmlSelect value={current_index.to_string()} on_change={callback} selected_idx={current_index}>
            {for <E as Editor>::Target::iter().enumerate().map(|(i, v)| {
                html!(<option value={i.to_string()} selected={v == current_value}>{v.to_html()}</option>)
            })}
        </HtmlSelect>
    }
}

/// Checkbox field for boolean editors
#[function_component]
pub fn CheckboxInput<E: Editor<Target = bool> + PartialEq>(props: &CheckboxInputProps<E>) -> Html {
    let save_context = use_context::<SaveContext>().unwrap();
    let checked = {
        let save = save_context.get();
        props.editor.get(save.get_save())
    };

    let editor = props.editor;
    let update = Callback::from(move |_| save_context.edit(move |save| editor.set(save, !checked)));

    html! {
        <Checkbox name="ngp" checked={checked} update={update}>
            {for props.children.clone()}
        </Checkbox>
    }
}

/// Input field for types that can be parsed from strings
#[function_component]
pub fn StringInput<T, E>(props: &StringInputProps<E>) -> Html
where
    T: ToString + FromStr + Eq + Clone + 'static,
    E: Editor<Target = T> + PartialEq,
{
    let save_context = use_context::<SaveContext>().unwrap();
    let current_value = {
        let save = save_context.get();
        props.editor.get(save.get_save())
    };

    let input = use_state(String::new);
    let valid = use_state_eq(|| true);
    {
        let input = input.clone();
        let valid = valid.clone();
        use_effect_with_deps(
            move |v| {
                input.set(v.to_string());
                valid.set(true);
            },
            current_value.clone(),
        );
    }

    let editor = props.editor;

    let change_listener = {
        let input_state = input.clone();
        let valid_state = valid.clone();
        let filter = props.filter.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                let value = input.value();
                match <E as Editor>::Target::from_str(&value)
                    .map_err(|_| ())
                    .and_then(|v| editor.validate(&v).map_err(|_| ()).map(|_| v))
                    .ok()
                    .and_then(|v| match &filter {
                        Some(filter) => filter.emit(v.clone()).then_some(v),
                        None => Some(v),
                    }) {
                    Some(v) => {
                        save_context.edit(move |save| editor.set(save, v));
                        valid_state.set(true);
                    }
                    None => {
                        e.prevent_default();
                        valid_state.set(false);
                    }
                };
                input_state.set(value)
            }
        })
    };

    let classes = if *valid {
        classes!("input")
    } else {
        classes!("input", "is-danger")
    };

    let disabled = props
        .disabled
        .as_ref()
        .map(|c| c.emit(current_value))
        .unwrap_or_default();

    html! {
        <input
            disabled={disabled}
            class={classes}
            type={props.input_type.clone()}
            value={input.to_string()}
            oninput={change_listener.reform(|e: InputEvent| e.dyn_into().unwrap())}
        />
    }
}

/// Select component with a button to empty the field, as well as
/// searchable options.
///
/// Note: only works when the option index = option ID - 1, and when the
/// "None" value is 0.
#[function_component]
pub fn SearchInput<E, I>(props: &SearchInputProps<E, I>) -> Html
where
    E: Editor + PartialEq,
    E::Target: TryInto<usize> + TryFrom<usize>,
    <E::Target as TryInto<usize>>::Error: Debug,
    <E::Target as TryFrom<usize>>::Error: Debug,
    I: Clone + HtmlName + PartialEq + 'static,
{
    let save_context = use_context::<SaveContext>().unwrap();
    let data = use_context::<Data>().unwrap();
    let lang = data.to_lang();

    let save = save_context.get();

    // Conveniently, this is None when the value is 0
    let current = props
        .editor
        .get(save.get_save())
        .try_into()
        .unwrap()
        .checked_sub(1);

    let update = {
        let editor = props.editor;
        let save_context = save_context.clone();
        Callback::from(move |new: usize| {
            save_context
                .edit(move |save| editor.set(save, new.checked_add(1).unwrap().try_into().unwrap()))
        })
    };

    let clear_callback = {
        let editor = props.editor;
        let save_context = save_context.clone();
        Callback::from(move |_: MouseEvent| {
            save_context.edit(move |save| editor.set(save, 0usize.try_into().unwrap()))
        })
    };

    html! {
        <Field classes={classes!("has-addons")}>
            <Control>
                <SearchSelect<I>
                    current={current}
                    options={props.options.clone()}
                    on_select={update}
                    lang={lang}
                />
            </Control>
            <Control>
                <Button onclick={clear_callback} disabled={current.is_none()}>
                    <Icon><X /></Icon>
                </Button>
            </Control>
        </Field>
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

impl<E: Editor> Editor for ToBool<E>
where
    E::Target: FlagConvert,
{
    type Target = bool;

    fn get(&self, save: &SaveData) -> Self::Target {
        self.0.get(save).to_bool()
    }

    fn set(&self, save: &mut SaveData, new: Self::Target) {
        self.0.set(save, E::Target::from_bool(new))
    }
}

impl FlagConvert for u32 {
    fn from_bool(b: bool) -> Self {
        u8::from(b).into()
    }

    fn to_bool(self) -> bool {
        self != 0
    }
}

impl From<Flag> for FlagEditor {
    fn from(value: Flag) -> Self {
        Self {
            flag_index: value.index,
            flag_type: FlagType::from_bits(value.bits),
        }
    }
}
