use yew::prelude::*;
use yew_router::Routable;

use self::field::FieldTab;

mod about;
mod character;
mod field;
mod flags;
mod item;
mod meta;
mod ouroboros;
mod quest;

#[derive(Routable, Clone, PartialEq, Copy)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/meta")]
    Meta,
    #[at("/characters")]
    Characters,
    #[at("/ouroboros")]
    Ouroboros,
    #[at("/items")]
    Items,
    #[at("/field")]
    Field,
    #[at("/quests")]
    Quests,
    #[at("/uniques")]
    Uniques,
    #[at("/formations")]
    Formations,
    #[at("/powaugment")]
    PowAugment,
    #[at("/chbtl")]
    ChallengeBattle,
    #[at("/gauntlet")]
    Gauntlet,
    #[at("/masha")]
    Masha,
    #[at("/dlc4colle")]
    Dlc4Collepedia,
    #[at("/dlc4enemy")]
    Dlc4Enemypedia,
    #[at("/flags")]
    Flags,
}

pub fn render(route: Route) -> Html {
    match route {
        Route::Home => html!(),
        Route::About => html!(<about::About />),
        Route::Meta => html!(<meta::SaveMeta />),
        Route::Characters => html!(<character::Characters />),
        Route::Ouroboros => html!(<ouroboros::OuroborosPage />),
        Route::Items => html!(<item::ItemInventory />),
        Route::Field => html!(<field::FieldPage />),
        Route::Quests => html!(<quest::Quests />),
        Route::Uniques => html!(),
        Route::Formations => html!(),
        Route::PowAugment => html!(),
        Route::ChallengeBattle => html!(),
        Route::Gauntlet => html!(),
        Route::Masha => html!(),
        Route::Dlc4Collepedia => html!(),
        Route::Dlc4Enemypedia => html!(),
        Route::Flags => html!(<flags::FlagList />),
    }
}
