use game_data::{
    field::{Location, LocationType, Map},
    lang::Nameable,
};
use ybc::{Container, Table};
use yew::prelude::*;

use crate::{
    components::{
        character::Selector,
        edit::{CheckboxInput, FlagEditor, ToBool},
    },
    data::Data,
    lang::Text,
};

#[derive(Properties, PartialEq)]
struct LocationProps {
    location: Location,
}

#[function_component]
pub fn LocationsPage() -> Html {
    let data = use_context::<Data>().unwrap();
    let map_state = use_state(|| 18); // Aetia // TODO

    let map = data
        .game()
        .field
        .get_map_by_id(*map_state)
        .expect("map not found");

    html! {
        <Container>
            <Selector<Map> state={map_state} values={data.game().field.maps()} />

            <Table classes={classes!("is-fullwidth")}>
                <thead>
                    <tr>
                        <th><Text path="field_location_visited" /></th>
                        <th><Text path="field_location_id" /></th>
                        <th><Text path="field_location_type" /></th>
                        <th><Text path="field_location_name" /></th>
                    </tr>
                </thead>
                <tbody>
                    {for map.locations.iter().map(|location| {
                        html!(<LocationRow location={*location} />)
                    })}
                </tbody>
            </Table>
        </Container>
    }
}

#[function_component]
fn LocationRow(props: &LocationProps) -> Html {
    let data = use_context::<Data>().unwrap();

    let location = &props.location;
    let base_flag = FlagEditor::from(data.game().manual.flags.location);
    let visited_editor = ToBool(FlagEditor {
        flag_type: base_flag.flag_type,
        flag_index: base_flag.flag_index.checked_add(location.id).unwrap(),
    });

    html! {
        <tr>
            <td>
                <CheckboxInput<ToBool<FlagEditor>> editor={visited_editor} />
            </td>
            <th>{location.id}</th>
            <td><Text path={location_type_lang(location.location_type)}/></td>
            <td>{location.get_name_str(data.lang())}</td>
        </tr>
    }
}

fn location_type_lang(ty: LocationType) -> String {
    let id = match ty {
        LocationType::Region => "region",
        LocationType::Location => "location",
        LocationType::Landmark => "landmark",
        LocationType::RestSpot => "camp",
        LocationType::SecretArea => "secret",
        LocationType::Colony => "colony",
        LocationType::RespawnPoint => "respawn",
    };
    format!("field_location_type_{id}")
}
