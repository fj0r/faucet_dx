use dioxus::prelude::*;
mod libs;
use libs::store::use_store;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {

    let url = "ws://localhost:3000/channel";
    let r = use_store(url).expect("connecting failed");
    let x = r.layout;
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "svg", href: HEADER_SVG }
        div {
            "{x:?}"
        }
    }
}

