#![cfg_attr(not(test), no_std)]

// pub mod ffi;

#[derive(Debug, Default)]
pub struct NxpFusion {
    freq: f32,
}

impl NxpFusion {
    pub fn new(freq: f32) -> NxpFusion {
        NxpFusion { freq }
    }

    pub fn update(&mut self, gx: f32, gy: f32, gz: f32, ax: f32, ay: f32, az: f32, mx: f32, my: f32, mz: f32) {
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
