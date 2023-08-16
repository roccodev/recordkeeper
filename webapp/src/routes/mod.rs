use yew::prelude::*;
use yew_router::Routable;

mod flags;
mod item;
mod meta;

#[derive(Routable, Clone, PartialEq, Copy)]
pub enum Route {
    #[at("/")]
    Home,
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
    #[at("/chbtl")]
    ChallengeBattle,
    #[at("/gauntlet")]
    Gauntlet,
    #[at("/masha")]
    Masha,
    #[at("/dlc4enemy")]
    Dlc4Enemypedia,
    #[at("/flags")]
    Flags,
}

pub fn render(route: Route) -> Html {
    match route {
        Route::Home => html!(),
        Route::Meta => html!(<meta::SaveMeta />),
        Route::Characters => html!(),
        Route::Ouroboros => html!(),
        Route::Items => html!(<item::ItemInventory />),
        Route::Field => html!(),
        Route::Quests => html!(),
        Route::Uniques => html!(),
        Route::Formations => html!(),
        Route::ChallengeBattle => html!(),
        Route::Gauntlet => html!(),
        Route::Masha => html!(),
        Route::Dlc4Enemypedia => html!(),
        Route::Flags => html!(<flags::FlagList />),
    }
}
