use std::{borrow::Cow, rc::Rc};

use web_sys::HtmlSelectElement;
use yew::prelude::*;

use game_data::LanguageData;

use crate::lang::LangManager;

mod search;
mod update;

pub use search::*;
pub use update::*;

pub trait HtmlName {
    fn get_name_html(&self, language: &LanguageData) -> Html;
    fn get_search_query<'a, 'b: 'a>(
        &'a self,
        language: &'b LanguageData,
        ui_lang: &'b LangManager,
    ) -> Option<Cow<'a, str>>;
    fn get_name_for_filter<'a, 'b: 'a>(
        &'a self,
        language: &'b LanguageData,
        ui_lang: &'b LangManager,
    ) -> Option<Cow<'a, str>>;
}

#[derive(Clone)]
pub enum Options<O: 'static> {
    Borrowed(&'static [O]),
    Owned(Rc<[O]>),
}

#[derive(Properties, PartialEq)]
pub struct HtmlSelectProps {
    pub selected_idx: usize,
    pub value: AttrValue,
    pub on_change: Callback<String>,
    pub children: Children,
}

/// Easier to use `<select>`, with a workaround for a Firefox bug.
#[function_component]
pub fn HtmlSelect(props: &HtmlSelectProps) -> Html {
    let select_ref = use_node_ref();

    let update = {
        let update = props.on_change.clone();
        Callback::from(move |s: String| update.emit(s)).reform(|ev: web_sys::Event| {
            let select: HtmlSelectElement = ev
                .target_dyn_into()
                .expect("event target should be a select");
            select.value()
        })
    };

    let selected_index = props.selected_idx;

    // Firefox workaround. For some reason, sometimes the
    // select element fails to update its selectedIndex property
    // when the DOM is refreshed
    {
        let select_ref = select_ref.clone();
        use_effect(move || {
            let select: HtmlSelectElement = select_ref.cast().unwrap();
            select.set_selected_index(selected_index.try_into().unwrap());
        });
    }

    html! {
        <div class="select">
            <select ref={select_ref} name="selector" onchange={update} value={props.value.to_string()}>
                {for props.children.clone()}
            </select>
        </div>
    }
}
