//! Provides common functions for performing ALU operations.

pub struct ALU;

#[derive(Debug, PartialEq)]
pub struct ALUAdd {
    pub sum: u8,
    pub carry3: bool,
    pub carry7: bool,
    pub overflow: bool,
}

impl ALUAdd {
    pub fn new(sum: u8, carry3: bool, carry7: bool, overflow: bool) -> ALUAdd {
        ALUAdd {
            sum,
            carry3,
            carry7,
            overflow,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ALUSub(pub u8, pub bool, pub bool);

impl ALU {
    /// Add the two arguments, tracking overflows.
    ///
    /// # Arguments
    /// - `lhs`: first addend
    /// - `rhs`: second addend
    ///
    /// # Examples
    /// ```
    /// # use rz80::alu::{ALU, ALUAdd};
    /// assert_eq!(ALUAdd::new(5, false, false, false), ALU::add(2, 3));
    /// assert_eq!(ALUAdd::new(0x11, true, false, false), ALU::add(0x0f, 2));
    /// assert_eq!(ALUAdd::new(0xef, false, true, true), ALU::add(0xf0, 0xff));
    /// assert_eq!(ALUAdd::new(0xfe, true, true, true), ALU::add(0xff, 0xff));
    /// ```
    pub fn add(lhs: u8, rhs: u8) -> ALUAdd {
        // bit 3 carry
        let lo_sum = (lhs & 0x0f) + (rhs & 0x0f);
        let carry3 = lo_sum > 0x0f;
        // bit 7 carry
        let (sum, overflow) = lhs.overflowing_add(rhs);

        ALUAdd::new(sum, carry3, overflow, overflow)
    }

    /// Increment the argument, tracking overflows.
    ///
    /// # Argument
    /// - `lhs`: value to increment
    ///
    /// # Examples
    /// ```
    /// # use rz80::alu::{ALU, ALUAdd};
    /// assert_eq!(ALUAdd::new(0x2, false, false, false), ALU::increment(1));
    /// assert_eq!(ALUAdd::new(0x10, true, false, false), ALU::increment(0x0f));
    /// assert_eq!(ALUAdd::new(0x00, true, true, true), ALU::increment(0xff));
    /// ```
    #[inline(always)]
    pub fn increment(lhs: u8) -> ALUAdd {
        ALU::add(lhs, 1)
    }

    pub fn sub(lhs: u8, rhs: u8) -> ALUSub {
        let lo1 = lhs & 0x0f;
        let lo2 = rhs & 0x0f;
        let (diff, overflow) = lhs.overflowing_sub(rhs);
        ALUSub(diff, lo1 < lo2, overflow)
    }
}
