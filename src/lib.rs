//! All byte-level manipulations are assumed (expected) to be in big-endian form.
//! All bit-values may be mapped to a corresponding boolean value, with
//! "0" == `false`, and "1" == `true`.
//!
//! Therefore, the byte "1100_1101" will be indexed as follows:
//!
//! | Index | Value | Boolean |
//! |-------|-------|---------|
//! | 0     | 1     | True    |
//! | 1     | 0     | False   |
//! | 2     | 1     | True    |
//! | 3     | 1     | True    |
//! | 4     | 0     | False   |
//! | 5     | 0     | False   |
//! | 6     | 1     | True    |
//! | 7     | 1     | True    |

use std::iter::{ExactSizeIterator, IntoIterator, Iterator};
use std::ops::Deref;

/// Wraps a single bit, whose value is indicated as follows:
///
/// - `0`: *False*
/// - `1`: *True*
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Bit(bool);

impl Bit {
    /// Getter for a bit that has a value of "1".
    pub fn get_zero_bit() -> Self {
        Self(false)
    }

    /// Getter for a bit that has a value of "0".
    pub fn get_one_bit() -> Self {
        Self(true)
    }
    /// True if the bit encapsulated is a `1`.
    pub fn is_one(&self) -> bool {
        **self
    }

    /// True if the bit encapsulated is a `0`.
    pub fn is_zero(&self) -> bool {
        !self.is_one()
    }
}

impl From<bool> for Bit {
    fn from(bit: bool) -> Self {
        Self(bit)
    }
}

impl From<u8> for Bit {
    fn from(byte: u8) -> Self {
        if byte == 0 {
            Self(false)
        } else {
            Self(true)
        }
    }
}

impl Deref for Bit {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A composition of 8-bit values, as a byte.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Byte(u8);

impl Default for Byte {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Byte {
    /// Getter for the right-most bit in the byte. (Eighth left-most bit.)
    pub fn get_0(&self) -> Bit {
        self.get_bit(0)
    }

    /// Getter for the second right-most bit in the byte. (Seventh left-most bit.)
    pub fn get_1(&self) -> Bit {
        self.get_bit(1)
    }

    /// Getter for the third right-most bit in the byte. (Sixth left-most bit.)
    pub fn get_2(&self) -> Bit {
        self.get_bit(2)
    }

    /// Getter for the fourth right-most bit in the byte. (Fifth left-most bit.)
    pub fn get_3(&self) -> Bit {
        self.get_bit(3)
    }

    /// Getter for the fifth right-most bit in the byte. (Fourth left-most bit.)
    pub fn get_4(&self) -> Bit {
        self.get_bit(4)
    }

    /// Getter for the sixth right-most bit in the byte. (Third left-most bit.)
    pub fn get_5(&self) -> Bit {
        self.get_bit(5)
    }

    /// Getter for the seventh right-most bit in the byte. (Second left-most bit.)
    pub fn get_6(&self) -> Bit {
        self.get_bit(6)
    }

    /// Getter for the eighth right-most bit in the byte. (Left-most bit.)
    pub fn get_7(&self) -> Bit {
        self.get_bit(7)
    }

    /// Arbitrary getter for the bit at index `idx`.
    fn get_bit(&self, idx: usize) -> Bit {
        Bit::from((usize::from(self.0) & (1 << idx)) > 0)
    }

    /// Setter for the right-most bit in the byte. (Eighth left-most bit.)
    pub fn set_0(&mut self, val: bool) {
        self.set_bit(val, 0);
    }

    /// Setter for the second right-most bit in the byte. (Seventh left-most bit.)
    pub fn set_1(&mut self, val: bool) {
        self.set_bit(val, 1);
    }

    /// Setter for the third right-most bit in the byte. (Sixth left-most bit.)
    pub fn set_2(&mut self, val: bool) {
        self.set_bit(val, 2);
    }

    /// Setter for the fourth right-most bit in the byte. (Fifth left-most bit.)
    pub fn set_3(&mut self, val: bool) {
        self.set_bit(val, 3);
    }

    /// Setter for the fifth right-most bit in the byte. (Fourth left-most bit.)
    pub fn set_4(&mut self, val: bool) {
        self.set_bit(val, 4);
    }

    /// Setter for the sixth right-most bit in the byte. (Third left-most bit.)
    pub fn set_5(&mut self, val: bool) {
        self.set_bit(val, 5);
    }

    /// Setter for the seventh right-most bit in the byte. (Second left-most bit.)
    pub fn set_6(&mut self, val: bool) {
        self.set_bit(val, 6);
    }

    /// Setter for the eighth right-most bit in the byte. (Left-most bit.)
    pub fn set_7(&mut self, val: bool) {
        self.set_bit(val, 7);
    }

    /// Arbitrary setter for the bit at index `idx`.
    fn set_bit(&mut self, val: bool, idx: usize) {
        if val {
            self.0 |= 1 << idx;
        } else {
            self.0 &= !(1 << idx);
        }
    }

    /// Getter for the byte-representation of the internal bits being managed.
    pub fn as_byte(&self) -> u8 {
        self.0
    }
}

impl From<u8> for Byte {
    fn from(byte: u8) -> Self {
        Self(byte)
    }
}

impl From<[bool; 8]> for Byte {
    fn from(bits: [bool; 8]) -> Self {
        let mut byte = Byte::default();
        if bits[7] {
            byte.set_0(true);
        };
        if bits[6] {
            byte.set_1(true);
        };
        if bits[5] {
            byte.set_2(true);
        };
        if bits[4] {
            byte.set_3(true);
        };
        if bits[3] {
            byte.set_4(true);
        };
        if bits[2] {
            byte.set_5(true);
        };
        if bits[1] {
            byte.set_6(true);
        };
        if bits[0] {
            byte.set_7(true);
        };
        byte
    }
}

impl From<[Bit; 8]> for Byte {
    fn from(bits: [Bit; 8]) -> Self {
        Self::from(bits.map(|b| *b))
    }
}

impl IntoIterator for Byte {
    type Item = Bit;

    type IntoIter = BitIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            byte: self.clone(),
            idx: 0,
        }
    }
}

impl Deref for Byte {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// An iterator structure, wrapping a byte object and allowing for bit-level iteration.
struct BitIter {
    /// The byte-object being wrapped by this iterator.
    byte: Byte,
    /// The index of the next bit that will be dispatched by this iterator.
    idx: usize,
}

impl From<Byte> for BitIter {
    fn from(byte: Byte) -> Self {
        Self { byte, idx: 0 }
    }
}

impl Iterator for BitIter {
    type Item = Bit;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < 8 {
            let res = Some(self.byte.get_bit(self.idx));
            self.idx += 1;
            res
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(8))
    }
}

impl ExactSizeIterator for BitIter {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getters() {
        let zero_byte = Byte::from(0);
        assert!(!*zero_byte.get_0());
        assert!(!*zero_byte.get_1());
        assert!(!*zero_byte.get_2());
        assert!(!*zero_byte.get_3());
        assert!(!*zero_byte.get_4());
        assert!(!*zero_byte.get_5());
        assert!(!*zero_byte.get_6());
        assert!(!*zero_byte.get_7());

        let one_byte = Byte::from(u8::MAX);
        assert!(*one_byte.get_0());
        assert!(*one_byte.get_1());
        assert!(*one_byte.get_2());
        assert!(*one_byte.get_3());
        assert!(*one_byte.get_4());
        assert!(*one_byte.get_5());
        assert!(*one_byte.get_6());
        assert!(*one_byte.get_7());

        let rand_byte = Byte::from(0b1010_1010);
        assert!(!*rand_byte.get_0());
        assert!(*rand_byte.get_1());
        assert!(!*rand_byte.get_2());
        assert!(*rand_byte.get_3());
        assert!(!*rand_byte.get_4());
        assert!(*rand_byte.get_5());
        assert!(!*rand_byte.get_6());
        assert!(*rand_byte.get_7());
    }

    #[test]
    fn test_setters() {
        let mut test_byte = Byte::from(0);
        test_byte.set_0(true);
        test_byte.set_0(false);
        assert!(!*test_byte.get_0());

        test_byte.set_0(true);
        test_byte.set_1(true);
        test_byte.set_2(true);
        test_byte.set_3(true);
        assert!(*test_byte.get_0());
        assert!(*test_byte.get_1());
        assert!(*test_byte.get_2());
        assert!(*test_byte.get_3());
        assert!(!*test_byte.get_4());
        assert!(!*test_byte.get_5());
        assert!(!*test_byte.get_6());
        assert!(!*test_byte.get_7());

        test_byte.set_4(true);
        test_byte.set_5(true);
        test_byte.set_6(true);
        test_byte.set_7(true);
        assert!(*test_byte.get_0());
        assert!(*test_byte.get_1());
        assert!(*test_byte.get_2());
        assert!(*test_byte.get_3());
        assert!(*test_byte.get_4());
        assert!(*test_byte.get_5());
        assert!(*test_byte.get_6());
        assert!(*test_byte.get_7());

        test_byte.set_0(false);
        test_byte.set_1(false);
        test_byte.set_2(false);
        test_byte.set_3(false);
        test_byte.set_4(false);
        test_byte.set_5(false);
        test_byte.set_6(false);
        test_byte.set_7(false);
        assert!(!*test_byte.get_0());
        assert!(!*test_byte.get_1());
        assert!(!*test_byte.get_2());
        assert!(!*test_byte.get_3());
        assert!(!*test_byte.get_4());
        assert!(!*test_byte.get_5());
        assert!(!*test_byte.get_6());
        assert!(!*test_byte.get_7());
    }

    #[test]
    fn test_new_bit() {
        let zero = Bit::from(false);
        let one = Bit::from(true);

        assert!(!*zero);
        assert!(*one);

        assert_eq!(zero, Bit::from(0));
        assert_eq!(one, Bit::from(1));
        assert_eq!(one, Bit::from(2));
        assert_eq!(one, Bit::from(100));
        assert_ne!(zero, one);
    }

    #[test]
    fn test_new_byte() {
        let num = 0b1010_0001u8;
        let bools = [true, false, true, false, false, false, false, true];
        let bits = [
            Bit::get_one_bit(),
            Bit::get_zero_bit(),
            Bit::get_one_bit(),
            Bit::get_zero_bit(),
            Bit::get_zero_bit(),
            Bit::get_zero_bit(),
            Bit::get_zero_bit(),
            Bit::get_one_bit(),
        ];

        assert_eq!(161, Byte::from(num).as_byte());
        assert_eq!(161, Byte::from(bools).as_byte());
        assert_eq!(161, Byte::from(bits).as_byte());
    }

    #[test]
    fn test_iter() {
        let mut test_byte = Byte::from(0);
        for bit in test_byte {
            assert!(!*bit);
        }

        test_byte.set_1(true);
        test_byte.set_2(true);
        test_byte.set_3(true);
        test_byte.set_4(true);
        test_byte.set_6(true);
        let mut iter = test_byte.into_iter();
        assert_eq!(iter.next(), Some(Bit::get_zero_bit()));
        assert_eq!(iter.next(), Some(Bit::get_one_bit()));
        assert_eq!(iter.next(), Some(Bit::get_one_bit()));
        assert_eq!(iter.next(), Some(Bit::get_one_bit()));
        assert_eq!(iter.next(), Some(Bit::get_one_bit()));
        assert_eq!(iter.next(), Some(Bit::get_zero_bit()));
        assert_eq!(iter.next(), Some(Bit::get_one_bit()));
        assert_eq!(iter.next(), Some(Bit::get_zero_bit()));
        assert_eq!(iter.next(), None);
    }
}
