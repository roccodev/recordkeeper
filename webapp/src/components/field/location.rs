use game_data::{
    field::{Location, LocationType, Map},
    lang::Nameable,
    GameData,
};
use recordkeeper::SaveData;
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
    landmark_count: FlagEditor,
    secret_count: FlagEditor,
}

#[derive(Clone, Copy, PartialEq)]
struct LocationVisitEditor {
    location_type: LocationType,
    visited: ToBool<FlagEditor>,
    landmark_count_editor: FlagEditor,
    secret_count_editor: FlagEditor,
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

    let landmark_count_editor = FlagEditor::from(data.game().manual.flags.landmark_count);
    let secret_area_count_editor = FlagEditor::from(data.game().manual.flags.secret_count);

    let set_all = |val: bool| {
        let save_context = save_context.clone();
        let game = data.game();

        Callback::from(move |_: MouseEvent| {
            save_context.edit(move |save| {
                for loc in map.locations.iter() {
                    let flag = LocationVisitEditor::new(
                        game,
                        loc,
                        landmark_count_editor,
                        secret_area_count_editor,
                    );
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
                        html! {
                            <LocationRow
                                location={*location}
                                map_id={map_id}
                                landmark_count={landmark_count_editor}
                                secret_count={secret_area_count_editor}
                            />
                        }
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
                <CheckboxInput<LocationVisitEditor> editor={LocationVisitEditor::new(data.game(), &location, props.landmark_count, props.secret_count)} />
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

impl LocationVisitEditor {
    pub fn new(
        game: &GameData,
        location: &Location,
        landmark_count: FlagEditor,
        secret_count: FlagEditor,
    ) -> Self {
        let base_flag = FlagEditor::from(game.manual.flags.location);
        let editor = ToBool(FlagEditor {
            flag_type: base_flag.flag_type,
            flag_index: location
                .id
                .checked_sub(1)
                .and_then(|id| base_flag.flag_index.checked_add(id))
                .unwrap(),
        });
        Self {
            location_type: location.location_type,
            visited: editor,
            landmark_count_editor: landmark_count,
            secret_count_editor: secret_count,
        }
    }
}

impl Editor for LocationVisitEditor {
    type Target = bool;

    fn get(&self, save: &SaveData) -> Self::Target {
        self.visited.get(save)
    }

    fn set(&self, save: &mut SaveData, new: Self::Target) {
        let current = self.get(save);
        self.visited.set(save, new);

        if current == new {
            return;
        }

        let editor = match self.location_type {
            LocationType::Landmark => &self.landmark_count_editor,
            LocationType::SecretArea => &self.secret_count_editor,
            _ => return,
        };
        let count = editor.get(save);
        editor.set(
            save,
            if new {
                count.saturating_add(1)
            } else {
                count.saturating_sub(1)
            },
        );
    }
}
