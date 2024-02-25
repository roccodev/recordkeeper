use game_data::dlc::map::Dlc4Region;
use game_data::dlc::pedia::{CollepediaStatus, Dlc4Collepedia, Enemypedia, PediaItem, PediaValue};
use game_data::dlc::Regional;
use recordkeeper::SaveData;
use ybc::{Control, Field, Table, Tile};
use yew::prelude::*;

use crate::components::edit::{EnumInput, FlagEditor, NumberInput};
use crate::components::page::{PageControls, PageOrganizer};
use crate::ToHtml;
use crate::{
    components::{edit::Editor, select::Selector},
    data::Data,
    lang::Text,
};

#[derive(Properties, PartialEq)]
pub struct PediaProps<T: PartialEq + 'static> {
    items: &'static Regional<T>,
}

#[derive(Properties, PartialEq)]
struct PediaRowProps<T: PartialEq + 'static> {
    item: &'static T,
}

#[derive(PartialEq, Copy, Clone)]
pub struct CollepediaStatusEditor(FlagEditor);

const PAGES_PER_VIEW: usize = 2;
const ROWS_PER_PAGE: usize = 12;

#[function_component]
pub fn CollepediaPage() -> Html {
    let data = use_context::<Data>().unwrap();
    html!(<PediaPage<Dlc4Collepedia> items={&data.game().dlc.collepedia} />)
}

#[function_component]
pub fn EnemypediaPage() -> Html {
    let data = use_context::<Data>().unwrap();
    html!(<PediaPage<Enemypedia> items={&data.game().dlc.enemypedia} />)
}

#[function_component]
fn PediaPage<T: PartialEq + PediaItem + 'static>(props: &PediaProps<T>) -> Html {
    let data = use_context::<Data>().unwrap();

    let page = use_state(|| 0);
    let region_state = use_state(|| 1);
    let region = *region_state - 1;

    let items = props.items.get(region);

    let page_organizer = PageOrganizer::<PAGES_PER_VIEW>::new(ROWS_PER_PAGE, *page, items.len());

    html! {
        <>
            <Field>
                <label class="label"><Text path="dlc4_map_region" /></label>
                <Control>
                    <Selector<Dlc4Region> state={region_state.clone()} values={data.game().dlc.map.regions()} />
                </Control>
            </Field>
            <Tile classes="mb-2">
                {for page_organizer.bounds().map(|(s, e)| html! {
                    <Tile classes="is-align-items-start">
                        <Table classes={classes!("is-fullwidth")}>
                            <thead>
                                <tr>
                                    <th><Text path="dlc4_pedia_desc" /></th>
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
        PediaValue::Number { max } => html! {
            <td class={classes!("is-flex", "is-align-items-center")}>
                <NumberInput<FlagEditor> editor={editor} max={max as u32} />
                <span class="ml-2">{"/"}{max}</span>
            </td>
        },
        PediaValue::TriState => html! {
            <td>
                <EnumInput<CollepediaStatusEditor> editor={CollepediaStatusEditor(editor)} />
            </td>
        },
    };

    html! {
        <tr>
            <td>{props.item.get_name(data.game(), data.lang())}</td>
            {type_display}
        </tr>
    }
}

impl Editor for CollepediaStatusEditor {
    type Target = CollepediaStatus;

    fn get(&self, save: &SaveData) -> Self::Target {
        CollepediaStatus::from_repr(self.0.get(save) as usize).expect("unknown status")
    }

    fn set(&self, save: &mut SaveData, new: Self::Target) {
        self.0.set(save, new as u32);
    }
}

impl ToHtml for CollepediaStatus {
    fn to_html(&self) -> Html {
        let id = match self {
            CollepediaStatus::Unknown => "unknown",
            CollepediaStatus::InProgress => "progress",
            CollepediaStatus::Complete => "complete",
        };
        html!(<Text path={format!("dlc4_pedia_status_{id}")} />)
    }
}
