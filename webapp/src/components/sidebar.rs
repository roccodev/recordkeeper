use ybc::{Button, Icon, Menu, MenuList, Size};
use yew::prelude::*;
use yew_feather::{
    BookOpen, CornerUpLeft, CornerUpRight, Crosshair, FilePlus, Flag, HelpCircle, Info, LifeBuoy,
    Map, Save, Scissors, ShoppingBag, Target, Triangle, Users, Watch,
};

use crate::{
    components::upload::UploadButton,
    lang::Text,
    save::{SaveContext, SaveManager},
};

struct Category(&'static str);

struct Tab(&'static str, Html);

enum MenuItem {
    Category(Category),
    Tabs(Vec<Tab>),
}

#[function_component]
pub fn Sidebar() -> Html {
    let menu = [
        MenuItem::Category(Category("meta")),
        MenuItem::Tabs(vec![Tab("meta_meta", html!(<Info />))]),
        MenuItem::Category(Category("base")),
        MenuItem::Tabs(vec![
            Tab("base_characters", html!(<Users />)),
            Tab("base_ouroboros", html!(<Target />)),
            Tab("base_items", html!(<ShoppingBag />)),
            Tab("base_field", html!(<Map />)),
            Tab("base_quests", html!(<HelpCircle />)),
            Tab("base_ums", html!(<Crosshair />)),
            Tab("base_formations", html!(<Triangle />)),
        ]),
        MenuItem::Category(Category("dlc")),
        MenuItem::Tabs(vec![
            Tab("dlc_challenge", html!(<Watch />)),
            Tab("dlc_gauntlet", html!(<LifeBuoy />)),
            Tab("dlc_masha", html!(<Scissors />)),
        ]),
        MenuItem::Category(Category("dlc4")),
        MenuItem::Tabs(vec![Tab("dlc4_enemypedia", html!(<BookOpen />))]),
        MenuItem::Category(Category("danger")),
        MenuItem::Tabs(vec![Tab("danger_flags", html!(<Flag />))]),
    ];

    let save = use_context::<SaveContext>().unwrap();

    html! {
      <aside class="aside is-placed-left is-expanded">
          <div class="aside-tools">
              <div class="aside-tools-label buttons">
                  {edit_operations(&save.get()).collect::<Html>()}
              </div>
          </div>
          <Menu>
            {menu.into_iter().map(Html::from).collect::<Html>()}
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

impl From<Category> for Html {
    fn from(value: Category) -> Self {
        html! {
            <p class="menu-label">
                <Text path={format!("menu_category_{}", value.0)} />
            </p>
        }
    }
}

impl From<Tab> for Html {
    fn from(value: Tab) -> Self {
        html! {
            <li>
                <a>
                <span class="icon-text">
                    <Icon size={Size::Small}>{value.1}</Icon>
                    <span><Text path={format!("menu_{}", value.0)} /></span>
                </span>
                </a>
            </li>
        }
    }
}

impl From<MenuItem> for Html {
    fn from(value: MenuItem) -> Self {
        match value {
            MenuItem::Category(c) => c.into(),
            MenuItem::Tabs(tabs) => html! {
                <MenuList>
                    {tabs.into_iter().map(Html::from).collect::<Html>()}
                </MenuList>
            },
        }
    }
}
