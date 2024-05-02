use recordkeeper_macros::SaveBin;

#[cfg(feature = "map-bitmaps")]
pub mod map;

pub mod colony;

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

#[derive(SaveBin, Debug)]
pub struct ActiveMeal {
    meal_type: u16,
    pub meal_id: u16,
    /// Seconds left on the meal timer
    #[loc(0x6)]
    pub time_left: f32,
    /// Initial meal timer (seconds)
    pub time_max: f32,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MealType {
    Cooked = 1,
    Canteen = 2,
}

#[derive(SaveBin, Debug)]
pub struct CameraSettings {
    /// Camera settings on land
    pub profile_land: CameraProfile,
    /// Camera settings when aboard the ship
    pub profile_ship: CameraProfile,
}

#[derive(SaveBin, Debug)]
#[size(20)]
pub struct CameraProfile {
    pub fov: f32,
    pub elevation: f32,
    pub yaw: f32,
    pub pitch: f32,
    /// If this is `true`, the camera does not move with the character.
    ///
    /// In practice, this is `false` when the camera is reset (ZL+RStick), and it is set to
    /// `true` whenever the user moves the camera.
    pub detached: bool,
}

impl ActiveMeal {
    pub fn is_active(&self) -> bool {
        self.meal_type != 0
    }

    /// Returns the active meal type.
    ///
    /// ## Panics
    /// Panics if the meal is not active, or the type is invalid.
    pub fn meal_type(&self) -> MealType {
        match self.meal_type {
            0 => panic!("no meal active"),
            1 => MealType::Cooked,
            2 => MealType::Canteen,
            t => panic!("unknown meal type {t}"),
        }
    }

    pub fn set_meal_type(&mut self, meal_type: MealType) {
        self.meal_type = meal_type as u16;
    }
}
