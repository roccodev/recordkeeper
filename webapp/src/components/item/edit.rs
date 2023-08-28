use std::rc::Rc;

use game_data::item::{Item, ItemType};
use recordkeeper::item::{Inventory, ItemSlot};
use yew::prelude::*;

use ybc::{Button, Buttons, Icon};
use yew_feather::X;

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
    set |editor, save, new_value| { get_item_field_mut(&mut save.inventory, editor.item_type)[editor.index].amount = new_value },
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
            get_item_field_mut(&mut save.inventory, item_type)[index].item_id = new_id
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
                <SearchSelect<HtmlItem>
                    current={current}
                    options={options.clone()}
                    on_select={on_select}
                    lang={lang.clone()}
                />
            </td>
            <td>
                <NumberInput<AmountEditor> editor={amount_editor} />
            </td>
            <td>
                <Buttons>
                    <Button disabled={!slot.is_valid()} onclick={clear_callback}>
                        <Icon>
                            <X />
                        </Icon>
                    </Button>
                </Buttons>
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
