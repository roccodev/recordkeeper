use std::rc::Rc;

use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use ybc::*;
use yew::prelude::*;
use yew_feather::ChevronDown;

use game_data::lang::Nameable;
use game_data::LanguageData;

pub trait HtmlName {
    fn get_name_html(&self, language: &LanguageData) -> Html;
    fn get_name_for_filter<'a, 'b: 'a>(&'a self, language: &'b LanguageData) -> Option<&'a str>;
}

#[derive(Clone)]
pub enum Options<O: 'static> {
    Borrowed(&'static [O]),
    Owned(Rc<[O]>),
}

#[derive(Properties)]
pub struct SearchSelectProps<O: Clone + 'static> {
    pub current: Option<usize>,
    /// List of searchable/selectable options
    pub options: Options<O>,
    pub on_select: Callback<usize, ()>,
    pub lang: Rc<LanguageData>,
}

#[derive(Properties)]
struct DropdownProps<O>
where
    O: Clone + 'static,
{
    pub open: bool,
    pub visible_options: Vec<(usize, O)>,
    pub on_select: Callback<usize, ()>,
    pub lang: Rc<LanguageData>,
}

/// Select dropdown with searchable options.
#[function_component]
pub fn SearchSelect<O>(props: &SearchSelectProps<O>) -> Html
where
    O: HtmlName + Clone + 'static,
{
    let value = use_state(|| props.current);
    let value_state = value.clone();
    use_effect_with_deps(
        move |(_, new)| value_state.set(*new),
        (props.options.id(), props.current),
    );

    let default_search = use_memo(|_| props.search_query(*value), (props.options.id(), *value));

    let search_query = use_state(|| (*default_search).clone());
    let focused = use_state(|| false);

    let search_state = search_query.clone();
    // Refresh search query when current value is changed as a prop
    use_effect_with_deps(
        move |_| search_state.set((*default_search).clone()),
        (props.options.id(), *value),
    );

    if value.is_some_and(|v| v >= props.options.len()) {
        return html!();
    }

    let visible = props
        .options
        .iter()
        .enumerate()
        .filter(|(_, o)| {
            o.get_name_for_filter(&props.lang)
                .is_some_and(|n| n.contains(&**search_query))
        })
        .map(|(i, o)| (i, o.clone()))
        .collect::<Vec<_>>();

    let on_select = props.on_select.clone();
    let value_state = value.clone();
    let select_callback = Callback::from(move |option| {
        value_state.set(Some(option));
        on_select.emit(option);
    });

    let current_display = value
        .map(|v| props.options.get(v).get_name_html(&props.lang))
        .unwrap_or_else(|| html!());

    let search_state = search_query.clone();
    let update_search_query = Callback::from(move |e: InputEvent| {
        let target: Option<EventTarget> = e.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        if let Some(input) = input {
            search_state.set(input.value().to_string().into());
        }
    });

    let update_focus = |has_focus: bool| {
        let focus_state = focused.clone();
        Callback::from(move |_: FocusEvent| {
            focus_state.set(has_focus);
        })
    };

    // Having the dropdown inside form control gives it the correct width

    html! {
        <>
            <Control classes={classes!("has-icons-right")}>
                <input class="input"
                    value={(*search_query).clone()}
                    oninput={update_search_query}
                    onfocus={update_focus(true)}
                    onfocusout={update_focus(false)}
                />
                <Icon classes={classes!("is-right")}>
                    <ChevronDown />
                </Icon>
                <Dropdown<O>
                    open={*focused}
                    visible_options={visible}
                    on_select={select_callback}
                    lang={props.lang.clone()}
                />
            </Control>
        </>
    }
}

#[function_component]
fn Dropdown<O>(props: &DropdownProps<O>) -> Html
where
    O: Clone + HtmlName + 'static,
{
    if !props.open {
        return html!();
    }

    let callback = |index| {
        let on_select = props.on_select.clone();
        Callback::from(move |_: MouseEvent| on_select.emit(index))
    };

    html!(
        <Menu classes={classes!("card", "recordkeeper-dropdown")}>
            <MenuList classes={classes!("recordkeeper-dropdown-list")}>
                {for props.visible_options.iter().map(|(index, item)| {
                    html!(<li><a onclick={callback(*index)}>{item.get_name_html(&props.lang)}</a></li>)
                })}
            </MenuList>
        </Menu>
    )
}

impl<O: 'static> Options<O> {
    fn get(&self, i: usize) -> &O {
        &self.as_slice()[i]
    }

    fn get_if_present(&self, i: usize) -> Option<&O> {
        self.as_slice().get(i)
    }

    fn len(&self) -> usize {
        self.as_slice().len()
    }

    fn iter(&self) -> std::slice::Iter<O> {
        self.as_slice().iter()
    }

    fn id(&self) -> usize {
        self.as_slice().as_ptr() as usize
    }

    fn as_slice(&self) -> &[O] {
        match self {
            Self::Owned(v) => &v,
            Self::Borrowed(s) => &s,
        }
    }
}

impl<O: Clone + HtmlName + 'static> SearchSelectProps<O> {
    fn search_query(&self, current: Option<usize>) -> AttrValue {
        current
            .and_then(|o| {
                self.options
                    .get_if_present(o)?
                    .get_name_for_filter(&self.lang.clone())
                    .map(|s| AttrValue::from(s.to_string()))
            })
            .unwrap_or_default()
    }
}

impl<O: 'static> PartialEq for Options<O> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Owned(s1), Self::Owned(s2)) => Rc::ptr_eq(s1, s2),
            (Self::Borrowed(s1), Self::Borrowed(s2)) => std::ptr::eq(*s1, *s2),
            _ => false,
        }
    }
}

impl<O: 'static> From<&'static [O]> for Options<O> {
    fn from(value: &'static [O]) -> Self {
        Self::Borrowed(value)
    }
}

impl<O: 'static> From<Rc<[O]>> for Options<O> {
    fn from(value: Rc<[O]>) -> Self {
        Self::Owned(value)
    }
}

impl<O: 'static> FromIterator<O> for Options<O> {
    fn from_iter<T: IntoIterator<Item = O>>(iter: T) -> Self {
        Self::Owned(iter.into_iter().collect())
    }
}

impl<O: Clone + 'static> PartialEq for SearchSelectProps<O> {
    fn eq(&self, other: &Self) -> bool {
        self.on_select == other.on_select
            && self.options == other.options
            && self.current == other.current
            && Rc::ptr_eq(&self.lang, &other.lang)
    }
}

impl<O: Clone + 'static> PartialEq for DropdownProps<O> {
    fn eq(&self, other: &Self) -> bool {
        self.open == other.open
            && self.on_select == other.on_select
            && std::ptr::eq(
                self.visible_options.as_ptr(),
                other.visible_options.as_ptr(),
            )
            && Rc::ptr_eq(&self.lang, &other.lang)
    }
}

impl<T> HtmlName for T
where
    T: Nameable,
{
    fn get_name_html(&self, language: &LanguageData) -> Html {
        html!(<>{self.get_name(language)}</>)
    }

    fn get_name_for_filter<'a, 'b: 'a>(&'a self, language: &'b LanguageData) -> Option<&'a str> {
        self.get_name(language)
    }
}
