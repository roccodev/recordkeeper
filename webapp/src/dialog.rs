//! Dialog/modal singleton context, accessible by all components.

use std::rc::Rc;

use ybc::{Block, Button, Icon, Modal};
use yew::prelude::*;
use yew_feather::AlertOctagon;

use crate::lang::Text;

#[derive(PartialEq, Clone)]
pub struct Dialog {
    layout: DialogLayout,
}

#[derive(PartialEq, Clone)]
pub enum DialogLayout {
    Ok {
        title: Option<AttrValue>,
        message: Html,
        severity: Severity,
    },
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Severity {
    Success,
    Info,
    Warning,
    Error,
}

#[derive(Properties, PartialEq)]
pub struct DialogRendererProps {
    pub children: Children,
}

#[derive(PartialEq, Clone, Default)]
pub struct DialogRc(Rc<Option<Dialog>>);

pub type DialogQueue = UseReducerHandle<DialogRc>;

#[function_component]
pub fn DialogRenderer(props: &DialogRendererProps) -> Html {
    let dialog = use_reducer(DialogRc::default);

    let for_callback = dialog.clone();
    let close_modal_callback = Callback::from(move |_: MouseEvent| {
        for_callback.dispatch(None);
    });

    let modal = match &*dialog.0 {
        Some(dialog) => html! {
            <Modal id="card" classes={classes!("is-active")}>
                <ybc::Box>
                    {dialog.layout.to_fragment(close_modal_callback)}
                </ybc::Box>
            </Modal>
        },
        _ => html!(),
    };

    html! {
        <>
            {modal}
            <ContextProvider<DialogQueue> context={dialog}>
                {props.children.clone()}
            </ContextProvider<DialogQueue>>
        </>
    }
}

impl DialogLayout {
    fn to_fragment(&self, close_callback: Callback<MouseEvent>) -> Html {
        match self {
            Self::Ok {
                title,
                message,
                severity,
            } => html! {
                <>
                    <div class="icon-text">
                        <Icon classes={classes!(severity.color_class())}>
                            <AlertOctagon />
                        </Icon>
                        <span><b>{title.as_ref().map(|t| t.as_str()).unwrap_or_else(|| severity.title())}</b></span>
                    </div>

                    <Block>
                        {message.clone()}
                    </Block>

                    <footer>
                        <Button onclick={close_callback} classes={classes!("is-primary")}>
                            <Text path="ok" />
                        </Button>
                    </footer>
                </>
            },
        }
    }
}

impl Severity {
    fn title(self) -> &'static str {
        match self {
            Severity::Success => "Success",
            Severity::Info => "Info",
            Severity::Warning => "Warning",
            Severity::Error => "Error",
        }
    }

    fn color_class(self) -> &'static str {
        match self {
            Severity::Success => "has-text-success",
            Severity::Info => "has-text-info",
            Severity::Warning => "has-text-warning",
            Severity::Error => "has-text-danger",
        }
    }
}

impl From<DialogLayout> for Dialog {
    fn from(value: DialogLayout) -> Self {
        Self { layout: value }
    }
}

impl Reducible for DialogRc {
    type Action = Option<Dialog>;

    fn reduce(self: Rc<Self>, next_dialog: Self::Action) -> Rc<Self> {
        Rc::new(DialogRc(Rc::new(next_dialog)))
    }
}
