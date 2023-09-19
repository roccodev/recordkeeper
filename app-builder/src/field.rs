use std::{collections::HashMap, num::NonZeroU16};

use bdat::{hash::murmur3_str, Label, RowRef};
use game_data::field::{FieldLang, FieldRegistry, Location, LocationType, Map, MapId, MapPoint};

use crate::{const_hash, lang::filter_table_from_bdat, BdatRegistry, LangBdatRegistry};

type GimmickTable = HashMap<u32, MapPoint>;
type JumpTable = Vec<Option<MapPoint>>;

pub fn read_data(bdat: &BdatRegistry) -> FieldRegistry {
    let maps = bdat.table(&const_hash!("SYS_MapList"));
    let resources = bdat.table(&const_hash!("RSC_MapFile"));

    let gimmicks = read_gimmicks(bdat);
    let jumps = read_jumps(bdat, &gimmicks);

    let maps = maps.rows().map(|r| maps.row(r.id())).filter_map(|row| {
        let resource = resources.get_row(
            row[const_hash!("ResourceId")]
                .as_single()
                .unwrap()
                .to_integer() as usize,
        )?;
        read_map(bdat, row, resource, &gimmicks, &jumps)
    });

    FieldRegistry::new(maps)
}

pub fn read_lang(bdat: &LangBdatRegistry) -> FieldLang {
    let locations = bdat.table(&const_hash!("msg_location_name"));

    FieldLang {
        locations: filter_table_from_bdat(locations),
    }
}

fn read_map(
    bdat: &BdatRegistry,
    map: RowRef,
    resource: RowRef,
    gimmicks: &GimmickTable,
    jumps: &JumpTable,
) -> Option<Map> {
    let name_id = map[const_hash!("Name")].as_single()?.to_integer() as usize;
    let id = MapId {
        id: map.id(),
        name_id,
    };

    let bdat_prefix = resource[const_hash!("DefaultBdatPrefix")]
        .as_single()?
        .as_str();
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

fn read_location(row: RowRef, gimmicks: &GimmickTable, jumps: &JumpTable) -> Location {
    let hash_id = row[const_hash!("ID")].as_single().unwrap().to_integer();
    let name_id = row[const_hash!("LocationName")]
        .as_single()
        .unwrap()
        .to_integer() as usize;
    let category = row[const_hash!("CategoryPriority")]
        .as_single()
        .unwrap()
        .to_integer() as usize;
    let map_jump: u16 = row[const_hash!("MapJumpID")]
        .as_single()
        .unwrap()
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
    let table = bdat.table(&const_hash!("SYS_GimmickLocation_dlc04"));
    table
        .rows()
        .map(|r| table.row(r.id()))
        .filter_map(|row| {
            let gimmick_id = row[const_hash!("GimmickID")]
                .as_single()
                .unwrap()
                .to_integer();

            if gimmick_id == 0 {
                return None;
            }

            let x = row[const_hash!("X")].as_single().unwrap().to_float();
            let y = row[const_hash!("Y")].as_single().unwrap().to_float();
            let z = row[const_hash!("Z")].as_single().unwrap().to_float();

            Some((gimmick_id, MapPoint { x, y, z }))
        })
        .collect()
}

fn read_jumps(bdat: &BdatRegistry, gimmicks: &GimmickTable) -> JumpTable {
    let table = bdat.table(&const_hash!("SYS_MapJumpList"));

    table
        .rows()
        .map(|r| table.row(r.id()))
        .map(|row| {
            let formation_id = row[const_hash!("FormationID")]
                .as_single()
                .unwrap()
                .to_integer();

            (formation_id != 0)
                .then(|| ())
                .and_then(|_| gimmicks.get(&formation_id))
                .copied()
        })
        .collect()
}
