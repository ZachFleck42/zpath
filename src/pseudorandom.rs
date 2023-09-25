#![allow(dead_code)]

use std::time::{SystemTime, UNIX_EPOCH};

pub struct XorShiftRng {
    seed: u64,
}

impl XorShiftRng {
    fn new(seed: u64) -> Self {
        XorShiftRng { seed }
    }

    fn next_u32(&mut self) -> u32 {
        self.seed ^= self.seed << 11;
        self.seed ^= self.seed >> 21;
        self.seed ^= self.seed << 13;
        self.seed as u32
    }

    pub fn random_f32_in_range(min: f32, max: f32) -> f32 {
        let now = SystemTime::now();
        let since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
        let seed = since_epoch.as_secs() ^ since_epoch.subsec_nanos() as u64;

        let mut rng = XorShiftRng::new(seed);

        let random_f64 = rng.next_u32() as f64 / (u32::MAX as f64);
        let scaled_f32 = min + (max - min) * random_f64 as f32;
        scaled_f32
    }
}

pub struct LcgRng {
    state: u64,
}

impl LcgRng {
    fn new(seed: u64) -> LcgRng {
        LcgRng { state: seed }
    }

    fn next(&mut self) -> u64 {
        const A: u64 = 5576963409015389;
        const C: u64 = 1;
        self.state = A.wrapping_mul(self.state).wrapping_add(C);
        self.state
    }

    fn next_f64(&mut self) -> f64 {
        self.next() as f64 / u64::MAX as f64
    }

    pub fn random_f32_in_range(min: f32, max: f32) -> f32 {
        let now = SystemTime::now();
        let since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
        let seed = since_epoch.as_secs() ^ since_epoch.subsec_nanos() as u64;

        let mut rng = LcgRng::new(seed);

        let random_f64 = rng.next_f64();
        let scaled_f32 = min + (max - min) * random_f64 as f32;
        scaled_f32
    }
}
