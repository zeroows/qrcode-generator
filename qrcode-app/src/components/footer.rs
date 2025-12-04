use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "py-6 text-center text-slate-400 dark:text-slate-500 text-sm",
            "© 2025 Abdulrhman Alkhodiry. All rights reserved."
        }
    }
}
