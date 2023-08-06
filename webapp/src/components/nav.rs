use ybc::{Button, Icon, NavbarDropdown, NavbarFixed, NavbarItem, Size};
use yew::prelude::*;
use yew_feather::{CornerUpLeft, CornerUpRight, FilePlus, Github, Info, Key, Save};

use crate::BRAND_DISPLAY;

#[function_component]
pub fn Navbar() -> Html {
    html! {
        <ybc::Navbar fixed={NavbarFixed::Top}
            navend={html!(<Brand />)}
            navstart={html!{
                <>
                {edit_operations().collect::<Html>()}
                </>
            }}
        />
    }
}

#[function_component]
fn Brand() -> Html {
    let items = [
        ("About", html!(<Info />)),
        ("Source", html!(<Github />)),
        ("License", html!(<Key />)),
    ];

    html! {
        <NavbarItem>
            <NavbarDropdown navlink={html!(BRAND_DISPLAY)}>
                {items.into_iter().map(|(name, icon)| html! {
                    <NavbarItem>
                        <span class="icon-text">
                            <Icon size={Size::Small}>{icon}</Icon>
                            <span>{name}</span>
                        </span>
                    </NavbarItem>
                }).collect::<Html>()}
            </NavbarDropdown>
        </NavbarItem>
    }
}

fn edit_operations() -> impl Iterator<Item = Html> {
    let ops = [
        ("Open", html!(<FilePlus />)),
        ("Save", html!(<Save />)),
        ("Undo", html!(<CornerUpLeft />)),
        ("Redo", html!(<CornerUpRight />)),
    ];

    ops.into_iter().map(|(name, icon)| {
        html! {
            <Button>
                <Icon>{icon}</Icon>
            </Button>
        }
    })
}
