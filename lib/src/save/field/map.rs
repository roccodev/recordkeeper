use std::ops::RangeInclusive;

use recordkeeper_macros::SaveBin;

// Unfortunately these are hardcoded in the executable
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

// Overlaps with some sections from base game
static BITMAP_LEN_DLC4: &[&[usize]] = &[
    // ma40a
    &[(0), 0x11000, 0x11000],
];

// Maximum of base game maps total length and DLC4 total length
const BITMAP_TOTAL_LEN: usize = 0x102000;

#[derive(SaveBin, Debug)]
pub struct MapBitmaps {
    maps: Box<[u8; BITMAP_TOTAL_LEN]>,
}

pub struct Bitmap<'a>(&'a [u8]);
pub struct BitmapMut<'a>(&'a mut [u8]);

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
    pub fn get(&self, map: MapWorld, layer: usize) -> Bitmap {
        let (offset, len) = Self::offset_and_len(map, layer);
        Bitmap(&self.maps[offset..offset.checked_add(len).unwrap()])
    }

    pub fn get_mut(&mut self, map: MapWorld, layer: usize) -> BitmapMut {
        let (offset, len) = Self::offset_and_len(map, layer);
        BitmapMut(&mut self.maps[offset..offset.checked_add(len).unwrap()])
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

impl<'a> Bitmap<'a> {
    pub fn get(&self, index: usize) -> bool {
        let mask = 1 << (index % u8::BITS as usize);
        self.0[index / u8::BITS as usize] & mask != 0
    }
}

impl<'a> BitmapMut<'a> {
    const BITS: usize = u8::BITS as usize;

    pub fn set(&mut self, index: usize, value: bool) {
        let byte = &mut self.0[index / Self::BITS];
        let mask = 1 << (index % Self::BITS);

        *byte &= !mask;
        if value {
            *byte |= mask;
        }
    }

    pub fn set_multiple(&mut self, range: RangeInclusive<usize>, value: bool) {
        let start = *range.start();
        let end = *range.end();

        if start == end {
            self.set(start, value);
            return;
        }

        assert!(start < end, "only ascending ranges are supported");

        let start_byte = start / Self::BITS;
        let end_byte = end / Self::BITS;

        if start_byte == end_byte {
            let mask = if end - start == Self::BITS - 1 {
                u8::MAX
            } else {
                (1 << (end - start)) - 1 << (Self::BITS - end % Self::BITS)
            };
            let byte = &mut self.0[start_byte];
            *byte &= !mask;
            if value {
                *byte |= mask;
            }
            return;
        }

        if end_byte - start_byte > 1 {
            // memset the bytes in the middle
            self.0[start_byte + 1..end_byte].fill(if value { u8::MAX } else { 0 });
        }

        // Set/unset the specific bits in the start and end bytes

        let start_bits = Self::BITS - start % Self::BITS;
        let start_bits = if start_bits == Self::BITS {
            u8::MAX
        } else {
            (1 << start_bits) - 1
        };

        self.0[start_byte] &= !start_bits;
        if value {
            self.0[start_byte] |= start_bits;
        }

        let end_bits = end % Self::BITS + 1;
        let end_bits = if end_bits == Self::BITS {
            u8::MAX
        } else {
            (1 << end_bits) - 1 << (Self::BITS - end_bits)
        };

        self.0[end_byte] &= !end_bits;
        if value {
            self.0[end_byte] |= end_bits;
        }
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

    #[test]
    fn bitmap_set_multi() {
        let mut b1 = [0u8; 4];
        BitmapMut(&mut b1).set_multiple(5..=31, true);
        assert_eq!([0x07, 0xff, 0xff, 0xff], b1);
        BitmapMut(&mut b1).set_multiple(5..=31, false);
        assert_eq!([0u8; 4], b1);

        b1.fill(0);
        BitmapMut(&mut b1).set_multiple(0..=30, true);
        assert_eq!([0xff, 0xff, 0xff, 0xfe], b1);
        BitmapMut(&mut b1).set_multiple(0..=30, false);
        assert_eq!([0u8; 4], b1);

        b1.fill(0);
        BitmapMut(&mut b1).set_multiple(0..=1, true);
        assert_eq!([0x80, 0x0, 0x0, 0x0], b1);
        BitmapMut(&mut b1).set_multiple(0..=1, false);
        assert_eq!([0u8; 4], b1);

        b1.fill(0);
        BitmapMut(&mut b1).set_multiple(8..=16, true);
        assert_eq!([0x0, 0xff, 0x80, 0x0], b1);
        BitmapMut(&mut b1).set_multiple(8..=16, false);
        assert_eq!([0u8; 4], b1);

        b1.fill(0);
        BitmapMut(&mut b1).set_multiple(0..=7, true);
        assert_eq!([0xff, 0x0, 0x0, 0x0], b1);
        BitmapMut(&mut b1).set_multiple(0..=7, false);
        assert_eq!([0u8; 4], b1);
    }
}
