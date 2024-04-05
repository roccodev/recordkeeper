use game_data::dlc::map::Dlc4Region;
use game_data::dlc::pedia::{Dlc4Collepedia, Enemypedia, PediaItem, PediaStatus, PediaValue};
use game_data::dlc::Regional;
use recordkeeper::SaveData;
use ybc::{Button, Control, Field, Table, Tile};
use yew::prelude::*;

use crate::components::edit::{editor, EnumInput, FlagEditor, NumberInput};
use crate::components::page::{PageControls, PageOrganizer};
use crate::components::NON_BREAKING_SPACE;
use crate::save::SaveContext;
use crate::{
    components::{edit::Editor, select::Selector},
    data::Data,
    lang::Text,
    ToHtml,
};

#[derive(Properties, PartialEq)]
pub struct PediaProps<T: PartialEq + 'static> {
    items: &'static Regional<T>,
    is_collepedia: bool,
}

#[derive(Properties, PartialEq)]
struct PediaRowProps<T: PartialEq + 'static> {
    item: &'static T,
}

#[derive(Properties, PartialEq)]
pub struct PediaBulkProps<T: PartialEq + 'static> {
    items: &'static [T],
    on: bool,
}

#[derive(PartialEq, Copy, Clone)]
pub struct PediaStatusEditor(FlagEditor);

#[rustfmt::skip]
editor!(
    EnemypediaEditor,
    u8,
    get |editor, save| save.dlc4.get_enemypedia_count(editor.index),
    set |editor, save, new| save.dlc4.set_enemypedia_count(editor.index, new),
    capture index: usize
);

const PAGES_PER_VIEW: usize = 2;
const ROWS_PER_PAGE: usize = 12;

#[function_component]
pub fn CollepediaPage() -> Html {
    let data = use_context::<Data>().unwrap();
    html!(<PediaPage<Dlc4Collepedia> items={&data.game().dlc.collepedia} is_collepedia={true} />)
}

#[function_component]
pub fn EnemypediaPage() -> Html {
    let data = use_context::<Data>().unwrap();
    html!(<PediaPage<Enemypedia> items={&data.game().dlc.enemypedia} is_collepedia={false} />)
}

#[function_component]
fn PediaPage<T: PartialEq + PediaItem + 'static>(props: &PediaProps<T>) -> Html {
    let data = use_context::<Data>().unwrap();

    let page = use_state(|| 0);
    let region_state = use_state(|| 1u32);
    let region = *region_state as usize - 1;

    let items = props.items.get(region);

    let page_organizer = PageOrganizer::<PAGES_PER_VIEW>::new(ROWS_PER_PAGE, *page, items.len());

    html! {
        <>
            <Field classes={classes!("is-grouped", "is-align-items-end")}>
                <Control classes="is-flex-grow-1">
                    <Field>
                        <label class="label"><Text path="dlc4_map_region" /></label>
                        <Control>
                            <Selector<Dlc4Region> state={region_state.clone()} values={data.game().dlc.map.regions()} />
                        </Control>
                    </Field>
                </Control>
                <Control>
                    <PediaBulkEdit<T> items={items} on={true} />
                </Control>
                <Control>
                    <PediaBulkEdit<T> items={items} on={false} />
                </Control>
            </Field>
            <Tile classes="mb-2">
                {for page_organizer.bounds().map(|(s, e)| html! {
                    <Tile classes="is-align-items-start">
                        <Table classes={classes!("is-fullwidth")}>
                            <thead>
                                <tr>
                                    <th><Text path="dlc4_pedia_desc" /></th>
                                    {if !props.is_collepedia {
                                        html!(<th><Text path="dlc4_pedia_ene_count" /></th>)
                                    } else { html!() }}
                                    <th><Text path="dlc4_pedia_status" /></th>
                                </tr>
                            </thead>

                            <tbody>
                                {for (s..=e).map(|index| {
                                    html!(<PediaRow<T> item={&items[index]} />)
                                })}
                            </tbody>
                        </Table>
                    </Tile>
                })}
            </Tile>

            <PageControls<PAGES_PER_VIEW> organizer={page_organizer} state={page} />
        </>
    }
}

#[function_component]
fn PediaRow<T: PartialEq + PediaItem + 'static>(props: &PediaRowProps<T>) -> Html {
    let data = use_context::<Data>().unwrap();
    let editor = FlagEditor::from(props.item.flag());

    let type_display = match props.item.item() {
        PediaValue::Number { max, slot_id } => {
            let count_editor = EnemypediaEditor {
                index: slot_id as usize,
            };
            html! {
                <td class={classes!("is-flex", "is-align-items-center")}>
                    <NumberInput<EnemypediaEditor> editor={count_editor} max={max} />
                    <span class="ml-2">{"/"}{NON_BREAKING_SPACE}{max}</span>
                </td>
            }
        }
        PediaValue::TriState => html!(),
    };

    html! {
        <tr>
            <td>{props.item.get_name(data.game(), data.lang())}</td>
            {type_display}
            <td>
                <EnumInput<PediaStatusEditor> editor={PediaStatusEditor(editor)} />
            </td>
        </tr>
    }
}

#[function_component]
fn PediaBulkEdit<T: PartialEq + PediaItem + 'static>(props: &PediaBulkProps<T>) -> Html {
    let save = use_context::<SaveContext>().unwrap();

    let lang_path = format!("dlc4_pedia_bulk_{}", if props.on { "on" } else { "off" });

    let PediaBulkProps { on, items, .. } = *props;
    let bulk_set = Callback::from(move |_: MouseEvent| {
        for item in items {
            let editor = FlagEditor::from(item.flag());
            let value = if on {
                PediaStatus::Complete as u32
            } else {
                PediaStatus::Unknown as u32
            };
            let count_editor = match item.item() {
                PediaValue::Number { max, slot_id } => Some((
                    EnemypediaEditor {
                        index: slot_id as usize,
                    },
                    max,
                )),
                _ => None,
            };
            save.edit(move |save| {
                editor.set(save, value);
                if let Some((editor, max)) = count_editor {
                    editor.set(save, if on { max } else { 0 });
                }
            });
        }
    });

    html! {
        <Button onclick={bulk_set}>
            <Text path={lang_path} />
        </Button>
    }
}

impl Editor for PediaStatusEditor {
    type Target = PediaStatus;

    fn get(&self, save: &SaveData) -> Self::Target {
        PediaStatus::from_repr(self.0.get(save) as usize).expect("unknown status")
    }

    fn set(&self, save: &mut SaveData, new: Self::Target) {
        self.0.set(save, new as u32);
    }
}

impl ToHtml for PediaStatus {
    fn to_html(&self) -> Html {
        let id = match self {
            PediaStatus::Unknown => "unknown",
            PediaStatus::InProgress => "progress",
            PediaStatus::Complete => "complete",
        };
        html!(<Text path={format!("dlc4_pedia_status_{id}")} />)
    }
}
