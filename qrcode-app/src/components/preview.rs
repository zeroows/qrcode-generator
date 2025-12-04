use dioxus::prelude::*;
use super::icons::{IconDownload, IconCopy, IconCheck};

#[component]
pub fn PreviewPanel(
    svg_content: String, 
    on_download: EventHandler<()>, 
    on_copy: EventHandler<()>, 
    is_copying: bool
) -> Element {
    rsx! {
        div {
            class: "lg:col-span-7 p-8 md:p-12 flex flex-col items-center justify-center bg-[#FAF5FF] dark:bg-[#1a1625] border-l border-slate-200 dark:border-slate-700 relative",
            
            // Preview Area
            div {
                class: "relative group perspective-1000",
                div { class: "absolute -inset-4 bg-gradient-to-r from-purple-500 to-indigo-500 rounded-[2rem] blur-xl opacity-20 group-hover:opacity-30 transition duration-1000" }
                div {
                    class: "relative w-80 h-80 sm:w-96 sm:h-96 rounded-2xl overflow-hidden shadow-2xl transition-transform duration-500 bg-white p-6 flex items-center justify-center border border-white/50",
                    dangerous_inner_html: "{svg_content}"
                }
            }

            // Action Buttons
            div {
                class: "mt-10 flex flex-col sm:flex-row gap-4 w-full max-w-md",
                
                // Download Button
                button {
                    class: "flex-1 flex items-center justify-center gap-2 px-6 py-3.5 rounded-xl bg-slate-900 dark:bg-white text-white dark:text-slate-900 font-medium hover:opacity-90 active:scale-95 transition-all shadow-lg shadow-slate-900/20 dark:shadow-white/10 focus:outline-none focus:ring-2 focus:ring-slate-900 dark:focus:ring-white focus:ring-offset-2 dark:focus:ring-offset-slate-900",
                    onclick: move |_| on_download.call(()),
                    IconDownload {},
                    "Download SVG"
                }

                // Copy Button
                button {
                    class: format_args!(
                        "flex-1 flex items-center justify-center gap-2 px-6 py-3.5 rounded-xl border font-medium transition-all active:scale-95 focus:outline-none focus:ring-2 focus:ring-purple-500 focus:ring-offset-2 dark:focus:ring-offset-slate-900 {}",
                        if is_copying {
                            "bg-green-50 border-green-200 text-green-700 dark:bg-green-900/20 dark:border-green-800 dark:text-green-400"
                        } else {
                            "bg-white dark:bg-slate-800 border-slate-200 dark:border-slate-600 text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-slate-700 hover:border-slate-300"
                        }
                    ),
                    onclick: move |_| on_copy.call(()),
                    if is_copying {
                        IconCheck {}
                        span { "Copied!" }
                    } else {
                        IconCopy {}
                        span { "Copy Code" }
                    }
                }
            }
        }
    }
}
