use std::ops::RangeInclusive;

use bdat::label_hash;
use bdat::modern::ModernRowRef;
use bdat::modern::ModernTable;
use game_data::scenario::ScenarioRange;
use game_data::scenario::ScenarioRanges;

use crate::BdatRegistry;

pub fn read_scenario_events(bdat: &BdatRegistry) -> ScenarioRanges {
    let menu_scenario = bdat.table(label_hash!("MNU_saveload_scenario"));
    let cond_list = bdat.table(label_hash!("FLD_ConditionList"));
    let cond_scenario = bdat.table(label_hash!("FLD_ConditionScenario"));

    let (ranges_base, ranges_dlc4): (Vec<_>, Vec<_>) = menu_scenario
        .rows()
        .flat_map(|row| {
            let base_cond = row.get(label_hash!("ScenarioCond")).to_integer();
            let dlc_cond = row.get(label_hash!("ScenarioCondDLC4")).to_integer();
            let chapter = row.id();

            [
                cond_list.get_row(base_cond).map(|r| (false, chapter, r)),
                cond_list.get_row(dlc_cond).map(|r| (true, chapter, r)),
            ]
            .into_iter()
            .flatten()
        })
        .map(|(dlc, chapter, row)| (dlc, chapter, cond_to_range(cond_scenario, &row)))
        .partition(|(dlc, _, _)| *dlc);
    ScenarioRanges::new(
        ranges_base
            .into_iter()
            .map(|(_, chapter, range)| ScenarioRange {
                chapter: chapter as u32,
                range,
            }),
        ranges_dlc4
            .into_iter()
            .map(|(_, chapter, range)| ScenarioRange {
                chapter: chapter as u32,
                range,
            }),
    )
}

fn cond_to_range(cond_scenario: &ModernTable, cond_row: &ModernRowRef) -> RangeInclusive<u16> {
    let scenario_cond_id = cond_row.get(label_hash!("Condition")).to_integer();
    let row = cond_scenario.row(scenario_cond_id);
    let min = row.get(label_hash!("ScenarioMin")).get_as();
    let max = row.get(label_hash!("ScenarioMax")).get_as();
    min..=max
}
