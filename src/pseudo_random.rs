#![allow(dead_code)]

/// XorShiftRng is a pseudorandom number generator based on the XorShift algorithm.
pub struct XorShiftRng {
    seed: u64,
}

impl XorShiftRng {
    /// Creates a new instance of XorShiftRng with the given seed value.
    ///
    /// # Arguments
    ///
    /// * `seed` - The initial seed value for the random number generator.
    ///
    /// # Returns
    ///
    /// A new XorShiftRng instance.
    pub fn new(seed: u64) -> Self {
        XorShiftRng { seed }
    }

    /// Generates the next random 64-bit unsigned integer using the XorShift algorithm.
    ///
    /// # Returns
    ///
    /// A random 64-bit unsigned integer.
    pub fn next_u32(&mut self) -> u64 {
        self.seed ^= self.seed << 11;
        self.seed ^= self.seed >> 21;
        self.seed ^= self.seed << 13;
        self.seed
    }

    /// Generates a random 32-bit floating-point number within the specified range.
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum value (inclusive) of the range.
    /// * `max` - The maximum value (exclusive) of the range.
    ///
    /// # Returns
    ///
    /// A random 32-bit floating-point number within the specified range.
    pub fn random_f32_in_range(&mut self, min: f32, max: f32) -> f32 {
        let random_f64 = self.next_u32() as f64 / (u32::MAX as f64);
        let scaled_f32 = min + (max - min) * random_f64 as f32;
        scaled_f32
    }
}
/// LcgRng is a pseudorandom number generator based on the Linear Congruential Generator (LCG) algorithm.
pub struct LcgRng {
    state: u64,
}

impl LcgRng {
    /// Creates a new instance of LcgRng with the given seed value.
    ///
    /// # Arguments
    ///
    /// * `seed` - The initial seed value for the random number generator.
    ///
    /// # Returns
    ///
    /// A new LcgRng instance.
    pub fn new(seed: u64) -> LcgRng {
        LcgRng { state: seed }
    }

    /// Generates the next random 64-bit unsigned integer using the LCG algorithm.
    ///
    /// # Returns
    ///
    /// A random 64-bit unsigned integer.
    pub fn next(&mut self) -> u64 {
        const A: u64 = 5576963409015389;
        const C: u64 = 1;
        self.state = A.wrapping_mul(self.state).wrapping_add(C);
        self.state
    }

    /// Generates the next random 64-bit floating-point number in the range [0, 1) using the LCG algorithm.
    ///
    /// # Returns
    ///
    /// A random 64-bit floating-point number in the range [0, 1).
    pub fn next_f64(&mut self) -> f64 {
        self.next() as f64 / u64::MAX as f64
    }

    /// Generates a random 32-bit floating-point number within the specified range.
    ///
    /// # Arguments
    ///
    /// * `min` - The minimum value (inclusive) of the range.
    /// * `max` - The maximum value (exclusive) of the range.
    ///
    /// # Returns
    ///
    /// A random 32-bit floating-point number within the specified range.
    pub fn random_f32_in_range(&mut self, min: f32, max: f32) -> f32 {
        let random_f64 = self.next_f64();
        let scaled_f32 = min + (max - min) * random_f64 as f32;
        scaled_f32
    }
}
