use dioxus::prelude::*;

#[component]
pub fn ControlGroup(title: String, children: Element) -> Element {
    rsx! {
        div { class: "control-group",
            h3 {
                class: "control-group-title",
                style: "font-size: clamp(18px, 1.8vw, 22px); margin-bottom: 12px; color: #333;",
                "{title}"
            }
            div { class: "control-group-contents", {children} }
        }
    }
}
