use ybc::{Button, Icon, Menu, MenuList, Size};
use yew::prelude::*;
use yew_feather::{
    BookOpen, Clock, Crosshair, Download, FilePlus, Flag, HelpCircle, Info, LifeBuoy, Map,
    ShoppingBag, Target, TrendingUp, Triangle, Users, Watch,
};
use yew_router::prelude::{use_route, Link};

use crate::{
    components::upload::UploadButton,
    lang::Text,
    routes::Route,
    save::{EditAction, SaveContext},
};

struct Category(&'static str);

struct Tab(&'static str, Html, Route);

enum MenuItem {
    Category(Category),
    Tabs(Vec<Tab>),
    None,
}

#[function_component]
pub fn Sidebar() -> Html {
    let save = use_context::<SaveContext>().unwrap();
    let route = use_route::<Route>();
    let save = save.get();

    let is_dlc4 = save.is_loaded() && save.get().save().is_dlc4();

    let mut base_menu = vec![
        Tab("base_characters", html!(<Users />), Route::Characters),
        // Ouroboros here if not DLC4
        Tab("base_items", html!(<ShoppingBag />), Route::Items),
        Tab("base_field", html!(<Map />), Route::Field),
        Tab("base_quests", html!(<HelpCircle />), Route::Quests),
        Tab("base_ums", html!(<Crosshair />), Route::Uniques),
        Tab("base_chrono", html!(<Clock />), Route::ChronoData),
        // Party formations if not DLC4
    ];

    if !is_dlc4 {
        base_menu.insert(
            1,
            Tab("base_ouroboros", html!(<Target />), Route::Ouroboros),
        );
        base_menu.push(Tab(
            "base_formations",
            html!(<Triangle />),
            Route::Formations,
        ));
    }

    let menu = [
        MenuItem::Category(Category("meta")),
        MenuItem::Tabs(vec![Tab("meta_meta", html!(<Info />), Route::Meta)]),
        MenuItem::Category(Category("base")),
        MenuItem::Tabs(base_menu),
        if !is_dlc4 {
            MenuItem::Category(Category("dlc"))
        } else {
            MenuItem::None
        },
        if !is_dlc4 {
            MenuItem::Tabs(vec![
                Tab("dlc_powaugment", html!(<TrendingUp />), Route::PowAugment),
                Tab("dlc_challenge", html!(<Watch />), Route::ChallengeBattle),
                Tab("dlc_gauntlet", html!(<LifeBuoy />), Route::Gauntlet),
            ])
        } else {
            MenuItem::None
        },
        if is_dlc4 {
            MenuItem::Category(Category("dlc4"))
        } else {
            MenuItem::None
        },
        if is_dlc4 {
            MenuItem::Tabs(vec![
                Tab("dlc4_growth", html!(<TrendingUp />), Route::PowAugment),
                Tab(
                    "dlc4_collepedia",
                    html!(<BookOpen />),
                    Route::Dlc4Collepedia,
                ),
                Tab(
                    "dlc4_enemypedia",
                    html!(<BookOpen />),
                    Route::Dlc4Enemypedia,
                ),
            ])
        } else {
            MenuItem::None
        },
        MenuItem::Category(Category("danger")),
        MenuItem::Tabs(vec![Tab("danger_flags", html!(<Flag />), Route::Flags)]),
    ];

    html! {
      <aside class="aside is-placed-left is-expanded">
            <div class="aside-tools">
                <div class="aside-tools-label buttons">
                    <UploadButton>
                        <Icon><FilePlus /></Icon>
                        <span><Text path="open" /></span>
                    </UploadButton>
                    <DownloadButton />
                </div>
            </div>
            {if save.is_loaded() {
                html!(<Menu>{menu.into_iter().map(|i| i.into_html(route)).collect::<Html>()}</Menu>)
            } else {
                html!()
            }}
      </aside>
    }
}

#[function_component]
fn DownloadButton() -> Html {
    let save = use_context::<SaveContext>().unwrap();

    let save_context = save.clone();
    let on_click = Callback::from(move |_: MouseEvent| {
        save_context.submit_action(EditAction::Save);
        save_context.submit_action(EditAction::Download);
    });

    html! {
        <Button classes={classes!("is-info")} disabled={!save.get().is_loaded()} onclick={on_click}>
            <span class="icon-text">
                <Icon><Download /></Icon>
                <span><Text path="download" /></span>
            </span>
        </Button>
    }
}

impl Tab {
    fn into_html(self, current_route: Option<Route>) -> Html {
        let classes = if current_route.is_some_and(|r| r == self.2) {
            "is-active"
        } else {
            ""
        };
        html! {
            <li>
                <Link<Route> to={self.2} classes={classes!(classes)}>
                    <span class="icon-text">
                        <Icon size={Size::Small}>{self.1}</Icon>
                        <span><Text path={format!("menu_{}", self.0)} /></span>
                    </span>
                </Link<Route>>
            </li>
        }
    }
}

impl MenuItem {
    fn into_html(self, current_route: Option<Route>) -> Html {
        match self {
            MenuItem::Category(c) => c.into(),
            MenuItem::Tabs(tabs) => html! {
                <MenuList>
                    {tabs.into_iter().map(|t| t.into_html(current_route)).collect::<Html>()}
                </MenuList>
            },
            MenuItem::None => html!(),
        }
    }
}

impl From<Category> for Html {
    fn from(value: Category) -> Self {
        html! {
            <p class="menu-label">
                <Text path={format!("menu_category_{}", value.0)} />
            </p>
        }
    }
}
