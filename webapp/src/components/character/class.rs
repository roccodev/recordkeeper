use game_data::character::Class;
use ybc::{Control, Field, Tile};
use yew::prelude::*;

use crate::{components::character::Selector, data::Data, lang::Text};

use super::CharacterProps;

#[derive(Properties, PartialEq)]
pub struct ClassProps {
    pub char_id: usize,
    pub class_id: usize,
}

#[function_component]
pub fn ClassEditor(props: &CharacterProps) -> Html {
    let data = use_context::<Data>().unwrap();
    let class_id = use_state(|| 1); // TODO use selected class
    html! {
        <>
            <Tile>
                <Field>
                    <label class="label"><Text path="character_class" /></label>
                    <Control>
                        <Selector<Class> state={class_id.clone()} values={data.game().characters.classes()} />
                    </Control>
                </Field>
            </Tile>
            <Tile>
                <Tile classes={classes!("is-parent")}>
                    {"Arts"}
                </Tile>
                <Tile classes={classes!("is-parent")}>
                    {"Skills"}
                </Tile>
                <Tile classes={classes!("is-parent")}>
                    {"Gems"}
                </Tile>
                <Tile classes={classes!("is-parent")}>
                    {"Accessories"}
                </Tile>
            </Tile>
        </>
    }
}
