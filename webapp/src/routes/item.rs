use crate::components::item::edit::{get_item_field, ItemRow};
use crate::components::item::HtmlItem;
use crate::components::page::{PageControls, PageOrganizer};
use crate::components::select::Options;
use crate::data::Data;
use crate::lang::Text;
use crate::save::SaveContext;
use game_data::item::{Item, ItemType};
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

const PAGES_PER_VIEW: usize = 2;
const ROWS_PER_PAGE: usize = 10;

#[function_component]
pub fn ItemInventory() -> Html {
    let item_type = use_state(|| ITEM_TYPES[0]);
    let page = use_state(|| 0);
    let save = use_context::<SaveContext>().unwrap();
    let num_slots = get_item_field(&save.get().get().save().inventory, *item_type).len();

    // Reset page when item type changes
    let page_state = page.clone();
    use_effect_with_deps(move |_| page_state.set(0), *item_type);

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

            <PageControls<PAGES_PER_VIEW> organizer={page_organizer} state={page} />
        </Container>
    }
}

#[function_component]
fn TablePage(props: &TableProps) -> Html {
    let data = use_context::<Data>().unwrap();
    let item_type = props.item_type;

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
                    html!(<ItemRow item_type={item_type} index={index} items={items.clone()} options={options.clone()} />)
                })}
            </tbody>
        </Table>
    }
}
