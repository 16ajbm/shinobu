use dioxus::prelude::*;

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
            label {
                class: "slider-label",
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
