use ybc::{Icon, NavbarDropdown, NavbarFixed, NavbarItem, Size};
use yew::prelude::*;
use yew_feather::{Github, Info, Key};
use yew_router::prelude::Link;

use crate::{lang::Text, routes::Route, BRAND_DISPLAY, GITHUB_URL, LICENSE_URL};

#[derive(Properties, PartialEq)]
struct IconTextProps {
    icon: Html,
    name: Html,
}

#[function_component]
pub fn Navbar() -> Html {
    html! {
        <ybc::Navbar fixed={NavbarFixed::Top} classes={classes!("has-shadow")}
            navend={html! {
                <>
                    <Brand />
                </>
            }}
        />
    }
}

#[function_component]
fn Brand() -> Html {
    html! {
        <NavbarItem>
            <NavbarDropdown navlink={html!(BRAND_DISPLAY)}>
                <Link<Route> classes={classes!("navbar-item")} to={Route::About}>
                    <IconText icon={html!(<Info />)} name={html!(<Text path="nav_about" />)} />
                </Link<Route>>
                <a class={classes!("navbar-item")} href={GITHUB_URL} target="_blank">
                    <IconText icon={html!(<Github />)} name={html!(<Text path="nav_source" />)} />
                </a>
                <a class={classes!("navbar-item")} href={LICENSE_URL} target="_blank">
                    <IconText icon={html!(<Key />)} name={html!(<Text path="nav_license" />)} />
                </a>
            </NavbarDropdown>
        </NavbarItem>
    }
}

#[function_component]
fn IconText(props: &IconTextProps) -> Html {
    let IconTextProps { icon, name } = props;
    html!(
        <span class="icon-text">
            <Icon size={Size::Small}>{icon.clone()}</Icon>
            <span>{name.clone()}</span>
        </span>
    )
}
