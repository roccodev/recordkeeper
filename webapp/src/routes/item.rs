use crate::components::edit::{editor, NumberInput};
use crate::components::page::PageOrganizer;
use crate::components::select::{HtmlName, Options, SearchSelect};
use crate::data::Data;
use crate::lang::Text;
use crate::save::{EditAction, SaveContext};
use game_data::item::{Item, ItemType};
use game_data::lang::Nameable;
use game_data::LanguageData;
use recordkeeper::item::{Inventory, ItemSlot};
use std::rc::Rc;
use ybc::{Button, Buttons, Container, Field, Table, Tile};
use yew::prelude::*;

/// List of supported item types, in order of importance
static ITEM_TYPES: &[ItemType] = &[
    ItemType::Collection,
    ItemType::Accessory,
    ItemType::Precious,
    ItemType::Gem,
    ItemType::Cylinder,
];

#[derive(Properties, PartialEq)]
struct TableProps {
    pub item_type: ItemType,
    pub start: usize,
    pub end: usize,
}

#[derive(Properties, PartialEq)]
struct ItemProps {
    pub item_type: ItemType,
    pub slot: usize,
}

#[derive(Properties, PartialEq)]
struct ItemDisplayProps {
    pub item: Item,
}

#[derive(Clone, PartialEq, Copy)]
struct HtmlItem(Item);

const PAGES_PER_VIEW: usize = 2;
const ROWS_PER_PAGE: usize = 10;

editor!(
    pub AmountEditor,
    u16,
    get |editor, save| { get_item_field(&save.inventory, editor.item_type)[editor.index].amount },
    set |editor, save, new_value| { get_item_field_mut(&mut save.inventory, editor.item_type)[editor.index].amount = new_value },
    assert |editor, value| { Ok(()) },
    capture item_type: ItemType, index: usize
);

fn get_item_field(inventory: &Inventory, item_type: ItemType) -> &[ItemSlot] {
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

#[function_component]
pub fn ItemInventory() -> Html {
    let item_type = use_state(|| ITEM_TYPES[0]);
    let page = use_state(|| 0);
    let save = use_context::<SaveContext>().unwrap();
    let num_slots = get_item_field(&save.get().get().save().inventory, *item_type).len();

    let page_organizer = PageOrganizer::<PAGES_PER_VIEW>::new(ROWS_PER_PAGE, *page, num_slots);

    html! {
        <Container>
            <Tile classes={classes!("is-align-items-end")}>
                <Tile>
                    <Field>
                        <label class="label"><Text path="item_type" /></label>
                        <Buttons classes={classes!("has-addons")}>
                            {for ITEM_TYPES.into_iter().map(|ty| {
                                let item_type = item_type.clone();
                                let classes = if ty == &*item_type {
                                    classes!("is-info", "is-selected")
                                } else {
                                    classes!("")
                                };
                                let on_click = Callback::from(move |_: MouseEvent| {
                                    item_type.set(*ty);
                                });

                                html! {
                                    <Button onclick={on_click} classes={classes}>
                                        <Text path={format!("item_type_{}", ty.lang_id())} />
                                    </Button>
                                }
                            })}
                        </Buttons>
                    </Field>
                </Tile>
                <Tile classes={classes!("is-4")}>
                </Tile>
            </Tile>

            <Tile>
                {for page_organizer.current_bounds.into_iter().map(|(start, end)| html! {
                    <Tile>
                        <TablePage item_type={*item_type} start={start} end={end} />
                    </Tile>
                })}
            </Tile>
        </Container>
    }
}

#[function_component]
fn TablePage(props: &TableProps) -> Html {
    let data = use_context::<Data>().unwrap();
    let save_context = use_context::<SaveContext>().unwrap();
    let save = save_context.get();
    let item_type = props.item_type;
    let lang = data.to_lang();

    let items: Rc<[Item]> = data
        .game()
        .items
        .items_by_type(item_type)
        .iter()
        .copied()
        .collect();
    let options: Options<HtmlItem> = items
        .clone()
        .into_iter()
        .copied()
        .map(|i| HtmlItem(i))
        .collect();

    html! {
        <Table classes={classes!("is-fullwidth")}>
            <thead>
                <tr>
                    <th><Text path="item_slot_index" /></th>
                    <th><Text path="item_item" /></th>
                    <th><Text path="item_amount" /></th>
                    <th><Text path="item_actions" /></th>
                </tr>
            </thead>

            <tbody>
                {for (props.start..=props.end).map(|index| {
                    let slot = &get_item_field(&save.get().save().inventory, item_type)[index];
                    let current = items.binary_search_by_key(&(slot.item_id as u32), |i| i.id).ok();

                    let amount_editor = AmountEditor { index, item_type };

                    let items = items.clone();
                    let save_context = save_context.clone();
                    let on_select = Callback::from(move |new: usize| {
                        let new_id: u16 = items[new].id.try_into().unwrap();
                        save_context.submit_action(EditAction::Edit(Box::new(move |save| {
                            get_item_field_mut(&mut save.inventory, item_type)[index].item_id = new_id
                        })));
                    });

                    html! {
                        <tr>
                            <th>{index.to_string()}</th>
                            <td>
                                <SearchSelect<HtmlItem>
                                    current={current}
                                    options={options.clone()}
                                    on_select={on_select}
                                    lang={lang.clone()}
                                />
                            </td>
                            <td><NumberInput<AmountEditor> editor={amount_editor} /></td>
                            <td></td>
                        </tr>
                    }
                })}
            </tbody>
        </Table>
    }
}

#[function_component]
fn ItemRow() -> Html {
    html! {}
}

#[function_component]
fn ItemDisplay(props: &ItemDisplayProps) -> Html {
    let data = use_context::<Data>().unwrap();

    html! {
        <>
            <span><small>{props.item.id}{". "}</small></span>
            <span>{props.item.get_name(data.lang())}</span>
            <span><small>{" ("}<Text path={format!("item_rarity_{}", props.item.rarity.lang_id())} />{")"}</small></span>
        </>
    }
}

impl HtmlName for HtmlItem {
    fn get_name_html(&self, language: &LanguageData) -> Html {
        html!(<ItemDisplay item={self.0} />)
    }

    fn get_name_for_filter<'a, 'b: 'a>(&'a self, language: &'b LanguageData) -> Option<&'a str> {
        self.0.get_name(language)
    }
}
