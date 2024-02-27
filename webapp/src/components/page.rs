use ybc::{Pagination, PaginationEllipsis};
use yew::prelude::*;

#[derive(Clone, Copy, PartialEq)]
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
    let organizer = props.organizer;
    let current_page_for_display = organizer.page_display_id();
    let last_page = organizer.last_page;

    let change_page_callback = |new_page: Option<usize>| {
        let page = props.state.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(new_page) = new_page {
                page.set(new_page);
            }
        })
    };

    let pagination_classes = |new_page: usize| {
        if new_page == organizer.current_page {
            classes!("pagination-link", "is-current")
        } else {
            classes!("pagination-link")
        }
    };

    html! {
        <Pagination
            previous={html!(<a class="pagination-previous" disabled={organizer.previous_page().is_none()}
                            onclick={change_page_callback(organizer.previous_page())}>{"Previous"}</a>)}
            next={html!(<a class="pagination-next" disabled={organizer.next_page().is_none()}
                            onclick={change_page_callback(organizer.next_page())}>{"Next"}</a>)}
        >
            <a onclick={change_page_callback(Some(0))} class={pagination_classes(0)}>
                {"1"}
            </a>
            {(current_page_for_display > 2).then(|| html! {
                <PaginationEllipsis />
            })}
            {for (current_page_for_display-1..=current_page_for_display+1)
                .filter(|&page| page <= last_page && page > 1)
                .map(|page| html! {
                    <a onclick={change_page_callback(Some(page - 1))} class={pagination_classes(page - 1)}>
                        {page.to_string()}
                    </a>
                })
            }
            {(current_page_for_display <= last_page.saturating_sub(1)).then(|| html! {
                <PaginationEllipsis />
            })}
            {(last_page != 0).then(|| html! {
                <a onclick={change_page_callback(Some(last_page))} class={pagination_classes(last_page)}>
                    {(last_page + 1).to_string()}
                </a>
            })}

        </Pagination>
    }
}

impl<const PER_VIEW: usize> PageOrganizer<PER_VIEW> {
    pub fn new(per_page: usize, mut page: usize, total: usize) -> Self {
        let mut open_pages = [(0, 0); PER_VIEW];
        let last_page = (total - 1) / (per_page * PER_VIEW);
        if page > last_page {
            // If state dependencies change and the new state doesn't support
            // the page, reset to first.
            page = 0;
        }
        let mut start = per_page * page * PER_VIEW;
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

    pub fn bounds(&self) -> impl Iterator<Item = (usize, usize)> {
        self.current_bounds.into_iter().filter(|(s, e)| s <= e)
    }

    pub fn page_display_id(&self) -> usize {
        1 + self.current_page
    }

    pub fn next_page(&self) -> Option<usize> {
        self.current_page
            .checked_add(1)
            .filter(|&next| next <= self.last_page)
    }

    pub fn previous_page(&self) -> Option<usize> {
        self.current_page.checked_sub(1)
    }
}
