use game_data::manual::{Flags, ManualData};

static FLAGS_JSON: &str = include_str!("../res/flags.json");

pub fn read_manual_data() -> ManualData {
    let flags: Flags = serde_json::from_str(FLAGS_JSON).expect("flags.json read");

    ManualData { flags }
}
