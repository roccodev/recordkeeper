use ybc::{Icon, NavbarDropdown, NavbarFixed, NavbarItem, Size};
use yew::prelude::*;
use yew_feather::{Github, Info, Key};
use yew_router::prelude::Link;

use crate::{
    data::Data,
    lang::{Lang, LangMeta, Text},
    routes::Route,
    BRAND_NAME, GITHUB_URL, GIT_SHA, LICENSE_URL,
};

#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    pub game_lang_callback: Callback<String>,
    pub ui_lang_callback: Callback<String>,
}

#[derive(Properties, PartialEq)]
struct IconTextProps {
    icon: Html,
    name: Html,
}

#[derive(Properties, PartialEq)]
struct LangSelectProps {
    ui: bool,
    change_callback: Callback<LangMeta>,
}

#[derive(Properties, PartialEq)]
struct FlagProps {
    id: AttrValue,
}

#[function_component]
pub fn Navbar(props: &NavbarProps) -> Html {
    let ui_lang_cb = props.ui_lang_callback.clone();
    let game_lang_cb = props.game_lang_callback.clone();

    html! {
        <ybc::Navbar fixed={NavbarFixed::Top} classes={classes!("has-shadow")}
            navend={html! {
                <>
                    <LangSelect ui={true} change_callback={Callback::from(move |meta: LangMeta| {
                        ui_lang_cb.emit(meta.lang_id.unwrap());
                    })} />
                    <LangSelect ui={false} change_callback={Callback::from(move |meta: LangMeta| {
                        game_lang_cb.emit(meta.file.to_string());
                    })} />
                    <Brand />
                </>
            }}
        />
    }
}

#[function_component]
fn LangSelect(props: &LangSelectProps) -> Html {
    let lang = use_context::<Lang>().unwrap();
    let data = use_context::<Data>().unwrap();

    let (options, meta, label) = if props.ui {
        (
            &lang.ui_meta,
            lang.ui_meta(),
            html!(<Text path="lang_ui" />),
        )
    } else {
        (
            &lang.game_meta,
            lang.game_meta(data.lang_id()),
            html!(<Text path="lang_game" />),
        )
    };

    let label = html! {
        <span class="is-flex">
            {label}
            {": \u{00a0}"}
            <Flag id={meta.flag.clone()} />
        </span>
    };

    let callback = props.change_callback.clone();

    html! {
        <NavbarItem>
            <NavbarDropdown navlink={label}>
                {for options.iter().map(|meta| {
                    let callback = callback.clone();
                    let cb_meta = meta.clone();
                    html! {
                        <a class={classes!("navbar-item")} onclick={Callback::from(move |_| {
                            callback.emit(cb_meta.clone());
                        })}>
                            <span class="is-flex">
                                <Flag id={meta.flag.clone()} />
                                <span class="ml-2">{&meta.name}</span>
                            </span>
                        </a>
                    }
                })}
            </NavbarDropdown>
        </NavbarItem>
    }
}

#[function_component]
fn Brand() -> Html {
    html! {
        <NavbarItem>
            <NavbarDropdown navlink={html!(<span class="recordkeeper-brand">{BRAND_NAME}{" "}<code>{GIT_SHA}</code></span>)}>
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

#[function_component]
fn Flag(props: &FlagProps) -> Html {
    let cls = format!("fi fi-{}", props.id);
    html!(<span class={cls}>{"\u{00a0}"}</span>)
}
