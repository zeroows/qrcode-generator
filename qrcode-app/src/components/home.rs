use dioxus::prelude::*;
use qrcode_lib::fancy::FancyQr;
use gloo_timers::future::sleep;
use std::time::Duration;
use crate::types::{QrStyle, get_custom_style_options};
use super::{Header, UrlInput, StyleSelector, PreviewPanel, Footer, LogoUploader, ColorSchemePicker};

const LOGO_SVG: &str = include_str!("../../assets/logo-icon.svg");

#[component]
pub fn Home() -> Element {
    let content = use_signal(|| "https://example.com/".to_string());
    let style = use_signal(|| QrStyle::GradientMinimal);
    let mut svg_output = use_signal(|| String::new());
    let mut copying = use_signal(|| false);
    
    // Custom logo and colors
    let custom_logo = use_signal(|| Option::<String>::None);
    let background_color = use_signal(|| "#FFFFFF".to_string());
    let data_color = use_signal(|| "#4d3695".to_string());
    let finder_color = use_signal(|| "#4d3695".to_string());

    // Generate QR code when inputs change
    use_effect(move || {
        let url = content();
        let current_style = style();
        let logo = custom_logo();
        let bg = background_color();
        let data = data_color();
        let finder = finder_color();

        if url.is_empty() {
            return;
        }

        let qr = match FancyQr::from_text(&url) {
            Ok(q) => q,
            Err(_) => return,
        };

        // Use custom logo if provided, otherwise use default
        let logo_svg = logo.as_deref().unwrap_or(LOGO_SVG);
        let logo_base64 = if !logo_svg.is_empty() {
            base64_encode_svg(logo_svg)
        } else {
            String::new()
        };

        // Use custom colors if provided
        let options = get_custom_style_options(current_style, &logo_base64, &bg, &data, &finder);
        let svg = qr.render_svg(&options);
        svg_output.set(svg);
    });

    let handle_download = move |_| {
        let filename = format!("qr_code_{}.svg", match style() {
            QrStyle::Standard => "standard",
            QrStyle::MinimalLogo => "minimal_logo",
            QrStyle::GradientLogo => "gradient_logo",
            QrStyle::Premium => "premium",
            QrStyle::BrandedFinders => "branded_finders",
            QrStyle::MinimalFinders => "minimal_finders",
            QrStyle::GradientFinders => "gradient_finders",
            QrStyle::GradientMinimal => "gradient_minimal",
        });
        download_svg(&filename, &svg_output());
    };

    let handle_copy = move |_| {
        let svg = svg_output();
        spawn(async move {
            if copy_to_clipboard(svg).await {
                copying.set(true);
                sleep(Duration::from_secs(2)).await;
                copying.set(false);
            }
        });
    };

    rsx! {
        div {
            class: "min-h-screen flex flex-col bg-gradient-to-br from-slate-50 to-slate-100 dark:from-slate-900 dark:to-slate-800 text-slate-800 dark:text-white font-sans selection:bg-purple-200 selection:text-purple-900",
            
            // Main Content
            main {
                class: "flex-grow flex flex-col items-center justify-center p-4 md:p-8",
                
                div {
                    class: "w-full max-w-6xl grid grid-cols-1 lg:grid-cols-12 gap-8 bg-white dark:bg-slate-800 rounded-3xl shadow-2xl shadow-purple-900/10 overflow-hidden border border-slate-200 dark:border-slate-700 ring-1 ring-slate-900/5",
                    
                    // Left Column: Controls
                    div {
                        class: "lg:col-span-5 p-8 md:p-10 flex flex-col justify-center bg-slate-50/50 dark:bg-slate-800/50 relative overflow-hidden",
                        
                        // Decorative background blob
                        div { class: "absolute top-0 left-0 w-full h-full bg-gradient-to-b from-purple-50/50 to-transparent dark:from-purple-900/10 pointer-events-none" }

                        div {
                            class: "relative space-y-8",
                            Header {}
                            UrlInput { value: content }
                            StyleSelector { selected: style }
                            LogoUploader { custom_logo: custom_logo }
                            ColorSchemePicker { 
                                background_color: background_color,
                                data_color: data_color,
                                finder_color: finder_color
                            }
                        }
                    }

                    // Right Column: Preview
                    PreviewPanel {
                        svg_content: svg_output(),
                        on_download: handle_download,
                        on_copy: handle_copy,
                        is_copying: copying()
                    }
                }
            }

            // Footer
            Footer {}
        }
    }
}

fn download_svg(filename: &str, content: &str) {
    use wasm_bindgen::JsCast;
    use web_sys::{HtmlElement, Url, Blob, BlobPropertyBag};

    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            let props = BlobPropertyBag::new();
            props.set_type("image/svg+xml;charset=utf-8");
            
            // Create blob
            let parts = js_sys::Array::new();
            parts.push(&wasm_bindgen::JsValue::from_str(content));
            
            if let Ok(blob) = Blob::new_with_str_sequence_and_options(&parts, &props) {
                if let Ok(url) = Url::create_object_url_with_blob(&blob) {
                    if let Ok(element) = document.create_element("a") {
                        let a = element.unchecked_into::<HtmlElement>();
                        let _ = a.set_attribute("href", &url);
                        let _ = a.set_attribute("download", filename);
                        a.click();
                        let _ = Url::revoke_object_url(&url);
                    }
                }
            }
        }
    }
}

async fn copy_to_clipboard(content: String) -> bool {
    if let Some(window) = web_sys::window() {
        let navigator = window.navigator();
        let clipboard = navigator.clipboard();
        let promise = clipboard.write_text(&content);
        let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
        return true;
    }
    false
}

// Simple base64 encoding for SVG data URI
fn base64_encode_svg(svg: &str) -> String {
    let mut encoded = String::new();
    let bytes = svg.as_bytes();
    
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
    for chunk in bytes.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = byte;
        }
        
        let b1 = (buf[0] >> 2) as usize;
        let b2 = (((buf[0] & 0x03) << 4) | (buf[1] >> 4)) as usize;
        let b3 = (((buf[1] & 0x0f) << 2) | (buf[2] >> 6)) as usize;
        let b4 = (buf[2] & 0x3f) as usize;
        
        encoded.push(ALPHABET[b1] as char);
        encoded.push(ALPHABET[b2] as char);
        encoded.push(if chunk.len() > 1 { ALPHABET[b3] as char } else { '=' });
        encoded.push(if chunk.len() > 2 { ALPHABET[b4] as char } else { '=' });
    }
    
    format!("data:image/svg+xml;base64,{}", encoded)
}

