use ybc::{Icon, NavbarDropdown, NavbarFixed, NavbarItem, Size};
use yew::prelude::*;
use yew_feather::{Github, Info, Key};

use crate::BRAND_DISPLAY;

#[function_component]
pub fn Navbar() -> Html {
    html! {
        <ybc::Navbar fixed={NavbarFixed::Top}
            navend={html!(<Brand />)}
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
