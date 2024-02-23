use game_data::dlc::map::{Dlc4Region, MapAchievementProgress};
use game_data::lang::Nameable;
use game_data::GameData;
use recordkeeper::SaveData;
use strum::{EnumIter, FromRepr, IntoEnumIterator};
use ybc::{Button, Control, Field};
use yew::prelude::*;

use crate::components::dlc::map::AchievementStatusEditor;
use crate::components::edit::FlagEditor;
use crate::save::SaveContext;
use crate::{
    components::{
        dlc::map::RegionAchievements,
        edit::Editor,
        select::{HtmlSelect, Selector},
    },
    data::Data,
    lang::Text,
};

#[derive(Properties, PartialEq)]
pub struct RegionProps {
    region: usize,
}

#[derive(Properties, PartialEq)]
struct BulkSelectProps {
    state: UseStateHandle<BulkEditState>,
}

#[derive(Copy, Clone, Eq, PartialEq, EnumIter, FromRepr)]
enum BulkEditState {
    Category,
    Region,
    All,
}

#[function_component]
pub fn MapPage() -> Html {
    let data = use_context::<Data>().unwrap();
    let save_context = use_context::<SaveContext>().unwrap();

    let region_state = use_state(|| 1);
    let category = use_state(|| 1);
    let bulk_edit = use_state(|| BulkEditState::Category);

    let region = *region_state - 1;

    let categories: Vec<_> = data
        .game()
        .dlc
        .map
        .achievements(region)
        .iter()
        .map(|a| (a.ty, a.get_name_str(data.lang()).unwrap()))
        .collect();

    let update = {
        let category = category.clone();
        Callback::from(move |s: String| {
            category.set(s.parse::<u32>().unwrap());
        })
    };

    let selected_index = categories
        .iter()
        .position(|(i, _)| *i == *category)
        .unwrap_or_default();

    let bulk_set_callback = |value: MapAchievementProgress| {
        let bulk_edit = bulk_edit.clone();
        let game = data.game();
        let save_context = save_context.clone();
        Callback::from(move |_: MouseEvent| {
            let bulk = *bulk_edit;
            save_context
                .edit(move |save| bulk_set(save, game, region, selected_index, bulk, value));
        })
    };

    html! {
        <>
            <Field classes={classes!("is-grouped", "is-align-items-end")}>
                <Control>
                    <Field>
                        <label class="label"><Text path="dlc4_map_region" /></label>
                        <Control>
                            <Selector<Dlc4Region> state={region_state.clone()} values={data.game().dlc.map.regions()} />
                        </Control>
                    </Field>
                </Control>
                <Control classes="is-flex-grow-1">
                    <Field>
                        <label class="label"><Text path="dlc4_map_category" /></label>
                        <Control>
                            <HtmlSelect on_change={update} value={category.to_string()} selected_idx={selected_index}>
                                {for categories.iter().map(|(i, s)| {
                                    html! {
                                        <option value={i.to_string()} selected={*i == *category}>
                                            {s}
                                        </option>
                                    }
                                })}
                            </HtmlSelect>
                        </Control>
                    </Field>
                </Control>
                <Control>
                    <BulkEditSelect state={bulk_edit.clone()} />
                </Control>
                <Control>
                    <Button onclick={bulk_set_callback(MapAchievementProgress::Completed)}>
                        <Text path="dlc4_map_bulk_on" />
                    </Button>
                </Control>
                <Control>
                    <Button onclick={bulk_set_callback(MapAchievementProgress::Visible)}>
                        <Text path="dlc4_map_bulk_progress" />
                    </Button>
                </Control>
                <Control>
                    <Button onclick={bulk_set_callback(MapAchievementProgress::Hidden)}>
                        <Text path="dlc4_map_bulk_off" />
                    </Button>
                </Control>
            </Field>
            <RegionAchievements region={region} category={*category} />
        </>
    }
}

#[function_component]
fn BulkEditSelect(props: &BulkSelectProps) -> Html {
    let state = props.state.clone();
    let cur_id = *state as usize;

    let update = {
        let state = state.clone();
        Callback::from(move |s: String| {
            state.set(BulkEditState::from_repr(s.parse::<usize>().unwrap()).unwrap());
        })
    };

    html! {
        <Field>
            <label class="label"><Text path="dlc4_map_bulk" /></label>
            <HtmlSelect on_change={update} value={cur_id.to_string()} selected_idx={cur_id}>
                {for BulkEditState::iter().map(|s| {
                    html! {
                        <option value={(s as usize).to_string()} selected={s == *state}>
                            <Text path={format!("dlc4_map_bulk_state_{}", match s {
                                BulkEditState::Category => "category",
                                BulkEditState::Region => "region",
                                BulkEditState::All => "all",
                            })} />
                        </option>
                    }
                })}
            </HtmlSelect>
        </Field>
    }
}

fn bulk_set(
    save: &mut SaveData,
    game: &GameData,
    region: usize,
    category: usize,
    bulk: BulkEditState,
    value: MapAchievementProgress,
) {
    let region = game.dlc.map.achievements(region);
    let category = &region[category];

    let flags: Vec<_> = match bulk {
        BulkEditState::Category => category.searches.iter().map(|a| a.flag).collect(),
        BulkEditState::Region => region
            .iter()
            .flat_map(|a| a.searches.iter())
            .map(|a| a.flag)
            .collect(),
        BulkEditState::All => game
            .dlc
            .map
            .all_achievements()
            .flat_map(|a| a.searches.iter())
            .map(|a| a.flag)
            .collect(),
    };
    for flag in flags {
        AchievementStatusEditor(FlagEditor::from(flag)).set(save, value.clone());
    }
}
