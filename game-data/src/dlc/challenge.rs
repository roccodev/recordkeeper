use serde::{Deserialize, Serialize};

use crate::lang::FilterTable;

#[derive(Serialize, Deserialize)]
pub struct ChallengeLang {
    pub challenges: FilterTable,
}
