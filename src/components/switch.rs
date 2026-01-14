use dioxus::prelude::*;

#[component]
pub fn Switch(title: String) -> Element {
    let mut is_enabled = use_signal(|| false);

    rsx! {
        div {
            class: "switch-container",
            style: "display: flex; align-items: center; margin-bottom: 16px;",
            
            label {
                class: "switch",
                style: "position: relative; display: inline-block; width: 50px; height: 24px; margin-right: 12px;",
                
                input {
                    r#type: "checkbox",
                    checked: is_enabled(),
                    onchange: move |_| is_enabled.set(!is_enabled()),
                    style: "opacity: 0; width: 0; height: 0;",
                }
                
                span {
                    class: "slider",
                    style: "position: absolute; cursor: pointer; top: 0; left: 0; right: 0; bottom: 0; background-color: #ccc; transition: .4s; border-radius: 24px;",
                    style: if is_enabled() { "background-color: #2196F3;" } else { "background-color: #ccc;" },
                    
                    span {
                        style: "position: absolute; content: ''; height: 18px; width: 18px; left: 3px; bottom: 3px; background-color: white; transition: .4s; border-radius: 50%;",
                        style: if is_enabled() { "transform: translateX(26px);" } else { "" },
                    }
                }
            }
            
            span {
                "{
title}"
            }
        }
    }
}