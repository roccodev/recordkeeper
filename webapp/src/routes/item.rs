use crate::components::page::PageOrganizer;
use crate::lang::Text;
use crate::save::SaveContext;
use game_data::item::ItemType;
use recordkeeper::item::{Inventory, ItemSlot};
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

const PAGES_PER_VIEW: usize = 2;
const ROWS_PER_PAGE: usize = 10;

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
    let flag_type = props.item_type;

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
                    html! {
                        <tr>
                            <th>{index.to_string()}</th>
                            <td></td>
                            <td></td>
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
