use game_data::item::Item;
use recordkeeper::item::{edit::ItemEditor, ItemType};
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
    data::Data,
    save::SaveContext,
};

editor!(
    pub AmountEditor,
    u16,
    get |editor, save| {
        save.inventory.slots(editor.item_type)[editor.index].amount()
    },
    set |editor, save, new_value| {
        ItemEditor::new(save, editor.item_type, editor.index).set_amount(new_value)
    },
    capture item_type: ItemType, index: usize
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

    let slot = &save.get().save().inventory.slots(item_type)[index];
    let current = items
        .binary_search_by_key(&(slot.item_id() as u32), |i| i.id)
        .ok();

    let amount_editor = AmountEditor { index, item_type };

    let save = save_context.clone();
    let on_select = Callback::from(move |new: usize| {
        let new_id: u16 = items[new].id.try_into().unwrap();
        save.try_edit(move |save| Ok(ItemEditor::new(save, item_type, index).set_item_id(new_id)?));
    });

    let save = save_context.clone();
    let clear_callback = Callback::from(move |_: MouseEvent| {
        save.edit(move |save| ItemEditor::new(save, item_type, index).clear());
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
