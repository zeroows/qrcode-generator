# qrcode-lib

[![Crates.io](https://img.shields.io/crates/v/qrcode-lib.svg)](https://crates.io/crates/qrcode-lib)
[![Documentation](https://docs.rs/qrcode-lib/badge.svg)](https://docs.rs/qrcode-lib)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A powerful and elegant QR Code generator library for Rust with fancy rendering capabilities.

## ✨ Features

### Core QR Code Generation
- ✅ All 40 versions (sizes) and 4 error correction levels
- ✅ Optimized encoding for numeric, alphanumeric, and byte data
- ✅ Automatic version and mask selection
- ✅ Manual control over all parameters when needed
- ✅ Raw module access for custom rendering

### Fancy QR Code Rendering
- 🎨 **Custom Colors** for background, data, and finder patterns
- 🔷 **Module Shapes**: Square, Circle, or Rounded Square
- 🎯 **Finder Shapes**: Square or Rounded corners
- 🖼️ **Center Overlays**: Images or text with automatic safe zones
- 📄 **SVG Output**: High-quality vector graphics
- 🎭 **Multiple Styles**: Pre-configured and custom themes

### Additional Features
- 🚀 Zero runtime dependencies
- 📦 Lightweight and fast
- 🔒 Type-safe API
- 📖 Comprehensive documentation
- 🧪 Well-tested codebase

## 🚀 Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
qrcode-lib = "0.1.0"
```

### Basic Usage

```rust
use qrcode_lib::{QrCode, QrCodeEcc};

fn main() {
    // Create a simple QR code
    let qr = QrCode::encode_text("Hello, World!", QrCodeEcc::Medium)
        .expect("Failed to generate QR code");
    
    println!("QR Code size: {}x{}", qr.size(), qr.size());
    
    // Access individual modules
    for y in 0..qr.size() {
        for x in 0..qr.size() {
            print!("{}", if qr.get_module(x, y) { "██" } else { "  " });
        }
        println!();
    }
}
```

### Fancy QR Code

```rust
use qrcode_lib::fancy::{FancyQr, FancyOptions, ModuleShape, FinderShape};

fn main() {
    // Create a fancy QR code with custom styling
    let qr = FancyQr::from_text("https://example.com")
        .expect("Failed to generate QR code");
    
    let mut options = FancyOptions::default();
    options.color_data = "#6B4B8A".to_string();        // Purple data
    options.color_finder = "#8B5CF6".to_string();      // Light purple finders
    options.shape_module = ModuleShape::Circle;        // Circular dots
    options.shape_finder = FinderShape::Rounded(1.5);  // Rounded corners
    options.center_text = Some("AA".to_string());      // Center text
    
    let svg = qr.render_svg(&options);
    println!("{}", svg);
}
```

## 📚 Examples

The library includes several examples demonstrating different features:

```bash
# Basic QR code in terminal
cargo run --example basic

# Generate SVG file
cargo run --example svg_output

# Fancy styled QR code
cargo run --example fancy_qr

# Branded QR codes with logo overlay
cargo run --example branded

# Advanced usage with segments
cargo run --example advanced

# Multiple style variations
cargo run --example fancy_styled
```

## 🎨 Styling Options

### Colors

Customize colors using hex format:

```rust
let mut options = FancyOptions::default();
options.color_background = "#FAF5FF".to_string(); // Light purple background
options.color_data = "#6B4B8A".to_string();       // Purple data
options.color_finder = "#8B5CF6".to_string();     // Light purple finders
```

### Module Shapes

Choose from three module shapes:

```rust
// Square modules (default)
options.shape_module = ModuleShape::Square;

// Circular modules
options.shape_module = ModuleShape::Circle;

// Rounded square modules (0.0 to 0.5)
options.shape_module = ModuleShape::RoundedSquare(0.3);
```

### Finder Patterns

Customize the three corner patterns:

```rust
// Square finders (default)
options.shape_finder = FinderShape::Square;

// Rounded finders (radius relative to 7-module width)
options.shape_finder = FinderShape::Rounded(1.5);
```

### Center Overlays

Add images or text to the center:

```rust
// Image overlay (URL or base64)
options.center_image_url = Some("https://example.com/logo.png".to_string());
options.overlay_scale = 0.2; // 20% of QR code size

// Or embed SVG as base64 data URI
let logo_svg = std::fs::read_to_string("logo.svg").unwrap();
let logo_base64 = format!("data:image/svg+xml;base64,{}", base64_encode(&logo_svg));
options.center_image_url = Some(logo_base64);
options.overlay_scale = 0.3;

// Or text overlay
options.center_text = Some("AA".to_string());
options.overlay_scale = 0.25;
```

> **Note**: Use High error correction when adding overlays to ensure scannability!
> 
> **Tip**: See `examples/branded.rs` for a complete example of embedding a logo.

## 🔧 Advanced Usage

### Custom Segments

For maximum efficiency, use specific encoding modes:

```rust
use qrcode_lib::{QrCode, QrCodeEcc, QrSegment};

// Numeric mode for numbers (most efficient)
let numeric = QrSegment::make_numeric("123456789");
let qr = QrCode::encode_segments(&[numeric], QrCodeEcc::Low).unwrap();

// Alphanumeric mode for uppercase text
let alphanum = QrSegment::make_alphanumeric("HELLO WORLD");
let qr = QrCode::encode_segments(&[alphanum], QrCodeEcc::Medium).unwrap();

// Binary mode for any data
let bytes = QrSegment::make_bytes(b"Hello, \xF0\x9F\x8C\x8D!");
let qr = QrCode::encode_segments(&[bytes], QrCodeEcc::High).unwrap();
```

### Fine-Grained Control

Control every aspect of QR code generation:

```rust
use qrcode_lib::{QrCode, QrCodeEcc, QrSegment, Version, Mask};

let segments = QrSegment::make_segments("Custom QR");
let qr = QrCode::encode_segments_advanced(
    &segments,
    QrCodeEcc::High,         // Error correction level
    Version::new(5),         // Minimum version
    Version::new(10),        // Maximum version
    Some(Mask::new(3)),      // Specific mask pattern
    false                    // Don't boost ECC automatically
).unwrap();
```

## 📊 Error Correction Levels

| Level | Recovery | Use Case |
|-------|----------|----------|
| Low | ~7% | Clean environments, maximum data |
| Medium | ~15% | General use (recommended) |
| Quartile | ~25% | Moderate damage tolerance |
| High | ~30% | Overlays, high damage tolerance |

## 🏗️ Architecture

The library is organized into clean, focused modules:

- **`types`**: Core types (Version, Mask, QrCodeEcc, DataTooLong)
- **`segment`**: Data segmentation and encoding modes
- **`qrcode`**: QR Code generation (Model 2 specification)
- **`fancy`**: Fancy rendering with custom styles
- **`render`**: Basic rendering utilities (SVG, ASCII art)

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed documentation.

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific example
cargo run --example fancy_styled
```

## 📝 Documentation

```bash
# Build and open documentation
cargo doc --open
```

## 🤝 Contributing

Contributions are welcome! Please feel free to:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Credits

- **Core Algorithm**: Based on [Project Nayuki's QR Code generator](https://www.nayuki.io/page/qr-code-generator-library)
- **Architecture & Fancy Rendering**: Abdulrhman Alkhodiry

## 📧 Contact

**Abdulrhman Alkhodiry**  
Email: aalkhodiry@gmail.com

## 🔗 Links

- [Documentation](https://docs.rs/qrcode-lib)
- [Crates.io](https://crates.io/crates/qrcode-lib)
- [Repository](https://github.com/zeroows/qrcode-generator)
- [QR Code Specification](https://www.iso.org/standard/62021.html)

---

Made with ❤️ by Abdulrhman Alkhodiry

