/* 
 * QR Code generator library (Rust)
 * 
 * Copyright (c) Project Nayuki. (MIT License)
 * Copyright (c) Abdulrhman Alkhodiry (aalkhodiry@gmail.com)
 * 
 * Basic rendering utilities for QR codes
 */

//! Basic rendering utilities for QR codes.
//! 
//! This module provides simple rendering functions for QR codes,
//! including SVG and text output.

use crate::qrcode::QrCode;

/// Renders a QR code as a simple SVG string.
/// 
/// # Arguments
/// 
/// * `qr` - The QR code to render
/// * `border` - The size of the quiet zone (white border) in modules
/// * `module_size` - The size of each module in pixels (default: 1)
/// 
/// # Example
/// 
/// ```rust
/// use qrcode_lib::{QrCode, QrCodeEcc};
/// use qrcode_lib::render::to_svg_string;
/// 
/// let qr = QrCode::encode_text("Hello", QrCodeEcc::Low).unwrap();
/// let svg = to_svg_string(&qr, 4, 10);
/// ```
pub fn to_svg_string(qr: &QrCode, border: i32, module_size: i32) -> String {
    let size = qr.size();
    let full_size = (size + border * 2) * module_size;
    
    let mut svg = String::new();
    svg.push_str(&format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" version="1.1" viewBox="0 0 {w} {w}" stroke="none">"##,
        w = full_size
    ));
    svg.push_str("\n");
    
    // Background
    svg.push_str(&format!(
        r##"<rect width="{w}" height="{w}" fill="#FFFFFF"/>"##,
        w = full_size
    ));
    svg.push_str("\n");
    
    // Modules
    svg.push_str(r##"<path d=""##);
    for y in 0..size {
        for x in 0..size {
            if qr.get_module(x, y) {
                let px = (x + border) * module_size;
                let py = (y + border) * module_size;
                svg.push_str(&format!("M{},{}h{}v{}h-{}z", px, py, module_size, module_size, module_size));
            }
        }
    }
    svg.push_str(r##"" fill="#000000"/>"##);
    svg.push_str("\n</svg>");
    
    svg
}

/// Renders a QR code as ASCII art for terminal display.
/// 
/// Uses Unicode block characters for a compact representation.
/// 
/// # Example
/// 
/// ```rust
/// use qrcode_lib::{QrCode, QrCodeEcc};
/// use qrcode_lib::render::to_ascii_art;
/// 
/// let qr = QrCode::encode_text("Test", QrCodeEcc::Low).unwrap();
/// let art = to_ascii_art(&qr, 2);
/// println!("{}", art);
/// ```
pub fn to_ascii_art(qr: &QrCode, border: i32) -> String {
    let size = qr.size();
    let mut result = String::new();
    
    // Top border
    for _ in 0..(size + border * 2) {
        result.push_str("██");
    }
    result.push('\n');
    
    // QR code with side borders
    for y in -border..size + border {
        // Left border
        for _ in 0..border {
            result.push_str("██");
        }
        
        // Content
        for x in 0..size {
            let module = if y >= 0 && y < size {
                qr.get_module(x, y)
            } else {
                false
            };
            result.push_str(if module { "  " } else { "██" });
        }
        
        // Right border
        for _ in 0..border {
            result.push_str("██");
        }
        result.push('\n');
    }
    
    result
}

/// Returns a string of space-separated '0' and '1' characters representing the modules.
/// Useful for debugging or testing.
/// 
/// # Example
/// 
/// ```rust
/// use qrcode_lib::{QrCode, QrCodeEcc};
/// use qrcode_lib::render::to_debug_string;
/// 
/// let qr = QrCode::encode_text("Hi", QrCodeEcc::Low).unwrap();
/// let debug = to_debug_string(&qr);
/// ```
pub fn to_debug_string(qr: &QrCode) -> String {
    let size = qr.size();
    let mut result = String::new();
    
    for y in 0..size {
        for x in 0..size {
            result.push(if qr.get_module(x, y) { '1' } else { '0' });
            if x < size - 1 {
                result.push(' ');
            }
        }
        if y < size - 1 {
            result.push('\n');
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::QrCodeEcc;
    
    #[test]
    fn test_svg_rendering() {
        let qr = QrCode::encode_text("Test", QrCodeEcc::Low).unwrap();
        let svg = to_svg_string(&qr, 4, 10);
        assert!(svg.starts_with("<svg"));
        assert!(svg.ends_with("</svg>"));
    }
    
    #[test]
    fn test_ascii_art() {
        let qr = QrCode::encode_text("Hi", QrCodeEcc::Low).unwrap();
        let art = to_ascii_art(&qr, 2);
        assert!(!art.is_empty());
        assert!(art.contains("██"));
    }
    
    #[test]
    fn test_debug_string() {
        let qr = QrCode::encode_text("A", QrCodeEcc::Low).unwrap();
        let debug = to_debug_string(&qr);
        assert!(debug.contains('0'));
        assert!(debug.contains('1'));
    }
}

