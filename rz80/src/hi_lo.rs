//! Defines a trait for types that can be divided into 'high' and 'low' parts
//! along with some implementations.

/// Trait for types that have a 'high' and 'low' part.
pub trait HiLo {
    /// Type that represents half the width of the implementing type.
    type T;

    /// Returns the upper half of this value.
    /// 
    /// # Example
    /// ```
    /// # use rz80::hi_lo::HiLo;
    /// assert_eq!(0x1, 0x12_u8.hi());
    /// assert_eq!(0x12, 0x1234_u16.hi());
    /// ```
    fn hi(&self) -> Self::T;
    
    /// Set the upper half of this value.
    /// 
    /// # Arguments
    /// - `val`: value to set the upper part to
    /// 
    /// # Example
    /// ```
    /// # use rz80::hi_lo::HiLo;
    /// let mut n: u8 = 0xab;
    /// n.set_hi(0x4);
    /// assert_eq!(0x4b, n);
    /// ```
    fn set_hi(&mut self, val: Self::T);

    /// Returns the lower half of this value.
    /// 
    /// # Example
    /// ```
    /// # use rz80::hi_lo::HiLo;
    /// assert_eq!(0x2, 0x12_u8.lo());
    /// assert_eq!(0x34, 0x1234_u16.lo());
    /// ```
    fn lo(&self) -> Self::T;

    /// Set the lower half of this value.
    /// 
    /// # Arguments
    /// - `val`: value to set the lower part to
    /// 
    /// # Example
    /// ```
    /// # use rz80::hi_lo::HiLo;
    /// let mut n: u16 = 0xfedc;
    /// n.set_lo(0x10);
    /// assert_eq!(0xfe10, n);
    /// ```
    fn set_lo(&mut self, val: Self::T);
}

impl HiLo for u8 {
    type T = u8;

    #[inline(always)]
    fn hi(&self) -> Self::T {
        (self & 0xf0) >> 4
    }

    #[inline(always)]
    fn set_hi(&mut self, val: Self::T) {
        *self = (*self & 0x0f) | ((val & 0x0f) << 4);
    }

    #[inline(always)]
    fn lo(&self) -> Self::T {
        self & 0x0f
    }

    #[inline(always)]
    fn set_lo(&mut self, val: Self::T) {
        *self = (*self & 0xf0) | (val & 0x0f);
    }
}

impl HiLo for u16 {
    type T = u8;

    #[inline(always)]
    fn hi(&self) -> Self::T {
        ((self & 0xff00) >> 8) as u8
    }

    #[inline(always)]
    fn set_hi(&mut self, val: Self::T) {
        *self = (*self & 0x00ff) | ((val as u16) << 8);
    }

    #[inline(always)]
    fn lo(&self) -> Self::T {
        (self & 0x00ff) as u8
    }

    #[inline(always)]
    fn set_lo(&mut self, val: Self::T) {
        *self = (*self & 0xff00) | (val as u16);
    }
}