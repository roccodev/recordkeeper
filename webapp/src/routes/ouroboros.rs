use game_data::ouroboros::Ouroboros;
use ybc::{Control, Field, Tile};
use yew::prelude::*;

use crate::{
    components::{
        ouroboros::{
            ArtEditor, OuroEditorConfig, OuroborosEditor, SaveArt, SaveSkill, SkillEditor,
        },
        select::Selector,
    },
    data::Data,
    lang::Text,
};

#[function_component]
pub fn OuroborosPage() -> Html {
    let char_id_state = use_state(|| 1u32);
    let char_id = *char_id_state;
    let char_idx = char_id.checked_sub(1).unwrap() as usize;

    let data = use_context::<Data>().unwrap();

    html! {
        <>
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
                    skill: Callback::from(move |i| SkillEditor::Save(SaveSkill { char_idx, slot_idx: i })),
                    art: Callback::from(move |i| ArtEditor::Save(SaveArt { char_idx, slot_idx: i })),
                }} />
            </div>
        </>
    }
}
