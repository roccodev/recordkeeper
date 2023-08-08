use ybc::{Button, Icon, NavbarDropdown, NavbarFixed, NavbarItem, Size};
use yew::prelude::*;
use yew_feather::{Download, Github, Info, Key};

use crate::{lang::Text, save::SaveContext, BRAND_DISPLAY};

#[function_component]
pub fn Navbar() -> Html {
    html! {
        <ybc::Navbar fixed={NavbarFixed::Top} classes={classes!("has-shadow")}
            navend={html! {
                <>
                    <DownloadButton />
                    <Brand />
                </>
            }}
        />
    }
}

#[function_component]
fn DownloadButton() -> Html {
    let save = use_context::<SaveContext>().unwrap();
    html! {
        <NavbarItem>
            <Button classes={classes!("is-success")} disabled={!save.get().is_loaded()}>
                <span class="icon-text">
                    <Icon><Download /></Icon>
                    <span><Text path="download" /></span>
                </span>
            </Button>
        </NavbarItem>
    }
}

#[function_component]
fn Brand() -> Html {
    let items = [
        (html!(<Text path="nav_about" />), html!(<Info />)),
        (html!(<Text path="nav_source" />), html!(<Github />)),
        (html!(<Text path="nav_license" />), html!(<Key />)),
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
