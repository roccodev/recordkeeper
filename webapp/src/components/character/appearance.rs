use game_data::{
    character::{Attachment, Costume},
    IdInt,
};
use recordkeeper::character::CharacterFlag;
use strum::IntoEnumIterator;
use ybc::{Control, Field, Tile};
use yew::prelude::*;

use crate::{
    components::{
        edit::{editor, CheckboxInput, NumberInput, SearchInput},
        select::Options,
    },
    data::Data,
    lang::Text,
};

use super::CharacterAccessor;

#[rustfmt::skip]
editor!(
    DirtinessEditor,
    u8,
    get |editor, save| save.characters[editor.char_idx].dirty_level,
    set |editor, save, new| save.characters[editor.char_idx].dirty_level = new,
    capture char_idx: usize
);

#[rustfmt::skip]
editor!(
    CostumeEditor,
    u16,
    get |editor, save| editor.char.get_costume_id(save),
    set |editor, save, new| editor.char.set_costume_id(save, new),
    capture char: CharacterAccessor
);

#[rustfmt::skip]
editor!(
    AttachmentEditor,
    u8,
    get |editor, save| editor.char.get_attachment(save),
    set |editor, save, new| editor.char.set_attachment(save, new),
    capture char: CharacterAccessor
);

#[rustfmt::skip]
editor!(
    CharacterFlagEditor,
    bool,
    get |editor, save| save.characters[editor.char_idx].is_flag_set(editor.flag),
    set |editor, save, new| save.characters[editor.char_idx].set_flag(editor.flag, new),
    capture char_idx: usize, flag: CharacterFlag
);

#[derive(Properties, PartialEq)]
pub struct AppearanceProps {
    pub accessor: CharacterAccessor,
    pub char_id: IdInt,
}

trait FlagBox {
    fn lang_id(&self) -> String;
    fn is_dlc4(&self) -> bool;
}

#[function_component]
pub fn Appearance(props: &AppearanceProps) -> Html {
    let data = use_context::<Data>().unwrap();

    html! {
        <Tile classes={classes!("is-parent", "is-vertical")}>
            <Tile>
                {if let CharacterAccessor::Save { idx } = props.accessor {
                    html! {
                        <Field classes={classes!("mr-2")}>
                            <label class="label"><Text path="character_dirt" /></label>
                            <Control>
                                <NumberInput<DirtinessEditor> editor={DirtinessEditor { char_idx: idx }} />
                            </Control>
                        </Field>
                    }
                } else { html!() }}
                <Field classes={classes!("mr-2")}>
                    <label class="label"><Text path="character_costume" /></label>
                    <Control>
                        <SearchInput<CostumeEditor, Costume>
                            editor={CostumeEditor { char: props.accessor }}
                            options={Options::from(data.game().characters.costumes(props.char_id))}
                        />
                    </Control>
                </Field>
                <Field classes={classes!("mr-2")}>
                    <label class="label"><Text path="character_attachment" /></label>
                    <Control>
                        <SearchInput<AttachmentEditor, Attachment>
                            editor={AttachmentEditor { char: props.accessor }}
                            options={Options::from(data.game().characters.attachments())}
                        />
                    </Control>
                </Field>
            </Tile>
            {if let CharacterAccessor::Save { idx } = props.accessor {
                html! {
                    <Tile>
                        <Field classes={classes!("is-grouped", "is-grouped-multiline")}>
                            {for CharacterFlag::iter().map(|flag| html!{
                                <Control>
                                    <CheckboxInput<CharacterFlagEditor> editor={CharacterFlagEditor { char_idx: idx, flag }}>
                                        {" "}<Text path={flag.lang_id()} />
                                    </CheckboxInput<CharacterFlagEditor>>
                                </Control>
                            })}
                        </Field>
                    </Tile>
                }
            } else { html!() }}
        </Tile>
    }
}

impl FlagBox for CharacterFlag {
    fn lang_id(&self) -> String {
        format!(
            "character_flag_{}",
            match self {
                CharacterFlag::UnloadDlcCostume => "unload_dlc_costume",
                CharacterFlag::HasEyepatch => "eyepatch",
                CharacterFlag::Dlc4MasterArt1 => "dlc4_ma_1",
                CharacterFlag::Dlc4MasterArt2 => "dlc4_ma_2",
                CharacterFlag::Dlc4MasterArt3 => "dlc4_ma_3",
                CharacterFlag::Dlc4Gem2 => "dlc4_gem_2",
                CharacterFlag::Dlc4Gem3 => "dlc4_gem_3",
                CharacterFlag::Dlc4Accessory2 => "dlc4_acc_2",
                CharacterFlag::Dlc4Accessory3 => "dlc4_acc_3",
            }
        )
    }

    fn is_dlc4(&self) -> bool {
        !matches!(self, Self::HasEyepatch | Self::UnloadDlcCostume)
    }
}
