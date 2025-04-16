use dioxus::prelude::*;
use dioxus_logger::tracing;
use super::super::data::Layout;

#[component]
pub fn Container(layout:Layout, children: Element) -> Element {
    rsx!{
        div {
            class: "Container f v",
            {children}
        }
    }
}


#[component]
pub fn List(layout: Layout, children: Element) -> Element {
    rsx!{
        div {
            class: "List f v",
            {children}
        }
    }
}

#[component]
pub fn Card(layout: Layout, children: Element) -> Element {
    rsx! {
        div {
            class: "Card f v box border shadow",
            {children}
        }
    }
}
