use serde::{Deserialize, Serialize};

use self::{
    challenge::{ChallengeGame, ChallengeLang},
    community::{DlcCommunity, DlcCommunityLang},
    map::{Dlc4Map, Dlc4MapLang},
    masha::{GameCraftItems, LangCraftItems},
    pedia::{Dlc4Collepedia, Enemypedia},
};

pub mod challenge;
pub mod community;
pub mod map;
pub mod masha;
pub mod pedia;
pub mod pow_augment;

#[derive(Serialize, Deserialize)]
pub struct DlcData {
    pub masha: GameCraftItems,
    pub challenge: ChallengeGame,
    pub map: Dlc4Map,
    pub collepedia: Regional<Dlc4Collepedia>,
    pub enemypedia: Regional<Enemypedia>,
    pub community: DlcCommunity,
}

#[derive(Serialize, Deserialize)]
pub struct DlcLang {
    pub masha: LangCraftItems,
    pub challenge: ChallengeLang,
    pub map: Dlc4MapLang,
    pub community: DlcCommunityLang,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Regional<T> {
    region_map: [Box<[T]>; 5],
}

impl<T> Regional<T> {
    pub fn get(&self, region: usize) -> &[T] {
        &self.region_map[region]
    }

    pub fn iter(&self, region: usize) -> impl Iterator<Item = &T> {
        self.region_map[region].iter()
    }
}

impl<N, T> FromIterator<(N, T)> for Regional<T>
where
    N: Into<usize>,
    T: Clone,
{
    fn from_iter<I: IntoIterator<Item = (N, T)>>(iter: I) -> Self {
        let Ok(mut map): Result<[Vec<T>; 5], _> = vec![Vec::new(); 5].try_into() else {
            unreachable!()
        };
        for (region, item) in iter {
            map[region.into()].push(item);
        }
        Regional {
            region_map: map.map(Vec::into_boxed_slice),
        }
    }
}
