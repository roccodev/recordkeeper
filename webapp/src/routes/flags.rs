use recordkeeper::flags::FlagType;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use ybc::{Button, Buttons, Container, Field, Pagination, PaginationEllipsis, Table, Tile};
use yew::prelude::*;

use crate::{
    components::edit::{FlagEditor, NumberInput},
    lang::{Lang, Text},
};

const FLAG_TYPES: &[FlagType] = &[
    FlagType::Bit,
    FlagType::TwoBits,
    FlagType::FourBits,
    FlagType::Byte,
    FlagType::Short,
    FlagType::Int,
];

const PAGES_PER_VIEW: usize = 3;
const ROWS_PER_PAGE: usize = 10;

#[derive(Properties, PartialEq)]
struct TableProps {
    pub flag_type: FlagType,
    pub start: usize,
    pub end: usize,
}

#[derive(Properties, PartialEq)]
struct PageChangeProps {
    pub flag_type: FlagType,
    pub page_state: UseStateHandle<usize>,
}

#[function_component]
pub fn FlagList() -> Html {
    let flag_type = use_state(|| FlagType::Bit);
    let page = use_state(|| 0);

    let mut open_pages = [(0, 0); PAGES_PER_VIEW];
    let mut start = ROWS_PER_PAGE * *page;
    let last_page = (flag_type.num_flags() + ROWS_PER_PAGE * PAGES_PER_VIEW - 1)
        / (ROWS_PER_PAGE * PAGES_PER_VIEW);
    for (st, end) in &mut open_pages {
        *st = start;
        start = (start + ROWS_PER_PAGE).min(flag_type.num_flags());
        *end = start - 1;
    }

    let current_page_for_display = 1 + *page / PAGES_PER_VIEW;

    let change_page_callback = |new_page: usize| {
        let page = page.clone();
        Callback::from(move |_: MouseEvent| {
            page.set((new_page - 1) * PAGES_PER_VIEW);
        })
    };

    let pagination_classes = |new_page: usize| {
        if new_page == current_page_for_display {
            classes!("pagination-link", "is-current")
        } else {
            classes!("pagination-link")
        }
    };

    html! {
        <Container>
            <Tile classes={classes!("is-align-items-end")}>
                <Tile>
                    <Field>
                        <label class="label"><Text path="flag_bits" /></label>
                        <Buttons classes={classes!("has-addons")}>
                            {for FLAG_TYPES.into_iter().map(|flag| {
                                let page = page.clone();
                                let flag_type = flag_type.clone();
                                let classes = if flag == &*flag_type {
                                    classes!("is-info", "is-selected")
                                } else {
                                    classes!("")
                                };
                                let on_click = Callback::from(move |_: MouseEvent| {
                                    page.set(0);
                                    flag_type.set(*flag);
                                });

                                html! {
                                    <Button onclick={on_click} classes={classes}>{flag.num_bits().to_string()}</Button>
                                }
                            })}
                        </Buttons>
                    </Field>
                </Tile>
                <Tile classes={classes!("is-4")}>
                    <GoToFlag flag_type={*flag_type} page_state={page.clone()} />
                </Tile>
            </Tile>

            <Tile>
                {for open_pages.into_iter().map(|(start, end)| html! {
                    <Tile>
                        <TablePage flag_type={*flag_type} start={start} end={end} />
                    </Tile>
                })}
            </Tile>

            <Pagination
                previous={html!(<a class="pagination-previous" disabled={current_page_for_display == 1}
                                onclick={change_page_callback(current_page_for_display.saturating_sub(1))}>{"Previous"}</a>)}
                next={html!(<a class="pagination-next" disabled={current_page_for_display == last_page}
                                onclick={change_page_callback(current_page_for_display.saturating_add(1))}>{"Next"}</a>)}
            >
                <a onclick={change_page_callback(1)} class={pagination_classes(1)}>
                    {"1"}
                </a>
                {if current_page_for_display > 2 {
                    html!(<PaginationEllipsis />)
                } else {
                    html!()
                }}
                {for (current_page_for_display-1..=current_page_for_display+1)
                    .filter(|&page| page > 1 && page < last_page)
                    .map(|page| html!(<a onclick={change_page_callback(page)} class={pagination_classes(page)}>{page.to_string()}</a>))
                }
                {if current_page_for_display <= last_page - 2 {
                    html!(<PaginationEllipsis />)
                } else {
                    html!()
                }}
                <a onclick={change_page_callback(last_page)} class={pagination_classes(last_page)}>
                    {last_page.to_string()}
                </a>
            </Pagination>
        </Container>
    }
}

#[function_component]
fn TablePage(props: &TableProps) -> Html {
    let flag_type = props.flag_type;

    html! {
        <Table classes={classes!("is-fullwidth")}>
            <thead>
                <tr>
                    <th><Text path="flag_index" /></th>
                    <th><Text path="flag_value" /></th>
                </tr>
            </thead>

            <tbody>
                {for (props.start..=props.end).map(|index| {
                    let editor = FlagEditor {
                        flag_type: flag_type,
                        flag_index: index
                    };

                    html! {
                        <tr>
                            <th>{index.to_string()}</th>
                            <td><NumberInput<FlagEditor> editor={editor} /></td>
                        </tr>
                    }
                })}
            </tbody>
        </Table>
    }
}

#[function_component]
fn GoToFlag(props: &PageChangeProps) -> Html {
    let page_state = props.page_state.clone();
    let flag_type = props.flag_type;
    let lang = use_context::<Lang>().unwrap();

    let callback = Callback::from(move |e: InputEvent| {
        let target: Option<EventTarget> = e.target();
        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        if let Some(input) = input {
            match input.value().parse::<usize>() {
                Ok(flag_index) => {
                    if flag_index < flag_type.num_flags() {
                        let next_page = flag_index / (PAGES_PER_VIEW * ROWS_PER_PAGE);
                        page_state.set(next_page * PAGES_PER_VIEW);
                    }
                }
                Err(_) => {
                    e.prevent_default();
                }
            }
        }
    });

    html! {
        <input
            class="input" type="number"
            placeholder={lang.translate("flag_jump_page").to_string()}
            oninput={callback}
        />
    }
}
