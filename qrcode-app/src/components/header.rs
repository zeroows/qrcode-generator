use dioxus::prelude::*;
use super::icons::IconQr;

#[component]
pub fn Header() -> Element {
    rsx! {
        div {
            class: "space-y-3",
            div {
                class: "flex items-center gap-3",
                div { class: "p-2 bg-purple-100 dark:bg-purple-900/30 rounded-xl text-[#4d3695]", IconQr {} }
                h1 { class: "text-2xl md:text-3xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-[#4d3695] to-[#6B4B8A]", "QR Code Generator" }
            }
            p { class: "text-slate-500 dark:text-slate-400 text-base leading-relaxed", "Create professional, branded QR codes instantly. Choose a style, enter your link, and download." }
        }
    }
}
