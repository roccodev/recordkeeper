use bdat::{hash::murmur3_str, Label, RowRef};
use game_data::field::{FieldLang, FieldRegistry, Location, LocationType, Map, MapId};

use crate::{const_hash, lang::filter_table_from_bdat, BdatRegistry, LangBdatRegistry};

pub fn read_data(bdat: &BdatRegistry) -> FieldRegistry {
    let maps = bdat.table(&const_hash!("SYS_MapList"));
    let resources = bdat.table(&const_hash!("RSC_MapFile"));

    let maps = maps.rows().map(|r| maps.row(r.id())).filter_map(|row| {
        let resource = resources.get_row(
            row[const_hash!("ResourceId")]
                .as_single()
                .unwrap()
                .to_integer() as usize,
        )?;
        read_map(bdat, row, resource)
    });

    FieldRegistry::new(maps)
}

pub fn read_lang(bdat: &LangBdatRegistry) -> FieldLang {
    let locations = bdat.table(&const_hash!("msg_location_name"));

    FieldLang {
        locations: filter_table_from_bdat(locations),
    }
}

fn read_map(bdat: &BdatRegistry, map: RowRef, resource: RowRef) -> Option<Map> {
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
    ))))?;

    let locations = location_map
        .rows()
        .map(|row| read_location(location_map.row(row.id())))
        .collect();

    Some(Map { id, locations })
}

fn read_location(row: RowRef) -> Location {
    let name_id = row[const_hash!("LocationName")]
        .as_single()
        .unwrap()
        .to_integer() as usize;
    let category = row[const_hash!("CategoryPriority")]
        .as_single()
        .unwrap()
        .to_integer() as usize;

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

    Location {
        id: row.id(),
        name_id,
        location_type,
    }
}
