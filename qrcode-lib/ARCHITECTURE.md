# Architecture Documentation

## Overview

This QR Code Generator library is a well-structured library that separates concerns into distinct modules, making it maintainable, testable, and easy to extend.

## Module Structure

```
qrcode-lib/
├── src/
│   ├── lib.rs          # Public API and module exports
│   ├── types.rs        # Core types (QrCodeEcc, Version, Mask, DataTooLong)
│   ├── segment.rs      # QR segment encoding (QrSegment, QrSegmentMode, BitBuffer)
│   ├── qrcode.rs       # Core QR code generation logic
│   ├── fancy.rs        # Fancy rendering with custom styles
│   └── render.rs       # Basic rendering utilities (SVG, ASCII)
└── examples/
    ├── basic.rs        # Simple QR code in terminal
    ├── svg_output.rs   # Generate SVG files
    ├── fancy_qr.rs     # Fancy styled QR code
    ├── advanced.rs     # Advanced segment usage
    └── fancy_styled.rs # Multiple style examples
```

## Module Responsibilities

### 1. `types.rs` - Core Types

**Purpose**: Define fundamental types used throughout the library.

**Key Types**:
- `QrCodeEcc`: Error correction levels (Low, Medium, Quartile, High)
- `Version`: QR code version numbers (1-40)
- `Mask`: Mask patterns (0-7)
- `DataTooLong`: Error type for data capacity errors
- `get_bit()`: Utility function for bit manipulation

**Design Notes**: 
- All types are well-documented with clear constraints
- Implements standard traits (Clone, Copy, PartialEq, Eq, Debug)
- Version and Mask have const constructors with validation

### 2. `segment.rs` - QR Segment Encoding

**Purpose**: Handle data segmentation and encoding modes.

**Key Types**:
- `QrSegment`: Represents a data segment
- `QrSegmentMode`: Encoding modes (Numeric, Alphanumeric, Byte, Kanji, ECI)
- `BitBuffer`: Appendable bit sequence

**Key Functions**:
- `make_bytes()`: Encode binary data
- `make_numeric()`: Encode numeric strings efficiently
- `make_alphanumeric()`: Encode uppercase text efficiently
- `make_segments()`: Auto-select best encoding mode
- `make_eci()`: Extended Channel Interpretation

**Design Notes**:
- Automatic mode selection optimizes encoding efficiency
- Each mode has specific validation rules
- BitBuffer provides safe bit manipulation

### 3. `qrcode.rs` - Core Generation Logic

**Purpose**: Implement the QR Code Model 2 specification.

**Key Type**: `QrCode`

**Public Methods**:
- High-level: `encode_text()`, `encode_binary()`
- Mid-level: `encode_segments()`, `encode_segments_advanced()`
- Low-level: `encode_codewords()`
- Accessors: `size()`, `get_module()`, `version()`, `mask()`, `error_correction_level()`

**Internal Implementation**:
- Reed-Solomon error correction
- Mask pattern selection and application
- Function pattern drawing (finders, alignment, timing)
- Penalty score calculation
- Data interleaving

**Design Notes**:
- Three levels of API (high/mid/low) for different use cases
- Automatic version selection within constraints
- Optimal mask pattern detection
- Immutable after construction

### 4. `fancy.rs` - Fancy Rendering

**Purpose**: Provide stylized QR code rendering with custom appearance.

**Key Types**:
- `FancyQr`: Wrapper around QrCode with rendering capabilities
- `FancyOptions`: Configuration for colors, shapes, and overlays
- `ModuleShape`: Square, Circle, or RoundedSquare
- `FinderShape`: Square or Rounded corners

**Key Features**:
- Custom colors (background, data, finders)
- Multiple module shapes
- Rounded finder patterns
- Center overlays (image or text)
- Safe zone calculation for overlays
- High-quality SVG output

**Design Notes**:
- Builder pattern for configuration
- High error correction mandatory for overlays
- Finder patterns drawn separately for custom styling
- Safe zone prevents overlay from damaging critical data

### 5. `render.rs` - Basic Rendering

**Purpose**: Provide simple rendering utilities.

**Key Functions**:
- `to_svg_string()`: Generate simple SVG
- `to_ascii_art()`: Terminal-friendly display
- `to_debug_string()`: Debug representation

**Design Notes**:
- Minimal dependencies
- Fast and efficient
- Useful for testing and debugging

### 6. `lib.rs` - Public API

**Purpose**: Define the public interface and module visibility.

**Exports**:
- Core types: `QrCode`, `QrCodeEcc`, `Version`, `Mask`, `DataTooLong`
- Segments: `QrSegment`, `QrSegmentMode`, `BitBuffer`
- Public modules: `fancy`, `render`

**Design Notes**:
- Clean, minimal public API
- Well-documented with examples
- Module-level documentation
- Hides internal implementation details

## Data Flow

### Basic QR Code Generation

```
User Input (text/bytes)
    ↓
QrSegment::make_segments() → Auto-select encoding mode
    ↓
QrCode::encode_segments() → Find optimal version
    ↓
QrCode::encode_codewords() → Generate modules
    ↓
    ├─ draw_function_patterns() → Finders, alignment, timing
    ├─ add_ecc_and_interleave() → Reed-Solomon ECC
    ├─ draw_codewords() → Place data
    └─ apply_mask() → XOR with selected mask
    ↓
QrCode (immutable) ← Access via get_module()
```

### Fancy QR Code Rendering

```
QrCode
    ↓
FancyQr::from_qrcode() or FancyQr::from_text()
    ↓
FancyOptions → Configure colors, shapes, overlays
    ↓
render_svg()
    ├─ Render background
    ├─ Calculate safe zone
    ├─ Render data modules (skip finders & safe zone)
    ├─ Render custom finder patterns
    └─ Render center overlay
    ↓
SVG String
```

## Design Principles

### 1. Separation of Concerns
- Each module has a single, well-defined responsibility
- Clear boundaries between encoding, generation, and rendering

### 2. API Layering
- High-level: Simple, opinionated (`encode_text()`)
- Mid-level: Flexible (`encode_segments()`)
- Low-level: Full control (`encode_codewords()`)

### 3. Type Safety
- Strong types prevent invalid states
- Version and Mask types enforce valid ranges
- Error types provide actionable information

### 4. Performance
- Minimal allocations
- Efficient bit manipulation
- Pre-calculated lookup tables (when appropriate)

### 5. Extensibility
- Public modules allow custom rendering
- Module access enables custom QR manipulation
- Builder patterns for configuration

### 6. Documentation
- Module-level documentation explains purpose
- Function-level documentation with examples
- Examples demonstrate common use cases

## Testing Strategy

### Unit Tests
- Each module has its own test suite
- Tests cover both success and error cases
- Doctests ensure examples compile and run

### Integration Tests
- Examples serve as integration tests
- Verify end-to-end functionality
- Demonstrate real-world usage

### Test Coverage
```
├── types.rs     : Type constraints and error handling
├── segment.rs   : Encoding modes and validation
├── qrcode.rs    : QR code generation (implicit via examples)
├── fancy.rs     : Rendering with various styles
└── render.rs    : ASCII and SVG output
```

## Future Extensions

### Potential Improvements

1. **Additional Rendering Formats**
   - PNG/JPEG raster output
   - PDF generation
   - HTML Canvas rendering

2. **More Styles**
   - Gradient fills
   - Custom images for modules
   - Animation support

3. **Performance**
   - Parallel mask evaluation
   - SIMD optimizations
   - Pre-computed tables

4. **Features**
   - QR code reading/decoding
   - Micro QR codes
   - Structured append mode

5. **Utilities**
   - QR code validation
   - Capacity calculator
   - Version estimator

## Contribution Guidelines

### Adding New Features

1. **New Encoding Modes**: Extend `segment.rs`
2. **New Rendering Styles**: Extend `fancy.rs`
3. **New Output Formats**: Add to `render.rs` or new module
4. **Core Algorithm Changes**: Modify `qrcode.rs` (requires careful testing)

### Code Style

- Follow Rust standard conventions
- Add documentation for public APIs
- Include examples in doctests
- Write unit tests for new functionality
- Update ARCHITECTURE.md for significant changes

## Performance Characteristics

### Time Complexity
- Encoding: O(n) where n is data length
- Mask evaluation: O(8 × size²) for automatic selection
- Rendering: O(size²)

### Space Complexity
- QR Code: O(size²) for module storage
- ECC calculation: O(codewords)
- Rendering: O(size² × format_overhead)

### Typical Performance
- Small QR code (21×21): < 1ms generation
- Large QR code (177×177): < 10ms generation
- Fancy rendering: < 5ms additional

## Dependencies

The library has **zero runtime dependencies** for core functionality, making it:
- Lightweight
- Easy to audit
- Suitable for embedded systems
- Fast to compile

## Versioning

The project follows [Semantic Versioning](https://semver.org/):
- MAJOR: Breaking API changes
- MINOR: New features, backwards compatible
- PATCH: Bug fixes, backwards compatible

## License

MIT License - See LICENSE file for details.

## Copyright

Copyright (c) Project Nayuki  
Copyright (c) Abdulrhman Alkhodiry (aalkhodiry@gmail.com)

Based on Project Nayuki's QR Code generator.  
Modular architecture and fancy rendering by Abdulrhman Alkhodiry.

