use ybc::Button;
use yew::prelude::*;

use crate::{components::edit::Editor, save::SaveContext};

#[derive(PartialEq, Properties)]
pub struct ColorProps<E>
where
    E: PartialEq,
{
    pub colors: &'static [u32],
    pub editor: E,
}

#[function_component]
pub fn ColorList<E>(props: &ColorProps<E>) -> Html
where
    E: Editor<Target = usize> + PartialEq,
{
    let save_context = use_context::<SaveContext>().unwrap();

    let current = props.editor.get(save_context.get().get_save());
    let current_color = props.colors[current];

    let back = current.wrapping_sub(1).clamp(
        0,
        props.colors.len().checked_sub(1).expect("empty color list"),
    );
    let next = current.wrapping_add(1) % props.colors.len();

    let update_color = |next: usize| {
        let save_context = save_context.clone();
        let editor = props.editor;
        Callback::from(move |_: MouseEvent| save_context.edit(move |save| editor.set(save, next)))
    };

    html! {
        <div class={classes!("is-flex", "is-align-items-center")}>
            <Button onclick={update_color(back)}>
                {"<"}
            </Button>
            // Color visualizer
            <div style={format!("background-color: #{:06x};", current_color)} class={classes!("recordkeeper-collst-color", "ml-2", "mr-2")} />
            <Button onclick={update_color(next)}>
                {">"}
            </Button>
        </div>
    }
}
