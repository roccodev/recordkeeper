use ybc::{Button, Icon};
use yew::prelude::*;
use yew_feather::{CornerUpLeft, CornerUpRight, FilePlus, Save};

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
        ("Open", html!(<FilePlus />)),
        ("Save", html!(<Save />)),
        ("", html!(<CornerUpLeft />)),  // Undo
        ("", html!(<CornerUpRight />)), // Redo
    ];

    ops.into_iter().map(|(name, icon)| {
        html! {
            <Button>
                <Icon>{icon}</Icon>
                {if !name.is_empty() { html!(<span>{name}</span>) } else { html!() }}
            </Button>
        }
    })
}
