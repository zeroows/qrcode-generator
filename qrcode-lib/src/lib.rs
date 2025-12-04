/* 
 * QR Code generator library (Rust)
 * 
 * Copyright (c) Project Nayuki. (MIT License)
 * Copyright (c) Abdulrhman Alkhodiry (aalkhodiry@gmail.com)
 * https://www.nayuki.io/page/qr-code-generator-library
 * 
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 * - The above copyright notice and this permission notice shall be included in
 *   all copies or substantial portions of the Software.
 * - The Software is provided "as is", without warranty of any kind, express or
 *   implied, including but not limited to the warranties of merchantability,
 *   fitness for a particular purpose and noninfringement. In no event shall the
 *   authors or copyright holders be liable for any claim, damages or other
 *   liability, whether in an action of contract, tort or otherwise, arising from,
 *   out of or in connection with the Software or the use or other dealings in the
 *   Software.
 */

//! # QR Code Generator Library
//! 
//! A powerful QR Code generator library with fancy rendering capabilities.
//! 
//! This library provides both standard QR code generation and stylish, customizable
//! fancy QR codes with support for custom colors, shapes, and overlays.
//! 
//! ## Features
//! 
//! ### Core QR Code Features:
//! 
//! - Supports all 40 versions (sizes) and all 4 error correction levels
//! - Output format: Raw modules/pixels of the QR symbol
//! - Detects finder-like penalty patterns more accurately than other implementations
//! - Encodes numeric and special-alphanumeric text in less space than general text
//! - Open-source code under the permissive MIT License
//! 
//! ### Fancy QR Code Features:
//! 
//! - Custom colors for background, data, and finder patterns
//! - Multiple module shapes: Square, Circle, Rounded Square
//! - Custom finder pattern shapes
//! - Center image overlay support
//! - Center text label support
//! - SVG output with high-quality rendering
//! 
//! ## Examples
//! 
//! ### Basic QR Code
//! 
//! ```rust
//! use qrcode_lib::{QrCode, QrCodeEcc};
//! 
//! let qr = QrCode::encode_text("Hello, world!", QrCodeEcc::Medium).unwrap();
//! println!("Size: {}x{}", qr.size(), qr.size());
//! 
//! // Access individual modules
//! for y in 0..qr.size() {
//!     for x in 0..qr.size() {
//!         let module = qr.get_module(x, y);
//!         print!("{}", if module { "██" } else { "  " });
//!     }
//!     println!();
//! }
//! ```
//! 
//! ### Fancy QR Code with Custom Style
//! 
//! ```rust
//! use qrcode_lib::fancy::{FancyQr, FancyOptions, ModuleShape, FinderShape};
//! 
//! let qr = FancyQr::from_text("https://example.com").unwrap();
//! 
//! let mut options = FancyOptions::default();
//! options.color_data = "#1E40AF".to_string(); // Blue
//! options.color_finder = "#3B82F6".to_string(); // Light blue
//! options.shape_module = ModuleShape::Circle;
//! options.shape_finder = FinderShape::Rounded(1.5);
//! options.center_text = Some("SCAN ME".to_string());
//! 
//! let svg = qr.render_svg(&options);
//! // Save or use the SVG string
//! ```
//! 
//! ### Advanced Segment Usage
//! 
//! ```rust
//! use qrcode_lib::{QrCode, QrCodeEcc, QrSegment, Version, Mask};
//! 
//! let text = "3141592653589793238462643383";
//! let segs = QrSegment::make_segments(text);
//! let qr = QrCode::encode_segments_advanced(
//!     &segs, 
//!     QrCodeEcc::High,
//!     Version::new(5), 
//!     Version::new(5), 
//!     Some(Mask::new(2)), 
//!     false
//! ).unwrap();
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

// Module declarations
mod types;
mod segment;
mod qrcode;
pub mod fancy;
pub mod render;

// Re-export public API
pub use types::{QrCodeEcc, Version, Mask, DataTooLong};
pub use segment::{QrSegment, QrSegmentMode, BitBuffer};
pub use qrcode::QrCode;
