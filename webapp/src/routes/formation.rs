use recordkeeper::character::formation::PARTY_FORMATION_MAX;
use strum::{EnumIter, IntoEnumIterator};
use ybc::{Tabs, Tile};
use yew::prelude::*;

use crate::{
    components::character::formation::{
        FormationCardEmpty, FormationCardPresent, FormationCharacters, FormationOuroboros,
    },
    lang::Text,
    save::SaveContext,
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

const CARDS_PER_ROW: usize = 5;

#[function_component]
pub fn Formations() -> Html {
    let current = use_state(|| None::<usize>);
    let save = use_context::<SaveContext>().unwrap();
    let save = save.get();

    if let Some(current) = *current {
        html! {
            <>
                <Text path="formation_back" />
                <FormationEditor id={current} />
            </>
        }
    } else {
        let child_classes = classes!("is-child", "pr-2");

        html! {
            <Tile classes="is-vertical">
                {for (0..PARTY_FORMATION_MAX).step_by(CARDS_PER_ROW).map(|start| {
                    let end = (start + CARDS_PER_ROW).min(PARTY_FORMATION_MAX);
                    html! {
                        <Tile classes="is-parent">
                            {for (start..end).map(|i| if save.get().save().party_formations[i].is_valid() {
                                html!(<Tile classes={child_classes.clone()}><FormationCardPresent id={i} /></Tile>)
                            } else {
                                html!(<Tile classes={child_classes.clone()}><FormationCardEmpty id={i} /></Tile>)
                            })}
                        </Tile>
                    }
                })}
            </Tile>
        }
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
