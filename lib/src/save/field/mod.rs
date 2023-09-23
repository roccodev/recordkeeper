use recordkeeper_macros::SaveBin;

#[cfg(feature = "map-bitmaps")]
pub mod map;

#[derive(SaveBin, Debug)]
pub struct FieldConfig {
    /// ID for `QST_List`
    pub active_quest_id: u32,

    /// 0: Main, 2: Hero, 3: Side
    pub navi_mode: u8,
    pub navi_page: u8,
    /// Whether navigation is currently active
    pub show_route: bool,

    #[loc(0x7)]
    /// Bit 0: food recipe pinned
    /// Bit 1: gem recipe pinned
    pinned_flags: u8,
    /// Pinned food recipe (for the Pinned Items list)
    pub pinned_recipe: u16,
    /// Pinned gem recipe (for the Pinned Items list)
    pub pinned_gem: u16,
}
