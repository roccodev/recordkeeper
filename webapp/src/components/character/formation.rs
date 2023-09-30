use game_data::ouroboros::Ouroboros;
use ybc::{Container, Control, Field, Tile};
use yew::prelude::*;

use crate::{
    components::{
        character::Selector,
        edit::editor,
        ouroboros::{self, OuroEditorConfig, OuroborosEditor},
    },
    data::Data,
    lang::Text,
    routes::formation::FormationProps,
};

#[rustfmt::skip]
editor!(
    pub FormationOuroArt,
    Option<u16>,
    get |editor, save| save.party_formations[editor.formation].ouroboros(editor.char_id)?.art_slot(editor.slot_idx).get(),
    set |editor, save, new| save.party_formations[editor.formation].ouroboros_mut(editor.char_id).art_slot_mut(editor.slot_idx).set(new),
    capture formation: usize, char_id: u16, slot_idx: usize
);

#[rustfmt::skip]
editor!(
    pub FormationOuroSkill,
    Option<u16>,
    get |editor, save| save.party_formations[editor.formation].ouroboros(editor.char_id)?.linked_skill_slot(editor.slot_idx).get(),
    set |editor, save, new| save.party_formations[editor.formation].ouroboros_mut(editor.char_id).linked_skill_slot_mut(editor.slot_idx).set(new),
    capture formation: usize, char_id: u16, slot_idx: usize
);

#[function_component]
pub fn FormationCharacters(props: &FormationProps) -> Html {
    html! {}
}

#[function_component]
pub fn FormationOuroboros(props: &FormationProps) -> Html {
    let formation = props.id;
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
                    sp: false,
                    flags: false,
                    tree: false,
                    skill: Callback::from(move |i| ouroboros::SkillEditor::Formation(FormationOuroSkill { formation, char_id: char_id as u16, slot_idx: i })),
                    art: Callback::from(move |i| ouroboros::ArtEditor::Formation(FormationOuroArt { formation, char_id: char_id as u16, slot_idx: i })),
                }} />
            </div>
        </Container>
    }
}
