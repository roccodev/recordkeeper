use strum::{EnumIter, IntoEnumIterator};
use ybc::Tabs;
use yew::prelude::*;

use crate::lang::Text;

#[derive(Clone, PartialEq, Copy, EnumIter)]
pub enum ChronoTab {
    Characters,
    Classes,
    Quests,
    Items,
    Npc,
}

#[function_component]
pub fn ChronoPage() -> Html {
    let tab = use_state(|| ChronoTab::Characters);

    let update_tab = |t| {
        let tab_state = tab.clone();
        Callback::from(move |_: MouseEvent| {
            tab_state.set(t);
        })
    };

    html! {
        <>
            <Tabs classes={classes!("is-toggle", "is-centered")}>
                {for ChronoTab::iter().map(|t| {
                    let classes = if t == *tab { classes!("is-active") } else { classes!() };
                    html!(<li class={classes}><a onclick={update_tab(t)}><Text path={t.lang()} /></a></li>)
                })}
            </Tabs>

            {match *tab {
                ChronoTab::Characters => html!(),
                ChronoTab::Classes => html!(),
                ChronoTab::Quests => html!(),
                ChronoTab::Items => html!(),
                ChronoTab::Npc => html!(),
            }}
        </>
    }
}

impl ChronoTab {
    fn lang(&self) -> String {
        let id = match self {
            ChronoTab::Characters => "characters",
            ChronoTab::Classes => "classes",
            ChronoTab::Quests => "quests",
            ChronoTab::Items => "items",
            ChronoTab::Npc => "npc",
        };
        format!("chrono_tab_{id}")
    }
}
