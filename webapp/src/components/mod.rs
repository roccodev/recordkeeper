use ybc::Container;
use yew::prelude::*;

pub mod edit;
pub mod nav;
pub mod page;
pub mod select;
pub mod sidebar;
pub mod upload;

pub mod character;
pub mod dlc;
pub mod enemy;
pub mod field;
pub mod item;
pub mod meta;
pub mod ouroboros;
pub mod quest;

#[derive(Properties, PartialEq)]
pub struct FluidContainerProps {
    pub children: Children,
}

#[function_component]
pub fn FluidContainer(props: &FluidContainerProps) -> Html {
    html! {
        <Container fluid={true}>
            {props.children.clone()}
        </Container>
    }
}
