use dioxus::prelude::*;
use std::time::Duration;

#[derive(Clone, PartialEq)]
pub enum ToastType {
    Success,
    Error,
    Info,
}

#[derive(Clone)]
pub struct Toast {
    pub message: String,
    pub toast_type: ToastType,
}

struct ToastStyles {
    bg_color: &'static str,
    border_color: &'static str,
}

impl ToastType {
    fn styles(&self) -> ToastStyles {
        match self {
            ToastType::Success => ToastStyles {
                bg_color: "#4caf50",
                border_color: "#45a049",
            },
            ToastType::Error => ToastStyles {
                bg_color: "#f44336",
                border_color: "#da190b",
            },
            ToastType::Info => ToastStyles {
                bg_color: "#2196f3",
                border_color: "#0b7dda",
            },
        }
    }
}

const TOAST_CONTAINER: &str =
    "position: fixed; top: 20px; right: 20px; z-index: 1000; animation: slideIn 0.3s ease-out;";
const TOAST_BOX: &str = "color: white; padding: 16px 24px; border-radius: 8px; box-shadow: 0 4px 12px rgba(0,0,0,0.15); min-width: 250px; max-width: 400px;";
const TOAST_CONTENT: &str = "display: flex; justify-content: space-between; align-items: center;";
const TOAST_MESSAGE: &str = "margin: 0; font-size: 14px; font-weight: 500;";
const CLOSE_BUTTON: &str = "background: transparent; border: none; color: white; font-size: 20px; cursor: pointer; margin-left: 16px; line-height: 1;";
const SLIDE_IN_ANIMATION: &str = "
    @keyframes slideIn {
        from { transform: translateX(400px); opacity: 0; }
        to { transform: translateX(0); opacity: 1; }
    }
";

fn use_auto_dismiss(toast: Signal<Option<Toast>>, duration: Duration) {
    use_effect(move || {
        let Some(_) = toast() else { return };

        let mut toast_setter = toast.clone();

        spawn(async move {
            async_std::task::sleep(duration).await;
            toast_setter.set(None);
        });
    });
}

#[component]
pub fn ToastContainer(toast: Signal<Option<Toast>>) -> Element {
    use_auto_dismiss(toast, Duration::from_secs(3));

    let Some(current_toast) = toast() else {
        return rsx! { div {} };
    };

    rsx! {
        div {
            style: "{TOAST_CONTAINER}",
            ToastMessage {
                message: current_toast.message,
                toast_type: current_toast.toast_type,
                on_close: move |_| toast.set(None)
            }
        }
        style { {SLIDE_IN_ANIMATION} }
    }
}

#[component]
fn ToastMessage(message: String, toast_type: ToastType, on_close: EventHandler) -> Element {
    let styles = toast_type.styles();
    let toast_style = format!(
        "{TOAST_BOX} background: {}; border-left: 4px solid {};",
        styles.bg_color, styles.border_color
    );

    rsx! {
        div {
            style: "{toast_style}",
            div {
                style: "{TOAST_CONTENT}",
                p {
                    style: "{TOAST_MESSAGE}",
                    "{message}"
                }
                button {
                    style: "{CLOSE_BUTTON}",
                    onclick: move |_| on_close.call(()),
                    "⨉"
                }
            }
        }
    }
}
