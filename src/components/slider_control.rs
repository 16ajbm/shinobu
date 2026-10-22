use dioxus::prelude::*;

#[component]
pub fn SliderControl(
    label: String,
    value: f32,
    on_change: EventHandler<f32>,
    min: f32,
    max: f32,
    #[props(default = 0.01)] step: f32,
    #[props(default = 2)] precision: usize,
) -> Element {
    rsx! {
        div { class: "slider-control",
            label {
                class: "slider-label",
                style: "display: block; margin-bottom: 8px; font-weight: 500; font-size: inherit;",
                "{label}: {value:.precision$}"
            }
            input {
                r#type: "range",
                class: "slider-input",
                style: "width: 100%; height: 6px; cursor: pointer;",
                min: "{min}",
                max: "{max}",
                step: "{step}",
                value: "{value}",
                oninput: move |event| {
                    if let Ok(v) = event.value().parse::<f32>() {
                        on_change.call(v);
                    }
                },
            }
        }
    }
}
