use super::super::data::Layout;
use super::super::store::Store;
use super::types::Ele;
use super::utils::merge_css_class;
use super::Dynamic;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use dioxus_logger::tracing;

#[component]
pub fn List(layout: Layout, children: Element) -> Element {
    let last_element: Ele = use_signal(|| None);

    let mut css = vec!["list", "f"];
    let css = merge_css_class(&mut css, &layout);

    let item0 = &layout.clone().item.context("item")?[0];
    let data_bind = layout.data.clone().context("data")?;

    let s = use_context::<Store>();
    let c = s.list.read();
    let c = c
        .get(&data_bind.event)
        .cloned()
        .unwrap_or_else(|| Vec::new());
    let r = c.iter().enumerate().map(|(idx, child)| {
        let x = rsx! {
            Dynamic {
                el: None,
                layout: child.clone()
            }
        };
        let key = child.id.clone().unwrap_or(idx.to_string());
        let layout = item0.clone();
        if c.len() - 1 == idx {
            // last element
            rsx! {
                Dynamic {
                    el: Some(last_element),
                    key: "{key}",
                    layout: layout,
                    {x}
                }
            }
        } else {
            rsx! {
                Dynamic {
                    el: None,
                    key: "{key}",
                    layout: layout,
                    {x}
                }
            }
        }
    });

    let sl = s.list.clone();
    use_effect(move || {
        let _ = sl.read();
        if let Some(e) = last_element() {
            dioxus_logger::tracing::info!("{e:?}");
            let _ = e.scroll_to(ScrollBehavior::Smooth);
        };
    });

    rsx! {
        div {
            class: css.join(" "),
            {r}
        }
    }
}
