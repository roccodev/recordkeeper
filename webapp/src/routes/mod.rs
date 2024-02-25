use yew::prelude::*;
use yew_router::Routable;

use crate::components::FluidContainer;

mod about;
mod character;
mod chrono;
mod dlc;
mod enemy;
mod field;
mod flags;
pub mod formation;
mod home;
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
    #[at("/dlc4map")]
    Dlc4Map,
    #[at("/dlc4colle")]
    Dlc4Collepedia,
    #[at("/dlc4enemy")]
    Dlc4Enemypedia,
    #[at("/flags")]
    Flags,
    #[at("/chrono")]
    ChronoData,
}

pub fn render(route: Route) -> Html {
    let contents = match route {
        Route::Home => html!(<home::Home />),
        Route::About => html!(<about::About />),
        Route::Meta => html!(<meta::SaveMeta />),
        Route::Characters => html!(<character::Characters />),
        Route::Ouroboros => html!(<ouroboros::OuroborosPage />),
        Route::Items => html!(<item::ItemInventory />),
        Route::Field => html!(<field::FieldPage />),
        Route::Quests => html!(<quest::Quests />),
        Route::Uniques => html!(<enemy::EnemyPage />),
        Route::Formations => html!(<formation::Formations />),
        Route::PowAugment => html!(<dlc::pow_augment::PowAugmentPage />),
        Route::ChallengeBattle => html!(<dlc::challenge::ChallengePage />),
        Route::Gauntlet => html!(<dlc::gauntlet::GauntletPage />),
        Route::Dlc4Map => html!(<dlc::map::MapPage />),
        Route::Dlc4Collepedia => html!(<dlc::pedia::CollepediaPage />),
        Route::Dlc4Enemypedia => html!(<dlc::pedia::EnemypediaPage />),
        Route::Flags => html!(<flags::FlagList />),
        Route::ChronoData => html!(<chrono::ChronoPage />),
    };
    html! {
        <FluidContainer>
            {contents}
        </FluidContainer>
    }
}
