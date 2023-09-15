use recordkeeper::SaveData;
use ybc::{Control, Field, Tile, Title};
use yew::prelude::*;

use crate::{
    components::edit::{editor, StringInput},
    lang::Text,
};

#[rustfmt::skip]
editor!(
    CoordEditor,
    f32,
    get |editor, save| coord(save, *editor),
    set |editor, save, new| *coord_mut(save, *editor) = new,
    capture loc: Loc, coord: Coord
);

#[derive(Copy, Clone, PartialEq)]
enum Coord {
    X,
    Y,
    Z,
}

#[derive(Copy, Clone, PartialEq)]
enum Loc {
    Player,
    Ship,
}

#[function_component]
pub fn PlayerLoc() -> Html {
    html! {
        <Tile classes={classes!("is-child", "notification")}>
            <Title><Text path="field_player_pos" /></Title>

            <Field>
                <label class="label"><Text path="x" /></label>
                <Control>
                    <StringInput<f32, CoordEditor> editor={CoordEditor { loc: Loc::Player, coord: Coord::X }} />
                </Control>
            </Field>

            <Field>
                <label class="label"><Text path="y" /></label>
                <Control>
                    <StringInput<f32, CoordEditor> editor={CoordEditor { loc: Loc::Player, coord: Coord::Y }} />
                </Control>
            </Field>

            <Field>
                <label class="label"><Text path="z" /></label>
                <Control>
                    <StringInput<f32, CoordEditor> editor={CoordEditor { loc: Loc::Player, coord: Coord::Z }} />
                </Control>
            </Field>
        </Tile>
    }
}

fn coord(save: &SaveData, editor: CoordEditor) -> f32 {
    let pos = match editor.loc {
        Loc::Player => &save.player_pos,
        Loc::Ship => &save.ship_pos,
    };
    match editor.coord {
        Coord::X => pos.x,
        Coord::Y => pos.y,
        Coord::Z => pos.z,
    }
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
