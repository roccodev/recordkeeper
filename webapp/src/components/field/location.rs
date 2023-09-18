use game_data::{
    field::{Location, LocationType, Map},
    lang::Nameable,
    GameData,
};
use ybc::{Button, Container, Control, Field, Table};
use yew::prelude::*;

use crate::{
    components::{
        character::Selector,
        edit::{CheckboxInput, Editor, FlagEditor, ToBool},
    },
    data::Data,
    lang::Text,
    save::SaveContext,
};

#[derive(Properties, PartialEq)]
struct LocationProps {
    location: Location,
}

#[function_component]
pub fn LocationsPage() -> Html {
    let data = use_context::<Data>().unwrap();
    let save_context = use_context::<SaveContext>().unwrap();

    // Cent-Omnia / Aetia
    let map_state = use_state(|| {
        if save_context.get().get().save().is_dlc4() {
            79
        } else {
            18
        }
    });

    let map = data
        .game()
        .field
        .get_map_by_id(*map_state)
        .expect("map not found");

    let set_all = |val: bool| {
        let save_context = save_context.clone();
        let game = data.game();
        Callback::from(move |_: MouseEvent| {
            save_context.edit(move |save| {
                for loc in map.locations.iter() {
                    let flag = location_to_flag(game, loc);
                    flag.set(save, val);
                }
            })
        })
    };

    html! {
        <Container>
            <Field classes={classes!("is-grouped")}>
                <Control>
                    <Selector<Map> state={map_state} values={data.game().field.maps()} />
                </Control>
                <Control>
                    <Button onclick={set_all(true)}>
                        <Text path="field_location_all_on" />
                    </Button>
                </Control>
                <Control>
                    <Button onclick={set_all(false)}>
                        <Text path="field_location_all_off" />
                    </Button>
                </Control>
            </Field>

            <Table classes={classes!("is-fullwidth")}>
                <thead>
                    <tr>
                        <th><Text path="field_location_id" /></th>
                        <th><Text path="field_location_visited" /></th>
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

    html! {
        <tr>
            <th>{location.id}</th>
            <td>
                <CheckboxInput<ToBool<FlagEditor>> editor={location_to_flag(data.game(), location)} />
            </td>
            <td><Text path={location_type_lang(location.location_type)}/></td>
            <td>{location.get_name_str(data.lang())}</td>
        </tr>
    }
}

fn location_to_flag(data: &GameData, location: &Location) -> ToBool<FlagEditor> {
    let base_flag = FlagEditor::from(data.manual.flags.location);
    ToBool(FlagEditor {
        flag_type: base_flag.flag_type,
        flag_index: base_flag.flag_index.checked_add(location.id).unwrap(),
    })
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
