/* 
 * QR Code generator library (Rust)
 * 
 * Copyright (c) Project Nayuki. (MIT License)
 * Copyright (c) Abdulrhman Alkhodiry (aalkhodiry@gmail.com)
 * 
 * Fancy QR Code rendering with customizable styles
 */

//! Fancy QR code rendering with custom styles, colors, and overlays.

use crate::qrcode::QrCode;
use crate::types::{QrCodeEcc, DataTooLong};

/// Controls the shape of the small data dots.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ModuleShape {
    /// Standard square modules
    Square,
    /// Circular modules
    Circle,
    /// A square with rounded corners. Radius is 0.0 to 0.5 (relative to module size).
    RoundedSquare(f32), 
}

/// Controls the shape of the 3 large corner patterns.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FinderShape {
    /// Standard square finder patterns
    Square,
    /// Rounded corners. Radius is relative to the 7-module width.
    Rounded(f32), 
}

/// Configuration options for fancy QR code rendering.
pub struct FancyOptions {
    /// Background color (hex format, e.g., "#FFFFFF")
    pub color_background: String,
    /// Data module color (hex format, e.g., "#000000")
    pub color_data: String,
    /// Finder pattern color (hex format, e.g., "#000000")
    pub color_finder: String,
    
    /// Shape of the data modules
    pub shape_module: ModuleShape,
    /// Shape of the finder patterns
    pub shape_finder: FinderShape,
    
    /// URL or Base64 data for a center image overlay
    pub center_image_url: Option<String>,
    /// Text to display in the center (alternative to image, e.g., "SCAN ME")
    pub center_text: Option<String>,
    /// How large the center safe zone is (0.0 to 0.3).
    /// Note: Error correction High can typically recover up to 30% damage.
    pub overlay_scale: f32,
}

impl Default for FancyOptions {
    fn default() -> Self {
        FancyOptions {
            color_background: "#FFFFFF".to_string(),
            color_data: "#000000".to_string(),
            color_finder: "#000000".to_string(),
            shape_module: ModuleShape::Square,
            shape_finder: FinderShape::Square,
            center_image_url: None,
            center_text: None,
            overlay_scale: 0.2,
        }
    }
}

/// A fancy QR code with customizable rendering options.
pub struct FancyQr {
    code: QrCode,
    quiet_zone: usize,
}

impl FancyQr {
    /// Creates a new fancy QR code from text with High Error Correction (recommended for overlays).
    /// 
    /// High error correction level allows up to ~30% of the code to be covered/damaged,
    /// which is important when using center overlays.
    pub fn from_text(text: &str) -> Result<Self, DataTooLong> {
        let code = QrCode::encode_text(text, QrCodeEcc::High)?;
        Ok(FancyQr { 
            code,
            quiet_zone: 4, // Standard white border width
        })
    }
    
    /// Creates a new fancy QR code from binary data with High Error Correction.
    pub fn from_binary(data: &[u8]) -> Result<Self, DataTooLong> {
        let code = QrCode::encode_binary(data, QrCodeEcc::High)?;
        Ok(FancyQr { 
            code,
            quiet_zone: 4,
        })
    }
    
    /// Creates a fancy QR code from text with a specific error correction level.
    pub fn from_text_with_ecc(text: &str, ecl: QrCodeEcc) -> Result<Self, DataTooLong> {
        let code = QrCode::encode_text(text, ecl)?;
        Ok(FancyQr { 
            code,
            quiet_zone: 4,
        })
    }
    
    /// Creates a fancy QR code from an existing QrCode.
    pub fn from_qrcode(code: QrCode) -> Self {
        FancyQr { 
            code,
            quiet_zone: 4,
        }
    }
    
    /// Sets the quiet zone (white border) size in modules.
    pub fn with_quiet_zone(mut self, size: usize) -> Self {
        self.quiet_zone = size;
        self
    }
    
    /// Returns a reference to the underlying QR code.
    pub fn qrcode(&self) -> &QrCode {
        &self.code
    }

    /// Renders the QR code to a standalone SVG string with custom styling.
    pub fn render_svg(&self, options: &FancyOptions) -> String {
        let matrix_width = self.code.size() as usize;
        let full_width = matrix_width + (self.quiet_zone * 2);
        
        // SVG Header
        let mut svg = String::new();
        svg.push_str(&format!(
            r#"<svg viewBox="0 0 {w} {w}" xmlns="http://www.w3.org/2000/svg" shape-rendering="geometricPrecision">"#,
            w = full_width
        ));

        // 1. Background Layer
        svg.push_str(&format!(
            r#"<rect x="0" y="0" width="{w}" height="{w}" fill="{c}" />"#,
            w = full_width, c = options.color_background
        ));

        // Calculate Safe Zone (Center)
        let center_idx = matrix_width as f32 / 2.0;
        let safe_size = matrix_width as f32 * options.overlay_scale;
        let safe_min = center_idx - (safe_size / 2.0);
        let safe_max = center_idx + (safe_size / 2.0);

        let is_safe_zone = |c: usize, r: usize| -> bool {
            if options.center_image_url.is_none() && options.center_text.is_none() {
                return false;
            }
            let fx = c as f32;
            let fy = r as f32;
            fx >= safe_min && fx <= safe_max && fy >= safe_min && fy <= safe_max
        };

        // 2. Render Data Modules
        for r in 0..matrix_width {
            for c in 0..matrix_width {
                // Skip light modules
                if !self.code.get_module(c as i32, r as i32) { 
                    continue; 
                }
                
                // Identify Finders (7x7 corners)
                let is_finder = Self::is_finder_module(c, r, matrix_width);

                // Skip rendering raw finders (we draw custom ones later)
                if is_finder { 
                    continue; 
                }
                
                // Skip rendering center safety zone
                if is_safe_zone(c, r) { 
                    continue; 
                }

                // Draw Module
                let x = c + self.quiet_zone;
                let y = r + self.quiet_zone;
                let fill = &options.color_data;

                match options.shape_module {
                    ModuleShape::Square => {
                        svg.push_str(&format!(r#"<rect x="{x}" y="{y}" width="1" height="1" fill="{fill}" />"#));
                    },
                    ModuleShape::Circle => {
                        svg.push_str(&format!(
                            r#"<circle cx="{cx}" cy="{cy}" r="0.45" fill="{fill}" />"#, 
                            cx=x as f32 + 0.5, 
                            cy=y as f32 + 0.5
                        ));
                    },
                    ModuleShape::RoundedSquare(rad) => {
                        svg.push_str(&format!(
                            r#"<rect x="{x}" y="{y}" width="1" height="1" rx="{rad}" fill="{fill}" />"#
                        ));
                    }
                }
            }
        }

        // 3. Render Custom Finder Patterns
        Self::render_finder_patterns(&mut svg, matrix_width, self.quiet_zone, options);

        // 4. Render Center Overlay
        Self::render_center_overlay(&mut svg, center_idx, safe_size, self.quiet_zone, options);

        svg.push_str("</svg>");
        svg
    }
    
    /// Renders the QR code to SVG with default options.
    pub fn render_svg_default(&self) -> String {
        self.render_svg(&FancyOptions::default())
    }
    
    // Helper: Check if a module is part of a finder pattern
    fn is_finder_module(c: usize, r: usize, width: usize) -> bool {
        // Top-Left (0,0), Top-Right (W-7, 0), Bottom-Left (0, W-7)
        (r < 7 && c < 7) || 
        (r < 7 && c >= width.saturating_sub(7)) || 
        (r >= width.saturating_sub(7) && c < 7)
    }
    
    // Helper: Render the three finder patterns
    fn render_finder_patterns(
        svg: &mut String, 
        matrix_width: usize, 
        quiet_zone: usize, 
        options: &FancyOptions
    ) {
        let finder_positions = vec![
            (0, 0), 
            (matrix_width.saturating_sub(7), 0), 
            (0, matrix_width.saturating_sub(7))
        ];

        for (fc, fr) in finder_positions {
            let x = fc + quiet_zone;
            let y = fr + quiet_zone;
            
            // Calculate roundness
            let r_outer = match options.shape_finder {
                FinderShape::Square => 0.0,
                FinderShape::Rounded(r) => r,
            };
            
            // Draw concentric boxes
            // Outer Box (7x7)
            svg.push_str(&format!(
                r#"<rect x="{x}" y="{y}" width="7" height="7" rx="{r}" fill="{color}" />"#, 
                r=r_outer, 
                color=options.color_finder
            ));
            
            // Inner Cutout (5x5) - matches background
            let r_mid = if r_outer > 0.0 { r_outer * 0.7 } else { 0.0 };
            svg.push_str(&format!(
                r#"<rect x="{x}" y="{y}" width="5" height="5" rx="{r}" fill="{color}" />"#, 
                x=x+1, 
                y=y+1, 
                r=r_mid, 
                color=options.color_background
            ));

            // Center Dot (3x3)
            let r_inner = if r_outer > 0.0 { r_outer * 0.4 } else { 0.0 };
            svg.push_str(&format!(
                r#"<rect x="{x}" y="{y}" width="3" height="3" rx="{r}" fill="{color}" />"#, 
                x=x+2, 
                y=y+2, 
                r=r_inner, 
                color=options.color_finder
            ));
        }
    }
    
    // Helper: Render center overlay (image or text)
    fn render_center_overlay(
        svg: &mut String,
        center_idx: f32,
        safe_size: f32,
        quiet_zone: usize,
        options: &FancyOptions
    ) {
        let center_px = center_idx + quiet_zone as f32;
        let size_px = safe_size;
        let start_px = center_px - (size_px / 2.0);

        if let Some(img_href) = &options.center_image_url {
            svg.push_str(&format!(
                r#"<image x="{x}" y="{y}" width="{w}" height="{h}" href="{href}" preserveAspectRatio="xMidYMid slice" />"#,
                x=start_px, 
                y=start_px, 
                w=size_px, 
                h=size_px, 
                href=img_href
            ));
        } else if let Some(text) = &options.center_text {
            // Draw a "Label Badge" (white box + text)
            svg.push_str(&format!(
                r#"<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="1" fill="{bg}" stroke="{fg}" stroke-width="0.2" />"#,
                x=start_px - 0.5, 
                y=start_px + (size_px * 0.25),
                w=size_px + 1.0, 
                h=size_px * 0.5,
                bg=options.color_background, 
                fg=options.color_data
            ));
            
            svg.push_str(&format!(
                r#"<text x="{x}" y="{y}" font-family="sans-serif" font-weight="bold" font-size="{sz}" text-anchor="middle" fill="{fg}">{txt}</text>"#,
                x=center_px, 
                y=center_px + (size_px * 0.15),
                sz=size_px * 0.25, 
                fg=options.color_data, 
                txt=text
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fancy_qr_creation() {
        let qr = FancyQr::from_text("Hello, World!").unwrap();
        assert!(qr.qrcode().size() > 0);
    }
    
    #[test]
    fn test_svg_rendering() {
        let qr = FancyQr::from_text("Test").unwrap();
        let svg = qr.render_svg_default();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }
    
    #[test]
    fn test_custom_options() {
        let qr = FancyQr::from_text("Custom").unwrap();
        let mut options = FancyOptions::default();
        options.color_data = "#FF0000".to_string();
        options.shape_module = ModuleShape::Circle;
        let svg = qr.render_svg(&options);
        assert!(svg.contains("#FF0000"));
        assert!(svg.contains("<circle"));
    }
}

