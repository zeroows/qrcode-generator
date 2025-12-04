use dioxus::prelude::*;
use crate::types::QrStyle;
use super::icons::{IconLink, IconCheck};

#[component]
pub fn UrlInput(value: Signal<String>) -> Element {
    rsx! {
        div {
            class: "space-y-3",
            label { class: "block text-sm font-semibold text-slate-700 dark:text-slate-300 uppercase tracking-wider", "Content URL" }
            div {
                class: "relative group",
                div { class: "absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none text-slate-400 group-focus-within:text-[#4d3695] transition-colors", IconLink {} }
                input {
                    class: "w-full pl-11 pr-4 py-3.5 rounded-xl border border-slate-200 dark:border-slate-600 bg-white dark:bg-slate-700/50 focus:ring-2 focus:ring-[#4d3695] focus:border-transparent transition-all outline-none shadow-sm text-slate-800 dark:text-white placeholder:text-slate-400",
                    value: "{value}",
                    placeholder: "https://example.com/",
                    oninput: move |evt| value.set(evt.value())
                }
            }
        }
    }
}

#[component]
pub fn StyleSelector(selected: Signal<QrStyle>) -> Element {
    let styles = [
        QrStyle::Standard,
        QrStyle::MinimalLogo,
        QrStyle::GradientLogo,
        QrStyle::Premium,
        QrStyle::BrandedFinders,
        QrStyle::MinimalFinders,
        QrStyle::GradientFinders,
        QrStyle::GradientMinimal,
    ];

    rsx! {
        div {
            class: "space-y-3",
            label { class: "block text-sm font-semibold text-slate-700 dark:text-slate-300 uppercase tracking-wider", "Style Preset" }
            div {
                class: "grid grid-cols-1 sm:grid-cols-2 gap-3",
                for s in styles {
                    button {
                        class: format_args!(
                            "group relative px-4 py-3 rounded-xl text-sm font-medium transition-all text-left flex items-center justify-between {}",
                            if selected() == s { 
                                "bg-[#4d3695] text-white shadow-md shadow-purple-500/25 ring-1 ring-[#4d3695] ring-offset-1 dark:ring-offset-slate-800 scale-[1.02]" 
                            } else { 
                                "bg-white dark:bg-slate-700 hover:bg-slate-100 dark:hover:bg-slate-600 text-slate-600 dark:text-slate-300 border border-slate-200 dark:border-slate-600 hover:border-purple-200 dark:hover:border-slate-500" 
                            }
                        ),
                        onclick: move |_| selected.set(s),
                        span { "{s.name()}" }
                        if selected() == s {
                            span { class: "ml-2 bg-white/20 rounded-full p-0.5", IconCheck {} }
                        }
                    }
                }
            }
        }
    }
}

