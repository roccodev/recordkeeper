use game_data::ouroboros::Ouroboros;
use ybc::{Container, Control, Field, Tile};
use yew::prelude::*;

use crate::{
    components::{
        character::Selector,
        ouroboros::{
            ArtEditor, OuroEditorConfig, OuroborosEditor, SaveArt, SaveSkill, SkillEditor,
        },
    },
    data::Data,
    lang::Text,
};

#[function_component]
pub fn OuroborosPage() -> Html {
    let char_id_state = use_state(|| 1);
    let char_id = *char_id_state;

    let data = use_context::<Data>().unwrap();

    html! {
        <Container>
            <Tile classes={classes!("mb-2")}>
                <Field>
                    <label class="label"><Text path="ouroboros_character" /></label>
                    <Control>
                        <Selector<Ouroboros> state={char_id_state.clone()} values={data.game().ouroboros.as_slice()} />
                    </Control>
                </Field>
            </Tile>
            <div>
                <OuroborosEditor char_id={char_id} config={OuroEditorConfig {
                    sp: true,
                    flags: true,
                    tree: true,
                    skill: Callback::from(move |i| SkillEditor::Save(SaveSkill { char_idx: char_id, slot_idx: i })),
                    art: Callback::from(move |i| ArtEditor::Save(SaveArt { char_idx: char_id, slot_idx: i })),
                }} />
            </div>
        </Container>
    }
}
