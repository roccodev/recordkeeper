use ybc::{Pagination, PaginationEllipsis};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct PageOrganizer<const PER_VIEW: usize> {
    pub current_bounds: [(usize, usize); PER_VIEW],
    pub current_page: usize,
    pub last_page: usize,
}

#[derive(PartialEq, Properties)]
pub struct PageProps<const PER_VIEW: usize> {
    pub organizer: PageOrganizer<PER_VIEW>,
    pub state: UseStateHandle<usize>,
}

#[function_component]
pub fn PageControls<const PER_VIEW: usize>(props: &PageProps<PER_VIEW>) -> Html {
    let current_page_for_display = props.organizer.page_display_id();
    let last_page = props.organizer.last_page;

    let change_page_callback = |new_page: usize| {
        let page = props.state.clone();
        Callback::from(move |_: MouseEvent| {
            if new_page > last_page {
                return;
            }
            if let Some(new_page) = new_page.checked_sub(1) {
                page.set(new_page * PER_VIEW);
            }
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
            {if current_page_for_display <= last_page.saturating_sub(2) {
                html!(<PaginationEllipsis />)
            } else {
                html!()
            }}
            <a onclick={change_page_callback(last_page)} class={pagination_classes(last_page)}>
                {last_page.to_string()}
            </a>
        </Pagination>
    }
}

impl<const PER_VIEW: usize> PageOrganizer<PER_VIEW> {
    pub fn new(per_page: usize, page: usize, total: usize) -> Self {
        let mut open_pages = [(0, 0); PER_VIEW];
        let mut start = per_page * page;
        let last_page = (total + per_page * PER_VIEW - 1) / (per_page * PER_VIEW);
        for (st, end) in &mut open_pages {
            *st = start;
            start = (start + per_page).min(total);
            *end = start - 1;
        }
        Self {
            current_bounds: open_pages,
            current_page: page,
            last_page,
        }
    }

    pub fn page_display_id(&self) -> usize {
        1 + self.current_page / PER_VIEW
    }
}
