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
                    placeholder: "https://qr.spectrs.app/",
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

#[component]
pub fn LogoUploader(custom_logo: Signal<Option<String>>) -> Element {
    let input_id = "logo-upload-input";
    
    rsx! {
        div {
            class: "space-y-3",
            label { 
                class: "block text-sm font-semibold text-slate-700 dark:text-slate-300 uppercase tracking-wider", 
                "Custom Logo (SVG)"
            }
            div {
                class: "relative",
                input {
                    id: input_id,
                    r#type: "file",
                    accept: ".svg,image/svg+xml",
                    class: "w-full px-4 py-3 rounded-xl border border-slate-200 dark:border-slate-600 bg-white dark:bg-slate-700/50 focus:ring-2 focus:ring-[#4d3695] focus:border-transparent transition-all outline-none shadow-sm text-slate-800 dark:text-white file:mr-4 file:py-2 file:px-4 file:rounded-lg file:border-0 file:text-sm file:font-semibold file:bg-[#4d3695] file:text-white hover:file:bg-[#3d2875] file:cursor-pointer",
                    onchange: move |_| {
                        use wasm_bindgen::JsCast;
                        use web_sys::{FileReader, Event, HtmlInputElement};
                        use wasm_bindgen::closure::Closure;
                        
                        spawn(async move {
                            if let Some(window) = web_sys::window() {
                                if let Some(document) = window.document() {
                                    if let Some(input) = document.get_element_by_id(input_id) {
                                        if let Some(input_element) = input.dyn_ref::<HtmlInputElement>() {
                                            if let Some(file_list) = input_element.files() {
                                                if let Some(file) = file_list.get(0) {
                                                    if let Ok(reader) = FileReader::new() {
                                                        let reader_clone = reader.clone();
                                                        
                                                        let onload = Closure::wrap(Box::new(move |_: Event| {
                                                            if let Ok(result) = reader_clone.result() {
                                                                if let Some(text) = result.as_string() {
                                                                    custom_logo.set(Some(text));
                                                                }
                                                            }
                                                        }) as Box<dyn FnMut(_)>);
                                                        
                                                        reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                                                        let _ = reader.read_as_text(&file);
                                                        onload.forget();
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        });
                    }
                }
                if custom_logo().is_some() {
                    div {
                        class: "mt-2 flex items-center gap-2",
                        span {
                            class: "text-sm text-green-600 dark:text-green-400 font-medium",
                            "✓ Custom logo loaded"
                        }
                        button {
                            class: "px-3 py-1.5 text-sm rounded-lg bg-red-500 hover:bg-red-600 text-white transition-colors",
                            onclick: move |_| custom_logo.set(None),
                            "Clear"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ColorSchemePicker(
    background_color: Signal<String>,
    data_color: Signal<String>,
    finder_color: Signal<String>
) -> Element {
    rsx! {
        div {
            class: "space-y-4",
            label { 
                class: "block text-sm font-semibold text-slate-700 dark:text-slate-300 uppercase tracking-wider", 
                "Color Scheme"
            }
            
            div {
                class: "grid grid-cols-1 gap-3",
                
                // Background Color
                div {
                    class: "flex items-center gap-3",
                    label {
                        class: "text-sm font-medium text-slate-600 dark:text-slate-400 w-32",
                        "Background"
                    }
                    div {
                        class: "flex items-center gap-2 flex-1",
                        input {
                            r#type: "color",
                            class: "w-12 h-10 rounded-lg border border-slate-300 dark:border-slate-600 cursor-pointer",
                            value: "{background_color}",
                            oninput: move |evt| background_color.set(evt.value())
                        }
                        input {
                            r#type: "text",
                            class: "flex-1 px-3 py-2 rounded-lg border border-slate-200 dark:border-slate-600 bg-white dark:bg-slate-700/50 focus:ring-2 focus:ring-[#4d3695] focus:border-transparent transition-all outline-none text-sm text-slate-800 dark:text-white",
                            value: "{background_color}",
                            placeholder: "#FFFFFF",
                            oninput: move |evt| background_color.set(evt.value())
                        }
                    }
                }
                
                // Data/QR Modules Color
                div {
                    class: "flex items-center gap-3",
                    label {
                        class: "text-sm font-medium text-slate-600 dark:text-slate-400 w-32",
                        "QR Modules"
                    }
                    div {
                        class: "flex items-center gap-2 flex-1",
                        input {
                            r#type: "color",
                            class: "w-12 h-10 rounded-lg border border-slate-300 dark:border-slate-600 cursor-pointer",
                            value: "{data_color}",
                            oninput: move |evt| data_color.set(evt.value())
                        }
                        input {
                            r#type: "text",
                            class: "flex-1 px-3 py-2 rounded-lg border border-slate-200 dark:border-slate-600 bg-white dark:bg-slate-700/50 focus:ring-2 focus:ring-[#4d3695] focus:border-transparent transition-all outline-none text-sm text-slate-800 dark:text-white",
                            value: "{data_color}",
                            placeholder: "#4d3695",
                            oninput: move |evt| data_color.set(evt.value())
                        }
                    }
                }
                
                // Finder Pattern Color
                div {
                    class: "flex items-center gap-3",
                    label {
                        class: "text-sm font-medium text-slate-600 dark:text-slate-400 w-32",
                        "Finder Corners"
                    }
                    div {
                        class: "flex items-center gap-2 flex-1",
                        input {
                            r#type: "color",
                            class: "w-12 h-10 rounded-lg border border-slate-300 dark:border-slate-600 cursor-pointer",
                            value: "{finder_color}",
                            oninput: move |evt| finder_color.set(evt.value())
                        }
                        input {
                            r#type: "text",
                            class: "flex-1 px-3 py-2 rounded-lg border border-slate-200 dark:border-slate-600 bg-white dark:bg-slate-700/50 focus:ring-2 focus:ring-[#4d3695] focus:border-transparent transition-all outline-none text-sm text-slate-800 dark:text-white",
                            value: "{finder_color}",
                            placeholder: "#4d3695",
                            oninput: move |evt| finder_color.set(evt.value())
                        }
                    }
                }
            }
        }
    }
}
