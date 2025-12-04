// Example: Basic QR code generation
//
// This example demonstrates the simplest way to generate a QR code
// and display it in the terminal.

use qrcode_lib::{QrCode, QrCodeEcc};
use qrcode_lib::render::to_ascii_art;

fn main() {
    // Create a simple QR code
    let text = "Hello, World!";
    let qr = QrCode::encode_text(text, QrCodeEcc::Low)
        .expect("Failed to create QR code");
    
    println!("QR Code for: {}", text);
    println!("Size: {}x{}", qr.size(), qr.size());
    println!("Error Correction Level: {:?}", qr.error_correction_level());
    println!();
    
    // Display as ASCII art
    let ascii = to_ascii_art(&qr, 2);
    println!("{}", ascii);
}

