use game_data::lang::Nameable;
use game_data::LanguageData;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use ybc::*;
use yew::prelude::*;
use yew_feather::ChevronDown;

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
    pub visible_options: Vec<O>,
    pub on_select: Callback<usize, ()>,
    pub lang: Rc<LanguageData>,
}

/// Select dropdown with searchable options.
#[function_component]
pub fn SearchSelect<O>(props: &SearchSelectProps<O>) -> Html
where
    O: Nameable + Clone + 'static,
{
    let value = props.current;
    let default_search = use_memo(
        |_| props.default_search_query(),
        (props.options.id(), props.current),
    );

    let search_query = use_state(|| (*default_search).clone());
    let focused = use_state(|| false);

    let search_state = search_query.clone();
    // Refresh search query when current value is changed as a prop
    use_effect_with_deps(
        move |_| search_state.set((*default_search).clone()),
        (props.options.id(), props.current),
    );

    let visible = props
        .options
        .iter()
        .filter(|o| {
            o.get_name(&props.lang)
                .is_some_and(|n| n.contains(&**search_query))
        })
        .cloned()
        .collect::<Vec<_>>();

    let on_select = props.on_select.clone();
    let select_callback = Callback::from(move |option| {
        on_select.emit(option);
    });

    let current_display = value
        .and_then(|v| props.options.get(v).get_name(&props.lang))
        .unwrap_or("");

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
                    onblur={update_focus(false)}
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
    O: Clone + Nameable + 'static,
{
    if !props.open {
        return html!();
    }
    html!(
        <Menu classes={classes!("card", "recordkeeper-dropdown")}>
            <MenuList classes={classes!("recordkeeper-dropdown-list")}>
                {for props.visible_options.iter().map(|item| {
                    html!(<li><a>{item.get_name(&props.lang)}</a></li>)
                })}
            </MenuList>
        </Menu>
    )
}

impl<O: 'static> Options<O> {
    fn get(&self, i: usize) -> &O {
        match self {
            Self::Owned(v) => &v[i],
            Self::Borrowed(s) => &s[i],
        }
    }

    fn iter(&self) -> std::slice::Iter<O> {
        match self {
            Self::Owned(v) => v.iter(),
            Self::Borrowed(s) => s.iter(),
        }
    }

    fn id(&self) -> usize {
        match self {
            Self::Owned(v) => v.as_ptr() as usize,
            Self::Borrowed(s) => s.as_ptr() as usize,
        }
    }
}

impl<O: Clone + Nameable + 'static> SearchSelectProps<O> {
    fn default_search_query(&self) -> AttrValue {
        self.current
            .and_then(|o| {
                self.options
                    .get(o)
                    .get_name(&self.lang.clone())
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
