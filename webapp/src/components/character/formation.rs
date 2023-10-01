use game_data::{character::Character, formation::FormationNameProfile, ouroboros::Ouroboros};
use recordkeeper::character::formation::{FormationName, PartyFormation};
use ybc::{Card, CardContent, CardFooter, Container, Control, Field, Notification, Tile};
use yew::prelude::*;

use crate::{
    components::{
        character::{
            appearance::Appearance, class::ClassEditor, CharacterAccessor, Selector, UpdateSelector,
        },
        edit::{editor, Editor, NumberInput},
        ouroboros::{self, OuroEditorConfig, OuroborosEditor},
    },
    data::Data,
    lang::Text,
    routes::formation::FormationProps,
    save::SaveContext,
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

#[rustfmt::skip]
editor!(
    pub FormationNameId,
    u16,
    get |editor, save| save.party_formations[editor.formation].name.name_id,
    set |editor, save, new| save.party_formations[editor.formation].name.name_id = new,
    capture formation: usize
);

#[rustfmt::skip]
editor!(
    pub FormationNameNum,
    u16,
    get |editor, save| save.party_formations[editor.formation].name.number,
    set |editor, save, new| save.party_formations[editor.formation].name.number = new,
    capture formation: usize
);

#[rustfmt::skip]
editor!(
    pub FormationNameColor,
    u16,
    get |editor, save| save.party_formations[editor.formation].name.color_id,
    set |editor, save, new| save.party_formations[editor.formation].name.color_id = new,
    capture formation: usize
);

#[function_component]
pub fn FormationCharacters(props: &FormationProps) -> Html {
    let char_id_state = use_state(|| 1usize);
    let char_id = *char_id_state;
    let data = use_context::<Data>().unwrap();

    let accessor = CharacterAccessor::Formation {
        formation: props.id,
        id: char_id as u16,
    };

    html! {
        <Container>
            <Tile classes={classes!("mb-2")}>
                <Tile>
                    <Field>
                        <label class="label"><Text path="character_character" /></label>
                        <Control>
                            <Selector<Character> state={char_id_state.clone()} values={data.game().characters.characters()} />
                        </Control>
                    </Field>
                </Tile>
                <Tile classes={classes!("is-10", "is-justify-content-right")}>
                    {"<PartyEditor />"}
                </Tile>
            </Tile>
            <div>
                <Notification>
                    <Tile classes={classes!("notification")}>
                        <Appearance accessor={accessor} char_id={char_id} />
                    </Tile>
                </Notification>
                <Notification>
                    <ClassEditor accessor={accessor.into_class(0)} stats={false} />
                </Notification>
            </div>
        </Container>
    }
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

#[function_component]
pub fn FormationCardEmpty(props: &FormationProps) -> Html {
    let save_context = use_context::<SaveContext>().unwrap();

    let new_callback = {
        let save_context = save_context.clone();
        let id = props.id;
        Callback::from(move |_: MouseEvent| {
            save_context.edit(move |save| {
                save.party_formations[id] =
                    PartyFormation::from_save(save, FormationName::default())
            })
        })
    };

    html! {
        <Card>
            <CardContent>
                <Text path="formation_empty" />
            </CardContent>
            <CardFooter>
                <a class="card-footer-item" onclick={new_callback}><Text path="formation_create" /></a>
                <a class="card-footer-item"><Text path="formation_copy" /></a>
            </CardFooter>
        </Card>
    }
}

#[function_component]
pub fn FormationCardPresent(props: &FormationProps) -> Html {
    let data = use_context::<Data>().unwrap();
    let save_context = use_context::<SaveContext>().unwrap();

    let name_editor = FormationNameId {
        formation: props.id,
    };
    let name_callback = {
        let save_context = save_context.clone();
        Callback::from(move |id: usize| {
            save_context.edit(move |save| name_editor.set(save, id.try_into().unwrap()))
        })
    };

    html! {
        <Card>
            <CardContent>
                <Field>
                    <Control>
                        <UpdateSelector<FormationNameProfile>
                            values={data.game().formation.names.as_ref()}
                            update={name_callback}
                            current={name_editor.get(save_context.get().get().save()) as usize}
                        />
                    </Control>
                </Field>
                <Field>
                    <Control>
                        <NumberInput<FormationNameNum> editor={FormationNameNum { formation: props.id }} />
                    </Control>
                </Field>
            </CardContent>
            <CardFooter>
                <a class="card-footer-item"><Text path="formation_edit" /></a>
                <a class="card-footer-item"><Text path="formation_delete" /></a>
            </CardFooter>
        </Card>
    }
}
