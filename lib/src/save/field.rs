use recordkeeper_macros::SaveBin;

#[rustfmt::skip]
static BITMAP_LEN_BASE: &[&[usize]] = &[
    // First value: offset. Other values: size of each layer's bitmap

    // ma01a
    &[(0), 0x12000, 0x6000, 0x1000],
    // ma04a
    &[(0x19000), 0x15000, 0x1000, 0x1000, 0x1000],
    // ma07a
    &[(0x31000), 0xd000, 0x1000, 0x3000, 0x1000],
    // ma09a
    &[(0x43000), 0x15000, 0x1000, 0x1000, 0x2000, 0x2000, 0x1000, 0x1000],
    // ma11a
    &[(0x60000), 0x98000, 0x1000],
    // ma14a
    &[(0xf9000), 0x1000],
    // ma15a / ma21a
    &[(0xfa000), 0x1000],
    // ma17a
    &[(0xfb000), 0x5000],
    // ma22a
    &[(0x100000), 0x2000],
];

static BITMAP_LEN_DLC4: &[&[usize]] = &[
    // ma40a
    &[(0), 0x11000, 0x11000],
];

// Maximum of base game maps total length and DLC4 total length
const BITMAP_TOTAL_LEN: usize = 0x102000;

#[derive(SaveBin, Debug)]
pub struct MapBitmaps {
    maps: [u8; BITMAP_TOTAL_LEN],
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MapWorld {
    Ma01A = 0,
    Ma04A = 1,
    Ma07A = 2,
    Ma09A = 3,
    Ma11A = 4,
    Ma14A = 5,
    /// Also `ma21a`
    Ma15A = 6,
    Ma17A = 7,
    Ma22A = 8,
    Ma40A = 9,
}

impl MapBitmaps {
    pub fn get(&self, map: MapWorld, layer: usize) -> &[u8] {
        let (offset, len) = Self::offset_and_len(map, layer);
        &self.maps[offset..offset.checked_add(len).unwrap()]
    }

    pub fn get_mut(&mut self, map: MapWorld, layer: usize) -> &mut [u8] {
        let (offset, len) = Self::offset_and_len(map, layer);
        &mut self.maps[offset..offset.checked_add(len).unwrap()]
    }

    fn offset_and_len(map: MapWorld, layer: usize) -> (usize, usize) {
        let offsets = if map == MapWorld::Ma40A {
            BITMAP_LEN_DLC4[0]
        } else {
            BITMAP_LEN_BASE[map as usize]
        };
        let layer = layer.checked_add(1).unwrap();
        let offset = offsets[0] + offsets[1..].iter().take(layer).sum::<usize>();
        let len = offsets[layer];
        (offset, len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_size() {
        let len_base: usize = BITMAP_LEN_BASE.iter().flat_map(|s| s.iter().skip(1)).sum();
        let len_dlc: usize = BITMAP_LEN_BASE.iter().flat_map(|s| s.iter().skip(1)).sum();
        assert_eq!(BITMAP_TOTAL_LEN, len_base.max(len_dlc))
    }

    #[test]
    fn test_offsets() {
        for lens in [BITMAP_LEN_BASE, BITMAP_LEN_DLC4] {
            let mut offset = 0;
            for map in lens {
                assert_eq!(offset, map[0]);
                offset += map[1..].iter().sum::<usize>();
            }
        }
    }
}
