use strum::{EnumIter, IntoEnumIterator};
use ybc::{Control, Field, Notification, Tabs, Tile};
use yew::prelude::*;

use crate::{
    components::character::formation::{FormationCharacters, FormationOuroboros},
    lang::Text,
};

#[derive(Clone, Copy, PartialEq, EnumIter)]
enum Tab {
    Characters,
    Ouroboros,
}

#[derive(PartialEq, Properties)]
pub struct FormationProps {
    pub id: usize,
}

#[function_component]
pub fn Formations() -> Html {
    // TODO: formation list
    html! {
        <FormationEditor id={0} />
    }
}

#[function_component]
fn FormationEditor(props: &FormationProps) -> Html {
    let tab = use_state(|| Tab::Characters);

    let update_tab = |t| {
        let tab_state = tab.clone();
        Callback::from(move |_: MouseEvent| {
            tab_state.set(t);
        })
    };

    html! {
        <>
            <Tabs classes={classes!("is-boxed", "is-centered")}>
                {for Tab::iter().map(|t| {
                    let classes = if t == *tab { classes!("is-active") } else { classes!() };
                    html!(<li class={classes}><a onclick={update_tab(t)}><Text path={t.lang()} /></a></li>)
                })}
            </Tabs>

            {match *tab {
                Tab::Characters => html!(<FormationCharacters id={props.id} />),
                Tab::Ouroboros => html!(<FormationOuroboros id={props.id} />),
            }}
        </>
    }
}

impl Tab {
    fn lang(&self) -> String {
        format!(
            "formation_tab_{}",
            match self {
                Tab::Characters => "characters",
                Tab::Ouroboros => "ouroboros",
            }
        )
    }
}
