use game_data::character::Class;
use ybc::{Control, Field, Tile};
use yew::prelude::*;

use crate::{
    components::{
        edit::{editor, Editor, NumberInput},
        select::UpdateSelector,
    },
    data::Data,
    lang::Text,
    save::SaveContext,
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

#[rustfmt::skip]
editor!(
    SelectedClassEditor,
    u8,
    get |editor, save| save.characters[editor.char_idx].selected_class,
    set |editor, save, new_value| save.characters[editor.char_idx].selected_class = new_value,
    capture char_idx: usize
);

#[function_component]
pub fn CharacterStats(props: &CharacterProps) -> Html {
    let char_idx = props.char_id.checked_sub(1).unwrap() as usize;

    let data = use_context::<Data>().unwrap();
    let save_context = use_context::<SaveContext>().unwrap();
    let selected_class_editor = SelectedClassEditor { char_idx };
    let selected_class: u32 = selected_class_editor
        .get(save_context.get().get_save())
        .into();

    let update_selected_class = {
        let save_context = save_context.clone();
        Callback::from(move |class_id: u32| {
            save_context
                .edit(move |save| selected_class_editor.set(save, class_id.try_into().unwrap()))
        })
    };

    html! {
        <Tile classes={classes!("is-parent")}>
            <div>
                <Field classes={classes!("mr-2")}>
                    <label class="label"><Text path="character_level" /></label>
                    <Control>
                        <NumberInput<LevelEditor> editor={LevelEditor { char_idx }} />
                    </Control>
                </Field>
                <Field classes={classes!("mr-2")}>
                    <label class="label"><Text path="character_bexp" /></label>
                    <Control>
                        <NumberInput<BonusExpEditor> editor={BonusExpEditor { char_idx }} />
                    </Control>
                </Field>
                <Field>
                    <label class="label"><Text path="character_selected_class" /></label>
                    <Control>
                        <UpdateSelector<Class> update={update_selected_class} current={selected_class} values={data.game().characters.classes()} />
                    </Control>
                </Field>
            </div>
            <div>
                <Field classes={classes!("mr-2")}>
                    <label class="label"><Text path="character_exp" /></label>
                    <Control>
                        <NumberInput<ExpEditor> editor={ExpEditor { char_idx }} />
                    </Control>
                </Field>
                <Field>
                    <label class="label"><Text path="character_arrival_level" /></label>
                    <Control>
                        <NumberInput<ArrivalLevelEditor> editor={ArrivalLevelEditor { char_idx }} />
                    </Control>
                </Field>
            </div>
        </Tile>
    }
}
