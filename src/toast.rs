use std::time::Duration;

use leptos::*;

#[derive(Clone)]
pub enum ToastType {
    Success,
    Error,
}

#[derive(Clone)]
pub struct ToastMessage {
    pub message: String,
    pub toast_type: ToastType,
    pub visible: bool,
}

/// .
#[component]
pub fn Toast() -> impl IntoView {
    let (toast, set_toast) = create_signal::<ToastMessage>(ToastMessage {
        message: String::new(),
        toast_type: ToastType::Success,
        visible: false,
    });
    provide_context::<WriteSignal<ToastMessage>>(set_toast);

    let base_toast_classes = "fixed bottom-10 left-1/2 transform -translate-x-1/2 text-white px-4 py-2 rounded shadow-lg transition-opacity duration-300";

    let toast_classes = move || -> String {
        let t = toast.get();
        let extra_classes = if t.visible == true {
            let bg_color = match t.toast_type {
                ToastType::Success => "bg-green-600",
                ToastType::Error => "bg-red-600",
            };

            format!("{bg_color} opacity-1")
        } else {
            "opacity-0".to_string()
        };

        format!("{} {}", base_toast_classes, extra_classes)
    };

    create_effect(move |_| {
        let t = toast.get();
        if t.visible {
            set_timeout(
                move || {
                    set_toast.update(|msg| {
                        msg.visible = false;
                    });
                },
                Duration::new(4, 0),
            )
        }
    });

    view! {
        <div id="toast" class={toast_classes}>
            {move || toast.get().message}
        </div>
    }
}
