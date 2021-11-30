#![cfg_attr(not(test), no_std)]

// extern crate ahrs;

mod nxp;

#[derive(Debug)]
pub struct NxpFusion {
    freq: f32,
    nxp: nxp::Nxp
}

impl NxpFusion {
    pub fn new(freq: f32) -> NxpFusion {
        NxpFusion { freq, nxp: unsafe { nxp::nxp_c_begin(freq) } }
    }

    pub fn update(&mut self, gx: f32, gy: f32, gz: f32, ax: f32, ay: f32, az: f32, mx: f32, my: f32, mz: f32) {
        let nxp = &mut self.nxp as *mut nxp::Nxp;
        unsafe {
            nxp::nxp_c_update(nxp, gx, gy, gz, ax, ay, az, mx, my, mz);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiate_nxp() {
        let nxp = NxpFusion::new(150.);
        println!("{nxp:?}");
    }

    #[test]
    fn update_nxp() {
        let mut nxp = NxpFusion::new(150.);
        println!("{nxp:?}");

        nxp.update(1., 2., 3., 4., 5., 6., 0., 0., 0.);
        println!("{nxp:?}");
    }
}
