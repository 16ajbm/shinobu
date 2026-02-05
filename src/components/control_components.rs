use dioxus::prelude::*;

#[component]
pub fn ButtonControl(label: String, on_click: EventHandler<()>) -> Element {
    rsx! {
        button {
            class: "button-control",
            style: "",
            onclick: move |_| on_click(()),
            "{label}"
        }
    }
}

#[component]
pub fn SliderControl(
    label: String,
    value: Signal<f32>,
    min: f32,
    max: f32,
    #[props(default = 0.01)] step: f32,
    #[props(default = 2)] precision: usize,
) -> Element {
    rsx! {
        div {
            class: "slider-control",
            style: "",
            label {
                class: "slider-label",
                style: "",
                "{label}: {value():.precision$}"
            }
            input {
                r#type: "range",
                class: "slider-input",
                min: "{min}",
                max: "{max}",
                step: "{step}",
                value: "{value()}",
                oninput: move |evt| {
                    if let Ok(v) = evt.value().parse::<f32>() {
                        value.set(v);
                    }
                }
            }
        }
    }
}
