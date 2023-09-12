use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ManualData {
    pub flags: Flags,
}

#[derive(Serialize, Deserialize)]
pub struct Flags {
    pub new_game_plus: Flag,
    pub difficulty: Flag,
    pub scenario: Flag,
    /// Enables Interlink for Noah
    pub ouro_enable_noah: Flag,
    /// Base flag to enable Interlink for each character.
    ///
    /// Note: the base flag refers to Mio
    pub ouro_enable: Flag,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Flag {
    pub bits: usize,
    pub index: usize,
}
