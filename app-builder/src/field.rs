use std::{collections::HashMap, num::NonZeroU16};

use bdat::{hash::murmur3_str, label_hash, Label, TableAccessor};
use game_data::field::{FieldLang, FieldRegistry, Location, LocationType, Map, MapId, MapPoint};

use crate::{lang::filter_table_from_bdat, BdatRegistry, LangBdatRegistry, ModernRow};

type GimmickTable = HashMap<u32, MapPoint>;
type JumpTable = Vec<Option<MapPoint>>;

pub fn read_data(bdat: &BdatRegistry) -> FieldRegistry {
    let maps = bdat.table(label_hash!("SYS_MapList"));
    let resources = bdat.table(label_hash!("RSC_MapFile"));

    let gimmicks = read_gimmicks(bdat);
    let jumps = read_jumps(bdat, &gimmicks);

    let maps = maps.rows().filter_map(|row| {
        let resource =
            resources.get_row(row.get(label_hash!("ResourceId")).to_integer() as usize)?;
        read_map(bdat, row, resource, &gimmicks, &jumps)
    });

    FieldRegistry::new(maps)
}

pub fn read_lang(bdat: &LangBdatRegistry) -> FieldLang {
    let locations = bdat.table(label_hash!("msg_location_name"));

    FieldLang {
        locations: filter_table_from_bdat(locations),
    }
}

fn read_map(
    bdat: &BdatRegistry,
    map: ModernRow,
    resource: ModernRow,
    gimmicks: &GimmickTable,
    jumps: &JumpTable,
) -> Option<Map> {
    let name_id = map.get(label_hash!("Name")).to_integer() as usize;
    let id = MapId {
        id: map.id(),
        name_id,
    };

    let bdat_prefix = resource.get(label_hash!("DefaultBdatPrefix")).as_str();
    let location_map = bdat.get_table(&Label::Hash(murmur3_str(&format!(
        "{bdat_prefix}_GMK_Location"
    ))));

    let locations = match location_map {
        Some(table) => table
            .rows()
            .map(|row| read_location(table.row(row.id()), gimmicks, jumps))
            .collect(),
        None => std::iter::empty().collect(),
    };

    Some(Map { id, locations })
}

fn read_location(row: ModernRow, gimmicks: &GimmickTable, jumps: &JumpTable) -> Location {
    let hash_id = row.get(label_hash!("ID")).to_integer();
    let name_id = row.get(label_hash!("LocationName")).to_integer() as usize;
    let category = row.get(label_hash!("CategoryPriority")).to_integer() as usize;
    let map_jump: u16 = row
        .get(label_hash!("MapJumpID"))
        .to_integer()
        .try_into()
        .unwrap();

    let location_type = match category {
        0 => LocationType::RestSpot,
        1 => LocationType::SecretArea,
        2 => LocationType::Landmark,
        3 => LocationType::Colony,
        4 => LocationType::Location,
        5 => LocationType::Region,
        6 => LocationType::RespawnPoint,
        n => panic!("unknown location type {n}"),
    };

    let point = if map_jump != 0 {
        jumps[map_jump.checked_sub(1).unwrap() as usize]
    } else {
        gimmicks.get(&hash_id).copied()
    };

    Location {
        id: row.id(),
        name_id,
        location_type,
        map_jump: NonZeroU16::new(map_jump),
        map_point: point,
    }
}

fn read_gimmicks(bdat: &BdatRegistry) -> GimmickTable {
    let table = bdat.table(label_hash!("SYS_GimmickLocation_dlc04"));
    table
        .rows()
        .map(|r| table.row(r.id()))
        .filter_map(|row| {
            let gimmick_id = row.get(label_hash!("GimmickID")).to_integer();

            if gimmick_id == 0 {
                return None;
            }

            let x = row.get(label_hash!("X")).get_as();
            let y = row.get(label_hash!("Y")).get_as();
            let z = row.get(label_hash!("Z")).get_as();

            Some((gimmick_id, MapPoint { x, y, z }))
        })
        .collect()
}

fn read_jumps(bdat: &BdatRegistry, gimmicks: &GimmickTable) -> JumpTable {
    let table = bdat.table(label_hash!("SYS_MapJumpList"));

    table
        .rows()
        .map(|r| table.row(r.id()))
        .map(|row| {
            let formation_id = row.get(label_hash!("FormationID")).to_integer();

            (formation_id != 0)
                .then(|| ())
                .and_then(|_| gimmicks.get(&formation_id))
                .copied()
        })
        .collect()
}
