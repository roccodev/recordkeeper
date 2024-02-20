use std::num::NonZeroUsize;

use game_data::dlc::pow_augment::{AugmentNode, PowAugment};
use ybc::{Control, Field};
use yew::prelude::*;

use crate::{
    components::edit::{editor, CheckboxInput, NumberInput},
    data::Data,
    lang::Text,
    save::SaveContext,
};

#[derive(Properties, PartialEq)]
pub struct PowAugmentProps {
    pub char_id: u8,
    pub pow_augment: &'static PowAugment,
}

editor!(
    NodeEditor,
    bool,
    get |editor, save| save.pow_augment[editor.char_idx].is_learned(editor.index),
    set |editor, save, new| save.pow_augment[editor.char_idx].set_learned(editor.index, new),
    capture char_idx: usize, index: NonZeroUsize
);

editor!(
    TiersEditor,
    u8,
    get |editor, save| save.pow_augment[editor.char_idx].unlocked_tiers,
    set |editor, save, new| save.pow_augment[editor.char_idx].unlocked_tiers = new,
    capture char_idx: usize
);

#[function_component]
pub fn PowAugmentEditor(props: &PowAugmentProps) -> Html {
    let data = use_context::<Data>().unwrap();
    let save = use_context::<SaveContext>().unwrap();
    let mut char_id = props.char_id;
    if char_id == 43 {
        // Hack: Na'el 2 has a PowAugment table, but no slot in the save file
        char_id = 42;
    }
    let char_idx = save
        .get()
        .get_save()
        .pow_augment
        .iter()
        .position(|p| p.chr_id == char_id)
        .expect("no pow augment slot");
    html! {
        <>
            <Field>
                <Control>
                    <label class="label"><Text path="pow_augment_tiers" /></label>
                    <NumberInput<TiersEditor> editor={TiersEditor { char_idx }} />
                </Control>
            </Field>
            <Field classes={classes!("is-grouped", "is-grouped-multiline")}>
                {for props.pow_augment.nodes.iter().enumerate().map(|(index, node)| html! {
                    <Control>
                        <CheckboxInput<NodeEditor> editor={NodeEditor { char_idx, index: NonZeroUsize::new(index.checked_add(1).unwrap()).unwrap() }}>
                            {" "}
                            <b>{node_lang(node)}</b>
                            {": "}
                            {node.get_param_name(data.game(), data.lang())}
                        </CheckboxInput<NodeEditor>>
                    </Control>
                })}
            </Field>
        </>

    }
}

fn node_lang(node: &AugmentNode) -> Html {
    let path = match node {
        AugmentNode::UnlockArt(_) => "art_unlock",
        AugmentNode::UnlockSkill(_) => "skill_unlock",
        AugmentNode::UpgradeArt(_) => "art_upgrade",
        AugmentNode::UpgradeSkill(_) => "skill_upgrade",
    };
    html!(<Text path={format!("pow_augment_node_{path}")} />)
}
