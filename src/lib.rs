//! Access to the words held by a closed generated String newtype.
#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;
use alloc::string::String;

/// The explicit translator capability to read and create a closed word.
pub trait Worded: Sized {
    /// Read the words held by this value.
    fn words(&self) -> String;

    /// Create a value holding these words.
    fn word(words: String) -> Self;
}
