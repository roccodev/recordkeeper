use std::num::NonZeroUsize;

use game_data::{
    item::{Item, ItemDetails},
    GameData,
};
use recordkeeper::{
    item::{edit::ItemEditor, ItemType},
    SaveData,
};
use yew::prelude::*;

use ybc::{Button, Control, Field, Icon, Tile};
use yew_feather::{Tool, X};

use crate::{
    components::{
        dlc::masha::MashaModal,
        edit::{editor, NumberInput},
        item::HtmlItem,
        select::{Options, SearchSelect},
    },
    data::{Data, Singleton},
    save::SaveContext,
};

editor!(
    pub AmountEditor,
    u16,
    get |editor, save| {
        save.inventory.slots(editor.item_type)[editor.index].amount()
    },
    set |editor, save, new_value| {
        let old_id = save.inventory.slots(editor.item_type)[editor.index].item_id();
        ItemEditor::new(save, editor.item_type, editor.index).set_amount(new_value);
        on_edit_complete(save, &editor.game, old_id);
    },
    capture item_type: ItemType, index: usize, game: Singleton<GameData>
);

#[derive(Properties, PartialEq, Clone)]
pub struct ItemEditorProps {
    pub index: usize,
    pub item_type: ItemType,
    pub items: &'static [Item],
    pub options: Options<HtmlItem>,
}

#[function_component]
pub fn ItemRow(props: &ItemEditorProps) -> Html {
    let data = use_context::<Data>().unwrap();
    let game = data.game();
    let lang = data.to_lang();
    let save_context = use_context::<SaveContext>().unwrap();
    let save = save_context.get();
    let masha_modal = use_state(|| false);

    let ItemEditorProps {
        index,
        item_type,
        items,
        options,
    } = props.clone();

    let slot = save.get_save().inventory.slots(item_type)[index];
    let current = items
        .binary_search_by_key(&(slot.item_id() as u32), |i| i.id)
        .ok();

    let amount_editor = AmountEditor {
        index,
        item_type,
        game: *data.game_ref(),
    };

    let save = save_context.clone();
    let on_select = Callback::from(move |new: usize| {
        let new_id: u16 = items[new].id.try_into().unwrap();
        save.try_edit(move |save| {
            ItemEditor::new(save, item_type, index).set_item_id(new_id)?;
            on_edit_complete(save, game, new_id);
            Ok(())
        })
    });

    let save = save_context.clone();
    let clear_callback = Callback::from(move |_: MouseEvent| {
        save.edit(move |save| {
            ItemEditor::new(save, item_type, index).clear();
            on_edit_complete(save, game, slot.item_id());
        });
    });

    let masha_state = masha_modal.clone();
    let masha_callback = Callback::from(move |_: MouseEvent| {
        masha_state.set(true);
    });
    let masha_state = masha_modal.clone();
    let masha_close_callback = Callback::from(move |_| {
        masha_state.set(false);
    });

    // Buttons:
    // - Clear slot

    html! {
        <>
            <MashaModal item_slot={(*masha_modal).then_some(props.index)} close_callback={masha_close_callback} />
            <tr>
                <th>{props.index.to_string()}</th>
                <td>
                    <Tile classes={classes!("is-align-items-end")}>
                        <Tile classes={classes!("mr-1")}>
                            {if slot.is_valid() {
                                html!(<span class={classes!("recordkeeper-item-id")}>{slot.item_id().to_string()}{"."}</span>)
                            } else {
                                html!()
                            }}
                        </Tile>
                        <Tile classes={classes!("is-10")}>
                            <SearchSelect<HtmlItem>
                                current={current}
                                options={options.clone()}
                                on_select={on_select}
                                lang={lang.clone()}
                            />
                        </Tile>
                    </Tile>
                </td>
                <td>
                    <NumberInput<AmountEditor> editor={amount_editor} />
                </td>
                <td>
                    <Field classes={classes!("has-addons")}>
                        {if slot.is_crafted_accessory() {
                            html! {
                                <Control>
                                    <Button onclick={masha_callback}>
                                        <Icon>
                                            <Tool />
                                        </Icon>
                                    </Button>
                                </Control>
                            }
                        } else {
                            html!()
                        }}
                        <Control>
                            <Button disabled={!slot.is_valid()} onclick={clear_callback}>
                                <Icon>
                                    <X />
                                </Icon>
                            </Button>
                        </Control>
                    </Field>
                </td>
            </tr>
        </>
    }
}

fn on_edit_complete(save: &mut SaveData, game: &GameData, item_id: u16) {
    check_gem_level(save, game, item_id)
}

/// Updates the gem level table when a gem slot is edited.
fn check_gem_level(save: &mut SaveData, game: &GameData, item_id: u16) {
    if game.items.get_item(ItemType::Gem, item_id.into()).is_none() {
        return;
    }
    let mut max_ids = vec![0; game.items.gem_categories().len()];
    for slot in save.inventory.gems.iter() {
        if slot.item_id() == 0 {
            continue;
        }
        let Some(ItemDetails::Gem { category }) = game
            .items
            .get_item(ItemType::Gem, slot.item_id().into())
            .and_then(|i| i.details)
        else {
            continue;
        };
        let max_id = &mut max_ids[(category - 1) as usize];
        if *max_id < slot.item_id() {
            *max_id = slot.item_id();
        }
    }
    for (cat, id) in max_ids.into_iter().enumerate() {
        save.gem_levels
            .set_item(NonZeroUsize::new(cat + 1).unwrap(), id);
    }
}
