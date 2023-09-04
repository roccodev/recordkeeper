use std::{cmp::Ordering, ops::RangeInclusive, rc::Rc};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ScenarioRanges {
    ranges_base: Rc<[ScenarioRange]>,
    ranges_dlc4: Rc<[ScenarioRange]>,
}

#[derive(Serialize, Deserialize)]
pub struct ScenarioRange {
    pub chapter: u32,
    pub range: RangeInclusive<u16>,
}

impl ScenarioRanges {
    pub fn new(
        ranges_base: impl IntoIterator<Item = ScenarioRange>,
        ranges_dlc4: impl IntoIterator<Item = ScenarioRange>,
    ) -> Self {
        let mut ranges_base: Rc<[ScenarioRange]> = ranges_base.into_iter().collect();
        Rc::get_mut(&mut ranges_base)
            .unwrap()
            .sort_unstable_by_key(|e| *e.range.start());
        let mut ranges_dlc4: Rc<[ScenarioRange]> = ranges_dlc4.into_iter().collect();
        Rc::get_mut(&mut ranges_dlc4)
            .unwrap()
            .sort_unstable_by_key(|e| *e.range.start());

        Self {
            ranges_base,
            ranges_dlc4,
        }
    }

    pub fn get_chapter_by_scenario(&self, scenario_flag: u16, dlc4: bool) -> Option<u32> {
        let ranges = if dlc4 {
            &self.ranges_base
        } else {
            &self.ranges_dlc4
        };
        ranges
            .binary_search_by(|r| range_cmp(&r.range, scenario_flag))
            .ok()
            .map(|i| ranges[i].chapter)
    }
}

fn range_cmp(range: &RangeInclusive<u16>, item: u16) -> Ordering {
    if range.contains(&item) {
        return Ordering::Equal;
    }
    if item < *range.start() {
        return Ordering::Greater;
    }
    Ordering::Less
}
