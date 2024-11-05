use std::ops::Add;

// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.
#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct SaturatingU16 {
    value: u16,
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}

impl PartialEq<SaturatingU16> for u16 {
    fn eq(&self, other: &SaturatingU16) -> bool {
        *self == other.value
    }
}

impl From<&SaturatingU16> for SaturatingU16 {
    fn from(value: &SaturatingU16) -> Self {
        Self { value: value.value }
    }
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        Self { value }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        Self {
            value: value as u16,
        }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        Self { value: *value }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        Self {
            value: *value as u16,
        }
    }
}

impl Add for SaturatingU16 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let sum = self.value.saturating_add(other.value);
        Self { value: sum }
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, other: u16) -> Self {
        let sum = self.value.saturating_add(other);
        Self { value: sum }
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, other: &u16) -> Self {
        let sum = self.value.saturating_add(*other);
        Self { value: sum }
    }
}

impl Add for &SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: Self) -> SaturatingU16 {
        let sum = self.value.saturating_add(other.value);
        SaturatingU16 { value: sum }
    }
}

impl Add<u16> for &SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: u16) -> SaturatingU16 {
        let sum = self.value.saturating_add(other);
        SaturatingU16 { value: sum }
    }
}

impl Add<&u16> for &SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: &u16) -> SaturatingU16 {
        let sum = self.value.saturating_add(*other);
        SaturatingU16 { value: sum }
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, other: &SaturatingU16) -> SaturatingU16 {
        let sum = self.value.saturating_add(other.value);
        SaturatingU16 { value: sum }
    }
}

impl Add<u8> for SaturatingU16 {
    type Output = Self;

    fn add(self, other: u8) -> Self {
        let sum = self.value.saturating_add(other as u16);
        Self { value: sum }
    }
}
