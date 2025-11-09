use dioxus::prelude::*;

#[component]
pub fn ControlGroup(title: String, children: Element) -> Element {
    rsx! {
        div {
            class: "control-group",
            h3 {
                class: "control-group-title",
                "{title}"
            }
            div {
                class: "control-group-contents",
                {children}
            }
        }
    }
}
