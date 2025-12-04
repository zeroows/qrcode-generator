/* 
 * QR Code generator library (Rust)
 * 
 * Copyright (c) Project Nayuki. (MIT License)
 * Copyright (c) Abdulrhman Alkhodiry (aalkhodiry@gmail.com)
 * https://www.nayuki.io/page/qr-code-generator-library
 */

//! Basic types used throughout the QR code library.

/// The error correction level in a QR Code symbol.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum QrCodeEcc {
	/// The QR Code can tolerate about  7% erroneous codewords.
	Low     ,
	/// The QR Code can tolerate about 15% erroneous codewords.
	Medium  ,
	/// The QR Code can tolerate about 25% erroneous codewords.
	Quartile,
	/// The QR Code can tolerate about 30% erroneous codewords.
	High    ,
}

impl QrCodeEcc {
	// Returns an unsigned 2-bit integer (in the range 0 to 3).
	pub(crate) fn ordinal(self) -> usize {
		use QrCodeEcc::*;
		match self {
			Low      => 0,
			Medium   => 1,
			Quartile => 2,
			High     => 3,
		}
	}
	
	// Returns an unsigned 2-bit integer (in the range 0 to 3).
	pub(crate) fn format_bits(self) -> u8 {
		use QrCodeEcc::*;
		match self {
			Low      => 1,
			Medium   => 0,
			Quartile => 3,
			High     => 2,
		}
	}
}

/// A number between 1 and 40 (inclusive).
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Version(u8);

impl Version {
	/// The minimum version number supported in the QR Code Model 2 standard.
	pub const MIN: Version = Version( 1);
	
	/// The maximum version number supported in the QR Code Model 2 standard.
	pub const MAX: Version = Version(40);
	
	/// Creates a version object from the given number.
	/// 
	/// Panics if the number is outside the range [1, 40].
	pub const fn new(ver: u8) -> Self {
		assert!(Version::MIN.value() <= ver && ver <= Version::MAX.value(), "Version number out of range");
		Self(ver)
	}
	
	/// Returns the value, which is in the range [1, 40].
	pub const fn value(self) -> u8 {
		self.0
	}
}

/// A number between 0 and 7 (inclusive).
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Mask(u8);

impl Mask {
	/// Creates a mask object from the given number.
	/// 
	/// Panics if the number is outside the range [0, 7].
	pub const fn new(mask: u8) -> Self {
		assert!(mask <= 7, "Mask value out of range");
		Self(mask)
	}
	
	/// Returns the value, which is in the range [0, 7].
	pub const fn value(self) -> u8 {
		self.0
	}
}

/// The error type when the supplied data does not fit any QR Code version.
///
/// Ways to handle this exception include:
/// 
/// - Decrease the error correction level if it was greater than `QrCodeEcc::Low`.
/// - If the `encode_segments_advanced()` function was called, then increase the maxversion
///   argument if it was less than `Version::MAX`. (This advice does not apply to the
///   other factory functions because they search all versions up to `Version::MAX`.)
/// - Split the text data into better or optimal segments in order to reduce the number of bits required.
/// - Change the text or binary data to be shorter.
/// - Change the text to fit the character set of a particular segment mode (e.g. alphanumeric).
/// - Propagate the error upward to the caller/user.
#[derive(Debug, Clone)]
pub enum DataTooLong {
	/// A segment is too long to fit in its length field
	SegmentTooLong,
	/// Data length exceeds capacity (data length, max capacity)
	DataOverCapacity(usize, usize),
}

impl std::error::Error for DataTooLong {}

impl std::fmt::Display for DataTooLong {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			Self::SegmentTooLong => write!(f, "Segment too long"),
			Self::DataOverCapacity(datalen, maxcapacity) =>
				write!(f, "Data length = {} bits, Max capacity = {} bits", datalen, maxcapacity),
		}
	}
}

/// Returns true iff the i'th bit of x is set to 1.
pub(crate) fn get_bit(x: u32, i: i32) -> bool {
	(x >> i) & 1 != 0
}

