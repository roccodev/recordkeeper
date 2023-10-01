use game_data::manual::{Flags, ManualData};

static FLAGS_JSON: &str = include_str!("../res/flags.json");
static FORMATION_COLORS_JSON: &str = include_str!("../res/formation_colors.json");

pub fn read_manual_data() -> ManualData {
    let flags: Flags = serde_json::from_str(FLAGS_JSON).expect("flags.json read");

    ManualData { flags }
}

pub fn read_formation_colors() -> impl IntoIterator<Item = u32> {
    let flags: Vec<String> =
        serde_json::from_str(FORMATION_COLORS_JSON).expect("formation_colors.json read");
    flags
        .into_iter()
        .map(|s| u32::from_str_radix(&s, 16).unwrap())
}
