use game_data::ouroboros::{OuroTreeNode, Ouroboros};
use ybc::{Control, Field};
use yew::prelude::*;

use crate::{
    components::edit::{editor, CheckboxInput},
    data::Data,
    lang::Text,
};

#[derive(Properties, PartialEq)]
pub struct OuroTreeProps {
    pub ouroboros: &'static Ouroboros,
}

editor!(
    SoulEditor,
    bool,
    get |editor, save| save.ouroboros[editor.char_idx].skill_tree.get(editor.index),
    set |editor, save, new| save.ouroboros[editor.char_idx].skill_tree.set(editor.index, new),
    capture char_idx: usize, index: usize
);

#[function_component]
pub fn OuroTree(props: &OuroTreeProps) -> Html {
    let data = use_context::<Data>().unwrap();
    let char_idx = props.ouroboros.id.checked_sub(1).unwrap();

    html! {
        <Field classes={classes!("is-grouped", "is-grouped-multiline")}>
            {for props.ouroboros.tree_nodes().map(|(index, node)| html! {
                <Control>
                    <CheckboxInput<SoulEditor> editor={SoulEditor { char_idx, index: index.checked_sub(1).unwrap() }}>
                        {" "}
                        <b>{soul_lang(&node)}</b>
                        {": "}
                        {node.get_param_name(data.game(), data.lang())}
                    </CheckboxInput<SoulEditor>>
                </Control>
            })}
        </Field>
    }
}

fn soul_lang(node: &OuroTreeNode) -> Html {
    let path = match node {
        OuroTreeNode::UnlockArt(_) => "art_unlock",
        OuroTreeNode::UnlockSkill(_) => "skill_unlock",
        OuroTreeNode::UpgradeArt(_) => "art_upgrade",
        OuroTreeNode::UpgradeSkill(_) => "skill_upgrade",
    };
    html!(<Text path={format!("ouroboros_tree_{path}")} />)
}
