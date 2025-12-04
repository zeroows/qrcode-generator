# QR Code Generator

A powerful and elegant QR Code generator library for Rust with fancy rendering capabilities by Abdulrhman Alkhodiry.

## 🌐 Live Demo

Try the web app: **[https://zeroows.github.io/qrcode-generator/](https://zeroows.github.io/qrcode-generator/)**

## Features

### Core QR Code Features

- ✅ Supports all 40 versions (sizes) and all 4 error correction levels
- ✅ Raw module/pixel access for custom rendering
- ✅ Accurate finder-like penalty pattern detection
- ✅ Optimized encoding for numeric and alphanumeric text
- ✅ Open-source under MIT License

### Fancy QR Code Features

- 🎨 **Custom Colors**: Background, data modules, and finder patterns
- 🔷 **Multiple Module Shapes**: Square, Circle, Rounded Square
- 🎯 **Custom Finder Patterns**: Square or rounded corners
- 🖼️ **Center Overlays**: Support for images or text
- 📄 **SVG Output**: High-quality vector graphics

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
qrcode-lib = "0.1.0"
```

## Quick Start

### Basic QR Code

```rust
use qrcode_lib::{QrCode, QrCodeEcc};

fn main() {
    let qr = QrCode::encode_text("Hello, World!", QrCodeEcc::Medium).unwrap();
    
    // Access modules
    for y in 0..qr.size() {
        for x in 0..qr.size() {
            let is_dark = qr.get_module(x, y);
            print!("{}", if is_dark { "██" } else { "  " });
        }
        println!();
    }
}
```

### Fancy QR Code

```rust
use qrcode_lib::fancy::{FancyQr, FancyOptions, ModuleShape, FinderShape};

fn main() {
    let qr = FancyQr::from_text("https://example.com").unwrap();
    
    let mut options = FancyOptions::default();
    options.color_data = "#6B4B8A".to_string();
    options.color_finder = "#8B5CF6".to_string();
    options.shape_module = ModuleShape::Circle;
    options.shape_finder = FinderShape::Rounded(1.5);
    options.center_text = Some("AA".to_string());
    
    let svg = qr.render_svg(&options);
    // Save or use the SVG
}
```

## Examples

The `examples/` directory contains several ready-to-run examples:

### Run Examples

```bash
# Basic QR code in terminal
cargo run --example basic

# Generate SVG file
cargo run --example svg_output

# Generate fancy styled QR code
cargo run --example fancy_qr

# Branded QR codes with logo overlay ✨
cargo run --example branded

# Advanced usage with segments
cargo run --example advanced

# Generate multiple styled QR codes
cargo run --example fancy_styled
```

## Module Structure

The library is organized into clean, well-documented modules:

- **`types`**: Core types (Version, Mask, QrCodeEcc, DataTooLong)
- **`segment`**: QR segment encoding and modes
- **`qrcode`**: Core QR code generation logic
- **`fancy`**: Fancy rendering with custom styles
- **`render`**: Basic rendering utilities (SVG, ASCII art)

## API Documentation

### Core Types

- **`QrCode`**: Main QR code struct with encoding methods
- **`QrCodeEcc`**: Error correction levels (Low, Medium, Quartile, High)
- **`QrSegment`**: Data segment with encoding modes
- **`Version`**: QR code version (1-40)
- **`Mask`**: Mask pattern (0-7)

### Fancy Rendering

- **`FancyQr`**: Wrapper for styled QR codes
- **`FancyOptions`**: Styling configuration
- **`ModuleShape`**: Shape of data dots (Square, Circle, RoundedSquare)
- **`FinderShape`**: Shape of corner patterns (Square, Rounded)

## Error Handling

The library uses Result types for error handling:

```rust
use qrcode_lib::{QrCode, QrCodeEcc, DataTooLong};

match QrCode::encode_text("Very long text...", QrCodeEcc::High) {
    Ok(qr) => println!("QR code created: {}x{}", qr.size(), qr.size()),
    Err(DataTooLong::SegmentTooLong) => println!("Segment is too long"),
    Err(DataTooLong::DataOverCapacity(len, max)) => {
        println!("Data {} bits exceeds capacity {} bits", len, max)
    }
}
```

## Advanced Usage

### Custom Segments

```rust
use qrcode_lib::{QrCode, QrCodeEcc, QrSegment};

// More efficient for numbers
let numeric = QrSegment::make_numeric("123456789");
let qr = QrCode::encode_segments(&[numeric], QrCodeEcc::Low).unwrap();

// Alphanumeric for uppercase text
let alphanum = QrSegment::make_alphanumeric("HELLO WORLD");
let qr = QrCode::encode_segments(&[alphanum], QrCodeEcc::Medium).unwrap();
```

### Fine-Grained Control

```rust
use qrcode_lib::{QrCode, QrCodeEcc, QrSegment, Version, Mask};

let segments = QrSegment::make_segments("Advanced QR");
let qr = QrCode::encode_segments_advanced(
    &segments,
    QrCodeEcc::High,
    Version::new(5),      // Min version
    Version::new(10),     // Max version
    Some(Mask::new(3)),   // Specific mask
    false                 // Don't boost ECC
).unwrap();
```

## Rendering Options

### ASCII Art

```rust
use qrcode_lib::render::to_ascii_art;

let qr = QrCode::encode_text("Test", QrCodeEcc::Low).unwrap();
let art = to_ascii_art(&qr, 2);
println!("{}", art);
```

### Simple SVG

```rust
use qrcode_lib::render::to_svg_string;

let qr = QrCode::encode_text("Test", QrCodeEcc::Low).unwrap();
let svg = to_svg_string(&qr, 4, 10); // border=4, module_size=10
```

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Copyright

Copyright (c) Project Nayuki  
Copyright (c) Abdulrhman Alkhodiry (aalkhodiry@gmail.com)

## Credits

Core QR code generation algorithm based on [Project Nayuki's QR Code generator](https://www.nayuki.io/page/qr-code-generator-library).

Fancy rendering and modular structure by Abdulrhman Alkhodiry.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

