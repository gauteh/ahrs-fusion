#![cfg_attr(not(test), no_std)]

#[derive(Debug, Default)]
pub struct Sensor {
    freq: f32,
}

impl Sensor {
    pub fn new(freq: f32) -> Sensor {
        Sensor { freq }
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
