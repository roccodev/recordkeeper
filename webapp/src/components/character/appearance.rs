use std::fmt::Debug;

use game_data::{
    character::{Attachment, Costume},
    lang::Id,
};
use recordkeeper::character::CharacterFlag;
use strum::IntoEnumIterator;
use ybc::{Control, Field, Tile};
use yew::prelude::*;

use crate::{
    components::{
        edit::{editor, CheckboxInput, Editor, NumberInput},
        select::{HtmlName, Options, SearchSelect},
    },
    data::Data,
    lang::Text,
    save::SaveContext,
};

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
    get |editor, save| save.characters[editor.char_idx].costume_id,
    set |editor, save, new| save.characters[editor.char_idx].costume_id = new,
    capture char_idx: usize
);

#[rustfmt::skip]
editor!(
    AttachmentEditor,
    u8,
    get |editor, save| save.characters[editor.char_idx].attachment,
    set |editor, save, new| save.characters[editor.char_idx].attachment = new,
    capture char_idx: usize
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
    pub char_idx: usize,
}

#[derive(Properties, PartialEq)]
struct EditorSelectProps<E: PartialEq, I: PartialEq + 'static> {
    pub editor: E,
    pub options: Options<I>,
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
                <Field classes={classes!("mr-2")}>
                    <label class="label"><Text path="character_dirt" /></label>
                    <Control>
                        <NumberInput<DirtinessEditor> editor={DirtinessEditor { char_idx: props.char_idx }} />
                    </Control>
                </Field>
                <Field classes={classes!("mr-2")}>
                    <label class="label"><Text path="character_costume" /></label>
                    <Control>
                        <EditorSelect<CostumeEditor, Costume>
                            editor={CostumeEditor { char_idx: props.char_idx }}
                            options={Options::from(data.game().characters.costumes(props.char_idx.checked_add(1).unwrap()))}
                        />
                    </Control>
                </Field>
                <Field classes={classes!("mr-2")}>
                    <label class="label"><Text path="character_attachment" /></label>
                    <Control>
                        <EditorSelect<AttachmentEditor, Attachment>
                            editor={AttachmentEditor { char_idx: props.char_idx }}
                            options={Options::from(data.game().characters.attachments())}
                        />
                    </Control>
                </Field>
            </Tile>
            <Tile>
                <Field classes={classes!("is-grouped", "is-grouped-multiline")}>
                    {for CharacterFlag::iter().filter_map(|flag| {
                        Some(html! {
                            <Control>
                                <CheckboxInput<CharacterFlagEditor> editor={CharacterFlagEditor { char_idx: props.char_idx, flag }}>
                                    {" "}<Text path={flag.lang_id()} />
                                </CheckboxInput<CharacterFlagEditor>>
                            </Control>
                        })
                    })}
                </Field>
            </Tile>
        </Tile>
    }
}

/// Select component with a button to empty the field, as well as
/// searchable options.
///
/// Note: only works when the option index = option ID - 1
#[function_component]
fn EditorSelect<E, I>(props: &EditorSelectProps<E, I>) -> Html
where
    E: Editor + PartialEq,
    E::Target: TryInto<usize> + TryFrom<usize>,
    <E::Target as TryInto<usize>>::Error: Debug,
    <E::Target as TryFrom<usize>>::Error: Debug,
    I: Clone + HtmlName + PartialEq + 'static,
{
    let save_context = use_context::<SaveContext>().unwrap();
    let data = use_context::<Data>().unwrap();
    let lang = data.to_lang();

    let save = save_context.get();

    // Conveniently, this is None when the value is 0
    let current = props
        .editor
        .get(save.get().save())
        .try_into()
        .unwrap()
        .checked_sub(1);

    let update = {
        let editor = props.editor;
        let save_context = save_context.clone();
        Callback::from(move |new: usize| {
            save_context
                .edit(move |save| editor.set(save, new.checked_add(1).unwrap().try_into().unwrap()))
        })
    };

    html! {
        <SearchSelect<I>
            current={current}
            options={props.options.clone()}
            on_select={update}
            lang={lang}
        />
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
                CharacterFlag::Dlc4Gem1 => "dlc4_gem_1",
                CharacterFlag::Dlc4Gem2 => "dlc4_gem_2",
                CharacterFlag::Dlc4Gem3 => "dlc4_gem_3",
                CharacterFlag::Dlc4Accessory2 => "dlc4_acc_2",
                CharacterFlag::Dlc4Accessory3 => "dlc4_acc_3",
            }
        )
    }

    fn is_dlc4(&self) -> bool {
        match self {
            Self::HasEyepatch | Self::UnloadDlcCostume => false,
            _ => true,
        }
    }
}
