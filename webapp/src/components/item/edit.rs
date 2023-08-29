use std::rc::Rc;

use game_data::item::{Item, ItemType};
use recordkeeper::{
    dlc::CRAFTED_ITEM_ID,
    item::{Inventory, ItemSlot},
};
use yew::prelude::*;

use ybc::{Button, Control, Field, Icon, Tile};
use yew_feather::{Tool, X};

use crate::{
    components::{
        edit::{editor, NumberInput},
        item::HtmlItem,
        select::{Options, SearchSelect},
    },
    data::Data,
    save::{EditAction, SaveContext},
};

editor!(
    pub AmountEditor,
    u16,
    get |editor, save| { get_item_field(&save.inventory, editor.item_type)[editor.index].amount },
    set |editor, save, new_value| {
        get_item_field_mut(&mut save.inventory, editor.item_type)[editor.index].edit(
            editor.index as u16,
            editor.item_type as u32,
            |i| i.amount = new_value
        )
    },
    assert |editor, value| { Ok(()) },
    capture item_type: ItemType, index: usize
);

#[derive(Properties, PartialEq, Clone)]
pub struct ItemEditorProps {
    pub index: usize,
    pub item_type: ItemType,
    pub items: Rc<[Item]>,
    pub options: Options<HtmlItem>,
}

#[function_component]
pub fn ItemRow(props: &ItemEditorProps) -> Html {
    let data = use_context::<Data>().unwrap();
    let lang = data.to_lang();
    let save_context = use_context::<SaveContext>().unwrap();
    let save = save_context.get();

    let ItemEditorProps {
        index,
        item_type,
        items,
        options,
    } = props.clone();

    let slot = &get_item_field(&save.get().save().inventory, item_type)[index];
    let current = items
        .binary_search_by_key(&(slot.item_id as u32), |i| i.id)
        .ok();

    let amount_editor = AmountEditor { index, item_type };

    let items = items.clone();
    let save = save_context.clone();
    let on_select = Callback::from(move |new: usize| {
        let new_id: u16 = items[new].id.try_into().unwrap();
        save.submit_action(EditAction::Edit(Box::new(move |save| {
            get_item_field_mut(&mut save.inventory, item_type)[index].edit(
                index as u16,
                item_type as u32,
                |i| {
                    i.item_id = new_id;
                    i.amount = 1;
                },
            )
        })));
    });

    let save = save_context.clone();
    let clear_callback = Callback::from(move |_: MouseEvent| {
        save.submit_action(EditAction::Edit(Box::new(move |save| {
            get_item_field_mut(&mut save.inventory, item_type)[index].clear();
        })));
    });

    // Buttons:
    // - Clear slot

    html! {
        <tr>
            <th>{props.index.to_string()}</th>
            <td>
                <Tile classes={classes!("is-align-items-end")}>
                    <Tile classes={classes!("mr-1")}>
                        {if slot.is_valid() {
                            html!(<span class={classes!("recordkeeper-item-id")}>{slot.item_id.to_string()}{"."}</span>)
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
                    {if u32::from(slot.item_id) == CRAFTED_ITEM_ID {
                        html! {
                            <Control>
                                <Button>
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
    }
}

pub fn get_item_field(inventory: &Inventory, item_type: ItemType) -> &[ItemSlot] {
    match item_type {
        ItemType::Cylinder => &inventory.cylinders,
        ItemType::Gem => &inventory.gems,
        ItemType::Collection => &inventory.collectibles,
        ItemType::Collectopedia => &[],
        ItemType::Info => &inventory.infos,
        ItemType::Accessory => &inventory.accessories,
        ItemType::Precious => &inventory.key_items,
        ItemType::Exchange => &inventory.exchange,
        ItemType::Extra => &inventory.extra,
    }
}

fn get_item_field_mut(inventory: &mut Inventory, item_type: ItemType) -> &mut [ItemSlot] {
    match item_type {
        ItemType::Cylinder => &mut inventory.cylinders,
        ItemType::Gem => &mut inventory.gems,
        ItemType::Collection => &mut inventory.collectibles,
        ItemType::Collectopedia => &mut [],
        ItemType::Info => &mut inventory.infos,
        ItemType::Accessory => &mut inventory.accessories,
        ItemType::Precious => &mut inventory.key_items,
        ItemType::Exchange => &mut inventory.exchange,
        ItemType::Extra => &mut inventory.extra,
    }
}
