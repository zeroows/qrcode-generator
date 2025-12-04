// Example: Generate SVG output
//
// This example shows how to generate a QR code and export it as SVG.

use qrcode_lib::{QrCode, QrCodeEcc};
use qrcode_lib::render::to_svg_string;
use std::fs::File;
use std::io::Write;

fn main() {
    // Create QR code
    let url = "https://github.com/zeroows/qrcode-generator";
    let qr = QrCode::encode_text(url, QrCodeEcc::Medium)
        .expect("Failed to create QR code");
    
    // Generate SVG with 4-module border and 5px module size
    let svg = to_svg_string(&qr, 4, 5);
    
    // Save to file
    let filename = "qrcode.svg";
    let mut file = File::create(filename)
        .expect("Failed to create file");
    file.write_all(svg.as_bytes())
        .expect("Failed to write to file");
    
    println!("QR code saved to {}", filename);
    println!("Size: {}x{} modules", qr.size(), qr.size());
}

