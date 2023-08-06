use ybc::{Button, Icon};
use yew::prelude::*;
use yew_feather::{CornerUpLeft, CornerUpRight, FilePlus, Save};

use crate::lang::Text;

struct Category(String);

#[function_component]
pub fn Sidebar() -> Html {
    html! {
      <aside class="aside is-placed-left is-expanded">
          <div class="aside-tools">
              <div class="aside-tools-label">
                  {edit_operations().collect::<Html>()}
              </div>
          </div>
      </aside>
    }
}

fn edit_operations() -> impl Iterator<Item = Html> {
    let ops = [
        (Some(html!(<Text path="open" />)), html!(<FilePlus />)),
        (Some(html!(<Text path="save" />)), html!(<Save />)),
        (None, html!(<CornerUpLeft />)),  // Undo
        (None, html!(<CornerUpRight />)), // Redo
    ];

    ops.into_iter().map(|(name, icon)| {
        html! {
            <Button>
                <Icon>{icon}</Icon>
                {if let Some(name) = name { html!(<span>{name}</span>) } else { html!() }}
            </Button>
        }
    })
}
