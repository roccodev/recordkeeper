use game_data::lang::{Filterable, Id};
use ybc::{Control, Field, Section, Tile};
use yew::prelude::*;

use crate::{
    components::edit::{editor, NumberInput},
    lang::Text,
};

use super::CharacterProps;

#[rustfmt::skip]
editor!(
    pub LevelEditor,
    u32,
    get |editor, save| save.characters[editor.char_idx].level,
    set |editor, save, new_value| save.characters[editor.char_idx].level = new_value,
    capture char_idx: usize
);

#[rustfmt::skip]
editor!(
    ArrivalLevelEditor,
    u8,
    get |editor, save| save.characters[editor.char_idx].arrival_level,
    set |editor, save, new_value| save.characters[editor.char_idx].arrival_level = new_value,
    capture char_idx: usize
);

#[rustfmt::skip]
editor!(
    ExpEditor,
    u32,
    get |editor, save| save.characters[editor.char_idx].exp,
    set |editor, save, new_value| save.characters[editor.char_idx].exp = new_value,
    capture char_idx: usize
);

#[rustfmt::skip]
editor!(
    BonusExpEditor,
    u32,
    get |editor, save| save.characters[editor.char_idx].bonus_exp,
    set |editor, save, new_value| save.characters[editor.char_idx].bonus_exp = new_value,
    capture char_idx: usize
);

#[function_component]
pub fn CharacterStats(props: &CharacterProps) -> Html {
    let char_idx = props.char_id.checked_sub(1).unwrap();

    html! {
        <div>
            <Tile classes={classes!("is-parent")} >
                <Tile>
                    <Field>
                        <label class="label"><Text path="character_level" /></label>
                        <Control>
                            <NumberInput<LevelEditor> editor={LevelEditor { char_idx }} />
                        </Control>
                    </Field>
                </Tile>
                <Tile>
                    <Field>
                        <label class="label"><Text path="character_exp" /></label>
                        <Control>
                            <NumberInput<ExpEditor> editor={ExpEditor { char_idx }} />
                        </Control>
                    </Field>
                </Tile>
            </Tile>
            <Tile classes={classes!("is-parent")}>
                <Tile>
                    <Field>
                        <label class="label"><Text path="character_bexp" /></label>
                        <Control>
                            <NumberInput<BonusExpEditor> editor={BonusExpEditor { char_idx }} />
                        </Control>
                    </Field>
                </Tile>
                <Tile>
                    <Field>
                        <label class="label"><Text path="character_arrival_level" /></label>
                        <Control>
                            <NumberInput<ArrivalLevelEditor> editor={ArrivalLevelEditor { char_idx }} />
                        </Control>
                    </Field>
                </Tile>
            </Tile>
        </div>
    }
}
