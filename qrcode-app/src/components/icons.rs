use dioxus::prelude::*;

#[component]
pub fn IconLink() -> Element {
    rsx! {
        svg {
            class: "w-5 h-5",
            fill: "none",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            stroke_width: "2",
            path {
                d: "M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1",
                stroke_linecap: "round",
                stroke_linejoin: "round"
            }
        }
    }
}

#[component]
pub fn IconQr() -> Element {
    rsx! {
        svg {
            class: "w-6 h-6",
            fill: "none",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            stroke_width: "2",
            path {
                d: "M12 4v1m6 11h2m-6 0h-2v4h2v-4zM6 6h6v6H6V6zm12 0h-6v6h6V6zm-6 12H6v6h6v-6z",
                stroke_linecap: "round",
                stroke_linejoin: "round"
            }
            rect { x: "4", y: "4", width: "6", height: "6", rx: "1" }
            rect { x: "14", y: "4", width: "6", height: "6", rx: "1" }
            rect { x: "4", y: "14", width: "6", height: "6", rx: "1" }
        }
    }
}

#[component]
pub fn IconDownload() -> Element {
    rsx! {
        svg {
            class: "w-5 h-5",
            fill: "none",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            stroke_width: "2",
            path {
                d: "M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4",
                stroke_linecap: "round",
                stroke_linejoin: "round"
            }
        }
    }
}

#[component]
pub fn IconCopy() -> Element {
    rsx! {
        svg {
            class: "w-5 h-5",
            fill: "none",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            stroke_width: "2",
            path {
                d: "M8 7v8a2 2 0 002 2h6M8 7V5a2 2 0 012-2h4.586a1 1 0 01.707.293l4.414 4.414a1 1 0 01.293.707V15a2 2 0 01-2 2h-2M8 7H6a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2v-2",
                stroke_linecap: "round",
                stroke_linejoin: "round"
            }
        }
    }
}

#[component]
pub fn IconCheck() -> Element {
    rsx! {
        svg {
            class: "w-5 h-5",
            fill: "none",
            view_box: "0 0 24 24",
            stroke: "currentColor",
            stroke_width: "2",
            path {
                d: "M5 13l4 4L19 7",
                stroke_linecap: "round",
                stroke_linejoin: "round"
            }
        }
    }
}

