//! This crate adds the `take_until` method as an extension for iterators.
//!
//! # Example
//!
//! Parsing the next base 128 varint from a byte slice.
//!
//! ```
//! use take_until::TakeUntilExt;
//!
//! let varint = &[0b1010_1100u8, 0b0000_0010, 0b1000_0001];
//! let int: u32 = varint
//!     .iter()
//!     .take_until(|b| (**b & 0b1000_0000) == 0)
//!     .enumerate()
//!     .fold(0, |acc, (i, b)| {
//!         acc | ((*b & 0b0111_1111) as u32) << (i * 7)
//!      });
//! assert_eq!(300, int);
//! ```
mod take_until;

pub use crate::take_until::TakeUntilExt;
