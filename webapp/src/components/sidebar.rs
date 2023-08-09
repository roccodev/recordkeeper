use ybc::{Button, Icon, Menu, MenuList, Size};
use yew::prelude::*;
use yew_feather::{
    BookOpen, CornerUpLeft, CornerUpRight, Crosshair, FilePlus, Flag, HelpCircle, Info, LifeBuoy,
    Map, Save, Scissors, ShoppingBag, Target, Triangle, Users, Watch,
};
use yew_router::prelude::{use_route, Link};

use crate::{
    components::upload::UploadButton,
    lang::Text,
    routes::Route,
    save::{SaveContext, SaveManager},
};

struct Category(&'static str);

struct Tab(&'static str, Html, Route);

enum MenuItem {
    Category(Category),
    Tabs(Vec<Tab>),
}

#[function_component]
pub fn Sidebar() -> Html {
    let menu = [
        MenuItem::Category(Category("meta")),
        MenuItem::Tabs(vec![Tab("meta_meta", html!(<Info />), Route::Meta)]),
        MenuItem::Category(Category("base")),
        MenuItem::Tabs(vec![
            Tab("base_characters", html!(<Users />), Route::Characters),
            Tab("base_ouroboros", html!(<Target />), Route::Ouroboros),
            Tab("base_items", html!(<ShoppingBag />), Route::Items),
            Tab("base_field", html!(<Map />), Route::Field),
            Tab("base_quests", html!(<HelpCircle />), Route::Quests),
            Tab("base_ums", html!(<Crosshair />), Route::Uniques),
            Tab("base_formations", html!(<Triangle />), Route::Formations),
        ]),
        MenuItem::Category(Category("dlc")),
        MenuItem::Tabs(vec![
            Tab("dlc_challenge", html!(<Watch />), Route::ChallengeBattle),
            Tab("dlc_gauntlet", html!(<LifeBuoy />), Route::Gauntlet),
            Tab("dlc_masha", html!(<Scissors />), Route::Masha),
        ]),
        MenuItem::Category(Category("dlc4")),
        MenuItem::Tabs(vec![Tab(
            "dlc4_enemypedia",
            html!(<BookOpen />),
            Route::Dlc4Enemypedia,
        )]),
        MenuItem::Category(Category("danger")),
        MenuItem::Tabs(vec![Tab("danger_flags", html!(<Flag />), Route::Flags)]),
    ];

    let save = use_context::<SaveContext>().unwrap();
    let route = use_route::<Route>();

    html! {
      <aside class="aside is-placed-left is-expanded">
          <div class="aside-tools">
              <div class="aside-tools-label buttons">
                  {edit_operations(&save.get()).collect::<Html>()}
              </div>
          </div>
          <Menu>
            {menu.into_iter().map(|i| i.into_html(route)).collect::<Html>()}
          </Menu>
      </aside>
    }
}

fn edit_operations(save: &SaveManager) -> impl Iterator<Item = Html> {
    let ops = [
        (
            Some(html!(<Text path="save" />)),
            html!(<Save />),
            "is-info",
            save.is_loaded(),
        ),
        (None, html!(<CornerUpLeft />), "is-light", false), // Undo
        (None, html!(<CornerUpRight />), "is-light", false), // Redo
    ];

    std::iter::once(html! {
        <UploadButton>
            <Icon><FilePlus /></Icon>
            <span><Text path="open" /></span>
        </UploadButton>
    })
    .chain(ops.into_iter().map(|(name, icon, style, enabled)| {
        html! {
            <Button classes={classes!(style)} disabled={!enabled}>
                <Icon>{icon}</Icon>
                {if let Some(name) = name { html!(<span>{name}</span>) } else { html!() }}
            </Button>
        }
    }))
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
