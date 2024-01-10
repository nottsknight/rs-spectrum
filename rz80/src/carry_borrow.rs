//! Provides traits for types that can track carries and borrows.
use std::ops::{Add, Sub};

/// Trait for types that can track carries when they are added.
pub trait AddCarry : Sized + Add<Self> {
    /// Add this value to another, and return the sum along with two flags
    /// indicating whether a carry occured from bit 3 or bit 7 (least significant
    /// bit is bit 0).
    /// 
    /// # Arguments
    /// - `rhs`: second addend
    /// 
    /// # Example
    /// ```
    /// # use rz80::carry_borrow::AddCarry;
    /// let (sum, carry3, carry7) = 0x1f.add_carry(0x21);
    /// assert_eq!(0x40, sum);
    /// assert!(carry3);
    /// assert!(!carry7);
    /// ```
    fn add_carry(self, rhs: Self) -> (Self, bool, bool);
}

impl AddCarry for u8 {
    fn add_carry(self, rhs: Self) -> (Self, bool, bool) {
        // bit 3 carry
        let lo_sum = (self & 0x0f) + (rhs & 0x0f);
        let carry3 = lo_sum > 0x0f;
        // bit 7 carry
        let hi_sum = (self as u16) + (rhs as u16);
        let carry7 = hi_sum > 0xff;

        ((hi_sum & 0xff) as u8, carry3, carry7)
    }
}

/// Trait for types that can track borrows when they are subtracted.
pub trait SubBorrow : Sized + Sub<Self> {
    /// Subtract another value from this, and return the difference along with
    /// two flags indicating whether a borrow was required from bit 4 (least
    /// significant bit is bit 0).
    ///
    /// # Arguments
    /// - `rhs`: the subtrahend
    /// 
    /// # Example
    /// ```
    /// # use rz80::carry_borrow::SubBorrow;
    /// let (diff, borrow) = 0x17.sub_borrow(0x08);
    /// assert_eq!(0x0f, diff);
    /// assert!(borrow);
    /// ```
    fn sub_borrow(self, rhs: Self) -> (Self, bool);
}

impl SubBorrow for u8 {
    fn sub_borrow(self, rhs: Self) -> (Self, bool) {
        let lo1 = self & 0x0f;
        let lo2 = rhs & 0x0f;
        (self - rhs, lo1 < lo2)
    }
}