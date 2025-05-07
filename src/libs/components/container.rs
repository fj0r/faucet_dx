use super::super::data::Layout;
use super::super::store::Store;
use super::types::Ele;
use super::utils::merge_css_class;
use super::Dynamic;
use dioxus::prelude::*;

#[component]
pub fn Container(layout: Layout, children: Element, el: Option<Ele>) -> Element {
    let mut css = vec!["container", "f"];
    let css = merge_css_class(&mut css, &layout);

    rsx! {
        div {
            class: css.join(" "),
            onmounted: move |element| {
                if let Some(mut e) = el {
                    e.set(Some(element.data()))
                }
            } ,
            {children}
        }
    }
}

#[component]
pub fn Tab(layout: Layout, children: Element) -> Element {
    let mut css = vec!["card", "f", "v", "box", "border", "shadow"];
    let css = merge_css_class(&mut css, &layout);
    rsx! {
        div {
            class: css.join(" "),
            {children}
        }
    }
}

#[component]
pub fn Menu(layout: Layout, children: Element) -> Element {
    let mut css = vec!["card", "f", "v", "box", "border", "shadow"];
    let css = merge_css_class(&mut css, &layout);
    rsx! {
        div {
            class: css.join(" "),
            {children}
        }
    }
}

#[component]
pub fn Card(layout: Layout, children: Element, el: Option<Ele>) -> Element {
    let mut css = vec!["card", "f", "v", "box", "border", "shadow"];
    let css = merge_css_class(&mut css, &layout);
    rsx! {
        div {
            onmounted: move |element| {
                if let Some(mut e) = el {
                    e.set(Some(element.data()))
                }
            } ,
            class: css.join(" "),
            {children}
        }
    }
}
