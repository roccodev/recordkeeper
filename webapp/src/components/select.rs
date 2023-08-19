use game_data::lang::Nameable;
use game_data::LanguageData;
use std::borrow::Cow;
use std::ops::Deref;
use std::rc::Rc;
use ybc::*;
use yew::prelude::*;

#[derive(Clone)]
pub enum Options<O: 'static> {
    Borrowed(&'static [O]),
    Owned(Rc<[O]>),
}

#[derive(Properties)]
pub struct SearchSelectProps<O: Clone + 'static> {
    /// Whether the field is currently focused, meaning
    /// the dropdown should display.
    pub selected: bool,
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
    let value = use_state(|| props.current);
    let search_query = use_state(|| {
        props
            .current
            //.and_then(|o| o.get_name(&lang.clone()))
            .map(|_| "")
            .unwrap_or("")
    });

    let visible = props
        .options
        .iter()
        .filter(|o| {
            o.get_name(&props.lang)
                .is_some_and(|n| n.contains(*search_query))
        })
        .cloned()
        .collect::<Vec<_>>();

    let on_select = props.on_select.clone();
    let value_state = value.clone();
    let select_callback = Callback::from(move |option| {
        on_select.emit(option);
        value_state.set(Some(option));
    });

    let current_display = value
        .and_then(|v| visible[v].get_name(&props.lang))
        .unwrap_or("");

    html! {
        <>
            <Dropdown<O> open={props.selected} visible_options={visible} on_select={select_callback} lang={props.lang.clone()} />
            <span>{current_display}</span>
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
        <Menu>
            <MenuList>
                {for props.visible_options.iter().take(5).map(|item| {
                    html!(
                        <p class="menu-label">
                            {item.get_name(&props.lang)}
                        </p>
                    )
                })}
            </MenuList>
        </Menu>
    )
}

impl<O: 'static> Options<O> {
    fn iter(&self) -> std::slice::Iter<O> {
        match self {
            Self::Owned(v) => v.iter(),
            Self::Borrowed(s) => s.iter(),
        }
    }
}

impl<O: 'static> PartialEq for Options<O> {
    fn eq(&self, other: &Self) -> bool {
        let s1 = match self {
            Self::Owned(v) => v,
            Self::Borrowed(s) => *s,
        };
        let s2 = match other {
            Self::Owned(v) => v,
            Self::Borrowed(s) => *s,
        };
        std::ptr::eq(s1, s2)
    }
}

impl<O: 'static> From<&'static [O]> for Options<O> {
    fn from(value: &'static [O]) -> Self {
        Self::Borrowed(value)
    }
}

impl<O: 'static> FromIterator<O> for Options<O> {
    fn from_iter<T: IntoIterator<Item = O>>(iter: T) -> Self {
        Self::Owned(iter.into_iter().collect())
    }
}

impl<O: Clone + 'static> PartialEq for SearchSelectProps<O> {
    fn eq(&self, other: &Self) -> bool {
        self.selected == other.selected
            && self.on_select == other.on_select
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
