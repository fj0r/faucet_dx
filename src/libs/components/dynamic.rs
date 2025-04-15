use super::container::*;
use super::widgets::*;
use dioxus::prelude::*;

#[component]
pub fn Dynamic(kind: String, children: Element) -> Element {
    let c = {
        match kind.as_str() {
            "Container" => rsx!( Container { {children} } ),
            "List" => rsx!( List { {children} } ),
            "Input" => rsx! ( Input { {children} } ),
            "Text" => rsx! ( Text { {children} } ),
            "Card" => rsx! ( Card { {children } } ),
            "Button" => rsx! ( Button { {children} } ),
            _ => {
                let t = format!("{} unimplemented!", &kind);
                rsx! { div { "{t}" } }
            }
        }
    };
    rsx! {
        div {
            class: "dynamic",
            {c}
        }
    }
}
