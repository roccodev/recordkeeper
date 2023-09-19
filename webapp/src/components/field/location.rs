use game_data::{
    field::{Location, LocationType, Map},
    lang::Nameable,
    GameData,
};
use ybc::{Button, Buttons, Container, Control, Field, Table};
use yew::prelude::*;

use crate::{
    components::{
        character::Selector,
        edit::{CheckboxInput, Editor, FlagEditor, ToBool},
        field::player::{Coord, CoordEditor, Loc, MapIdEditor, MapJumpEditor},
    },
    data::Data,
    lang::Text,
    save::SaveContext,
};

#[derive(Properties, PartialEq)]
struct LocationProps {
    location: Location,
    map_id: usize,
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

    let map_id = *map_state;
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
                        <th><Text path="field_location_actions" /></th>
                    </tr>
                </thead>
                <tbody>
                    {for map.locations.iter().map(|location| {
                        html!(<LocationRow location={*location} map_id={map_id} />)
                    })}
                </tbody>
            </Table>
        </Container>
    }
}

#[function_component]
fn LocationRow(props: &LocationProps) -> Html {
    let data = use_context::<Data>().unwrap();
    let save_context = use_context::<SaveContext>().unwrap();

    let location = props.location;
    let save = save_context.get();

    let spawn_editor = MapJumpEditor {};

    let spawn_callback = |id: u16| {
        let save_context = save_context.clone();
        Callback::from(move |_: MouseEvent| {
            save_context.edit(move |save| spawn_editor.set(save, id))
        })
    };

    let teleport_callback = {
        let save_context = save_context.clone();
        let map = MapIdEditor {};
        let map_id = props.map_id;
        let x = CoordEditor {
            loc: Loc::Player,
            coord: Coord::X,
        };
        let y = CoordEditor {
            loc: Loc::Player,
            coord: Coord::Y,
        };
        let z = CoordEditor {
            loc: Loc::Player,
            coord: Coord::Z,
        };
        location.map_point.map(|point| {
            Callback::from(move |_: MouseEvent| {
                save_context.edit(move |save| {
                    map.set(save, map_id.try_into().unwrap());
                    x.set(save, point.x);
                    y.set(save, point.y);
                    z.set(save, point.z);
                })
            })
        })
    };

    html! {
        <tr>
            <th>{location.id}</th>
            <td>
                <CheckboxInput<ToBool<FlagEditor>> editor={location_to_flag(data.game(), &location)} />
            </td>
            <td><Text path={location_type_lang(location.location_type)}/></td>
            <td>{location.get_name_str(data.lang())}</td>
            <td>
                <Buttons classes={classes!("are-small")}>
                    {teleport_callback.map(|callback| html! {
                        <Button onclick={callback}>
                            <Text path="field_location_teleport" />
                        </Button>
                    })}
                    {location.map_jump.map(|map_jump| html! {
                        <Button disabled={spawn_editor.get(save.get().save()) == map_jump.get()} onclick={spawn_callback(map_jump.get())}>
                            <Text path="field_location_respawn" />
                        </Button>
                    })}
                </Buttons>
            </td>
        </tr>
    }
}

fn location_to_flag(data: &GameData, location: &Location) -> ToBool<FlagEditor> {
    let base_flag = FlagEditor::from(data.manual.flags.location);
    ToBool(FlagEditor {
        flag_type: base_flag.flag_type,
        flag_index: location
            .id
            .checked_sub(1)
            .and_then(|id| base_flag.flag_index.checked_add(id))
            .unwrap(),
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
