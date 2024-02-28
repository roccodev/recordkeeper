use crate::components::item::edit::ItemRow;
use crate::components::item::HtmlItem;
use crate::components::page::{PageControls, PageOrganizer};
use crate::components::select::{Options, SearchSelect};
use crate::data::Data;
use crate::lang::{Lang, Text};
use crate::save::SaveContext;
use game_data::item::Item;
use recordkeeper::item::{Inventory, ItemSlot, ItemType};
use ybc::{Button, Buttons, Control, Field, Table, Tile};
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
    pub items: &'static [Item],
    pub options: Options<HtmlItem>,
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
struct PageChangeProps {
    pub item_type: ItemType,
    pub options: Options<HtmlItem>,
    pub page_state: UseStateHandle<usize>,
}

#[derive(Properties, PartialEq)]
struct FirstEmptyProps {
    pub item_type: ItemType,
    pub page_state: UseStateHandle<usize>,
}

const PAGES_PER_VIEW: usize = 2;
const ROWS_PER_PAGE: usize = 10;

#[function_component]
pub fn ItemInventory() -> Html {
    let item_type = use_state(|| ITEM_TYPES[0]);
    let page = use_state(|| 0);
    let save = use_context::<SaveContext>().unwrap();
    let data = use_context::<Data>().unwrap();
    let num_slots = save.get().get_save().inventory.slots(*item_type).len();

    // Reset page when item type changes
    let page_state = page.clone();
    use_effect_with_deps(move |_| page_state.set(0), *item_type);

    let page_organizer = PageOrganizer::<PAGES_PER_VIEW>::new(ROWS_PER_PAGE, *page, num_slots);

    let items: &'static [Item] = data.game().items.items_by_type(*item_type);
    let options: Options<HtmlItem> = items.iter().copied().map(HtmlItem).collect();

    html! {
        <>
            <Tile classes={classes!("is-align-items-end", "mb-2")}>
                <Tile>
                    <Field>
                        <label class="label"><Text path="item_type" /></label>
                        <Buttons classes={classes!("has-addons")}>
                            {for ITEM_TYPES.iter().map(|ty| {
                                let item_type = item_type.clone();
                                let classes = if ty == &*item_type {
                                    classes!("is-primary", "is-selected")
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
                <Tile classes={classes!("field", "is-grouped", "is-grouped-right", "is-4")}>
                    <ItemFinder item_type={*item_type} page_state={page.clone()} options={options.clone()} />
                    <Control>
                        <FirstEmptySlot item_type={*item_type} page_state={page.clone()} />
                    </Control>
                </Tile>
            </Tile>

            <Tile classes="mb-2">
                {for page_organizer.current_bounds.into_iter().map(|(start, end)| html! {
                    <Tile>
                        <TablePage items={items} options={options.clone()} item_type={*item_type} start={start} end={end} />
                    </Tile>
                })}
            </Tile>

            <PageControls<PAGES_PER_VIEW> organizer={page_organizer} state={page} />
        </>
    }
}

#[function_component]
fn TablePage(props: &TableProps) -> Html {
    let item_type = props.item_type;

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
                    html!(<ItemRow item_type={item_type} index={index} items={props.items} options={props.options.clone()} />)
                })}
            </tbody>
        </Table>
    }
}

#[function_component]
fn ItemFinder(props: &PageChangeProps) -> Html {
    let item_type = props.item_type;
    let page_state = props.page_state.clone();
    let data = use_context::<Data>().unwrap();
    let game_lang = data.to_lang();
    let ui_lang = use_context::<Lang>().unwrap();
    let save = use_context::<SaveContext>().unwrap();

    let options = props.options.clone();
    let on_select = Callback::from(move |i: usize| {
        let item = &options.get(i).0;
        if let Some(index) = index_of_item(&save.get().get_save().inventory, item_type, |slot| {
            u32::from(slot.item_id()) == item.id
        }) {
            let next_page = index / (PAGES_PER_VIEW * ROWS_PER_PAGE);
            page_state.set(next_page);
        }
    });

    html! {
        <SearchSelect<HtmlItem>
            current={None}
            options={props.options.clone()}
            on_select={on_select}
            lang={game_lang}
            placeholder={ui_lang.translate("item_search").to_string()}
        />
    }
}

#[function_component]
fn FirstEmptySlot(props: &FirstEmptyProps) -> Html {
    let item_type = props.item_type;
    let page_state = props.page_state.clone();
    let save = use_context::<SaveContext>().unwrap();

    let on_click = Callback::from(move |_: MouseEvent| {
        if let Some(index) = index_of_item(&save.get().get_save().inventory, item_type, |slot| {
            !slot.is_valid()
        }) {
            let next_page = index / (PAGES_PER_VIEW * ROWS_PER_PAGE);
            page_state.set(next_page);
        }
    });

    html! {
        <Button onclick={on_click}>
            <Text path="item_first_empty" />
        </Button>
    }
}

fn index_of_item(
    inventory: &Inventory,
    item_type: ItemType,
    predicate: impl Fn(&ItemSlot) -> bool,
) -> Option<usize> {
    let items = inventory.slots(item_type);
    items.iter().position(predicate)
}
