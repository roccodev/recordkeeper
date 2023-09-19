use game_data::field::Map;
use recordkeeper::{SaveData, SaveFlag};
use ybc::{Control, Field, Tile, Title};
use yew::prelude::*;

use super::MetaFlagEditor;
use crate::{
    components::{
        character::UpdateSelector,
        edit::{editor, CheckboxInput, Editor, NumberInput, StringInput},
    },
    data::Data,
    lang::Text,
    save::SaveContext,
    util::FiniteF32,
};

#[rustfmt::skip]
editor!(
    pub MapIdEditor,
    u16,
    get |_, save| save.map_id,
    set |_, save, new| save.map_id = new
);

#[rustfmt::skip]
editor!(
    pub CoordEditor,
    FiniteF32,
    get |editor, save| coord(save, *editor),
    set |editor, save, new| *coord_mut(save, *editor) = new.into(),
    capture loc: Loc, coord: Coord
);

#[rustfmt::skip]
editor!(
    GoldEditor,
    u32,
    get |_, save| save.gold,
    set |_, save, new| save.gold = new
);

#[rustfmt::skip]
editor!(
    CylinderEditor,
    u16,
    get |_, save| save.ether_cylinder_progress,
    set |_, save, new| save.ether_cylinder_progress = new
);

#[rustfmt::skip]
editor!(
    CylinderDxEditor,
    u16,
    get |_, save| save.ether_cylinder_dx_progress,
    set |_, save, new| save.ether_cylinder_dx_progress = new
);

#[rustfmt::skip]
editor!(
    pub MapJumpEditor,
    u16,
    get |_, save| save.respawn_point,
    set |_, save, new| save.respawn_point = new
);

#[derive(Copy, Clone, PartialEq)]
pub enum Coord {
    X,
    Y,
    Z,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Loc {
    Player,
    Ship,
}

#[function_component]
pub fn PlayerLoc() -> Html {
    let data = use_context::<Data>().unwrap();
    let save_context = use_context::<SaveContext>().unwrap();
    let maps = data.game().field.maps();

    let save = save_context.get();
    let map_id_editor = MapIdEditor {};
    let update_map_id = {
        let save_context = save_context.clone();
        Callback::from(move |new: usize| {
            save_context.edit(move |save| map_id_editor.set(save, new.try_into().unwrap()))
        })
    };

    html! {
        <Tile classes={classes!("is-child", "notification")}>
            <Title><Text path="field_player_pos" /></Title>

            <Field>
                <label class="label"><Text path="field_map" /></label>
                <Control>
                    <UpdateSelector<Map> current={map_id_editor.get(save.get().save()) as usize} values={maps} update={update_map_id} />
                </Control>
            </Field>

            <Field>
                <label class="label"><Text path="x" /></label>
                <Control>
                    <StringInput<FiniteF32, CoordEditor> editor={CoordEditor { loc: Loc::Player, coord: Coord::X }} />
                </Control>
            </Field>

            <Field>
                <label class="label"><Text path="y" /></label>
                <Control>
                    <StringInput<FiniteF32, CoordEditor> editor={CoordEditor { loc: Loc::Player, coord: Coord::Y }} />
                </Control>
            </Field>

            <Field>
                <label class="label"><Text path="z" /></label>
                <Control>
                    <StringInput<FiniteF32, CoordEditor> editor={CoordEditor { loc: Loc::Player, coord: Coord::Z }} />
                </Control>
            </Field>
        </Tile>
    }
}

#[function_component]
pub fn ShipLoc() -> Html {
    html! {
        <Tile classes={classes!("is-child", "notification")}>
            <Title><Text path="field_ship_pos" /></Title>

            <Field>
                <label class="label"><Text path="x" /></label>
                <Control>
                    <StringInput<FiniteF32, CoordEditor> editor={CoordEditor { loc: Loc::Ship, coord: Coord::X }} />
                </Control>
            </Field>

            <Field>
                <label class="label"><Text path="y" /></label>
                <Control>
                    <StringInput<FiniteF32, CoordEditor> editor={CoordEditor { loc: Loc::Ship, coord: Coord::Y }} />
                </Control>
            </Field>

            <Field>
                <label class="label"><Text path="z" /></label>
                <Control>
                    <StringInput<FiniteF32, CoordEditor> editor={CoordEditor { loc: Loc::Ship, coord: Coord::Z }} />
                </Control>
            </Field>

            <Field>
                <Control>
                    <CheckboxInput<MetaFlagEditor> editor={MetaFlagEditor { flag: SaveFlag::AboardShip }}>
                        {" "}<Text path="field_aboard_ship" />
                    </CheckboxInput<MetaFlagEditor>>
                </Control>
            </Field>
        </Tile>
    }
}

#[function_component]
pub fn FieldStats() -> Html {
    html! {
        <Tile classes={classes!("is-child", "notification")}>
            <Title><Text path="field_stats" /></Title>

            <Field>
                <label class="label"><Text path="field_gold" /></label>
                <Control>
                    <NumberInput<GoldEditor> editor={GoldEditor {}} />
                </Control>
            </Field>

            <Field>
                <label class="label"><Text path="field_respawn_point" /></label>
                <Control>
                    <NumberInput<MapJumpEditor> editor={MapJumpEditor {}} />
                </Control>
            </Field>

            <Field>
                <label class="label"><Text path="field_ether_progress" /></label>
                <Control>
                    <NumberInput<CylinderEditor> editor={CylinderEditor {}} max={100} />
                </Control>
            </Field>

            <Field>
                <label class="label"><Text path="field_ether_progress_dx" /></label>
                <Control>
                    <NumberInput<CylinderDxEditor> editor={CylinderDxEditor {}} max={100} />
                </Control>
            </Field>
        </Tile>
    }
}

fn coord(save: &SaveData, editor: CoordEditor) -> FiniteF32 {
    let pos = match editor.loc {
        Loc::Player => &save.player_pos,
        Loc::Ship => &save.ship_pos,
    };
    match editor.coord {
        Coord::X => pos.x,
        Coord::Y => pos.y,
        Coord::Z => pos.z,
    }
    .try_into()
    .expect("non-finite coordinate")
}

fn coord_mut(save: &mut SaveData, editor: CoordEditor) -> &mut f32 {
    let pos = match editor.loc {
        Loc::Player => &mut save.player_pos,
        Loc::Ship => &mut save.ship_pos,
    };
    match editor.coord {
        Coord::X => &mut pos.x,
        Coord::Y => &mut pos.y,
        Coord::Z => &mut pos.z,
    }
}
