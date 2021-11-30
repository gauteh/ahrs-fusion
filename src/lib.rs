#![cfg_attr(not(test), no_std)]

mod nxp;

#[derive(Debug)]
pub struct NxpFusion {
    freq: f32,
    nxp: nxp::Nxp,
}

impl NxpFusion {
    /// Initializes the 9DOF Kalman filter.
    ///
    /// freq: The sensor sample rate in Herz (`s^-1`).
    pub fn new(freq: f32) -> NxpFusion {
        NxpFusion {
            freq,
            nxp: unsafe { nxp::nxp_c_begin(freq) },
        }
    }

    /// Updates the filter with new gyroscope, accelerometer, and
    /// magnetometer data. For roll, pitch, and yaw the accelerometer values can be
    /// either m/s^2 or g, but for linear acceleration they have to be in g.
    ///
    /// 9DOF orientation function implemented using a 12 element Kalman filter
    ///
    ///
    /// * gx The gyroscope x axis. In DPS.
    /// * gy The gyroscope y axis. In DPS.
    /// * gz The gyroscope z axis. In DPS.
    /// * ax The accelerometer x axis. In g.
    /// * ay The accelerometer y axis. In g.
    /// * az The accelerometer z axis. In g.
    /// * mx The magnetometer x axis. In uT.
    /// * my The magnetometer y axis. In uT.
    /// * mz The magnetometer z axis. In uT.
    pub fn update(
        &mut self,
        gx: f32,
        gy: f32,
        gz: f32,
        ax: f32,
        ay: f32,
        az: f32,
        mx: f32,
        my: f32,
        mz: f32,
    ) {
        let nxp = &mut self.nxp as *mut nxp::Nxp;
        unsafe {
            nxp::nxp_c_update(nxp, gx, gy, gz, ax, ay, az, mx, my, mz);
        }
    }

    pub fn freq(&self) -> f32 {
        self.freq
    }

    pub fn roll(&self) -> f32 {
        self.nxp.PhiPl
    }

    pub fn pitch(&self) -> f32 {
        self.nxp.ThePl
    }

    pub fn yaw(&self) -> f32 {
        self.nxp.PsiPl
    }

    /// Returns the quaternion `[w, x, y, z]`.
    pub fn quaternion(&self) -> [f32; 4] {
        [
            self.nxp.qPl.q0,
            self.nxp.qPl.q1,
            self.nxp.qPl.q2,
            self.nxp.qPl.q3,
        ]
    }

    /// Set the quaternion to `[w, x, y, z]`.
    pub fn set_quaternion(&mut self, q: [f32; 4]) {
        self.nxp.qPl.q0 = q[0];
        self.nxp.qPl.q1 = q[1];
        self.nxp.qPl.q2 = q[2];
        self.nxp.qPl.q3 = q[3];
    }

    /// Get the linear acceleration part of the acceleration value given to update (in `g`, `[x, y, z]`).
    pub fn linear_acceleration(&self) -> [f32; 3] {
        [self.nxp.aSePl[0], self.nxp.aSePl[1], self.nxp.aSePl[2]]
    }

    /// Get the gravity vector from the gyroscope values (in `g`, `[x, y, z]`).
    pub fn gravity_vector(&self) -> [f32; 3] {
        [
            self.nxp.gSeGyMi[0],
            self.nxp.gSeGyMi[1],
            self.nxp.gSeGyMi[2],
        ]
    }

    /// Get the geomagnetic vector in global frame (in `uT`, `[x, y, z]`).
    pub fn geomagnetic_vector(&self) -> [f32; 3] {
        [self.nxp.mGl[0], self.nxp.mGl[1], self.nxp.mGl[2]]
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

    #[test]
    fn set_and_get_quaternion() {
        let mut nxp = NxpFusion::new(150.);
        nxp.set_quaternion([0.5, 1., 2., 3.]);
        assert_eq!(nxp.quaternion(), [0.5, 1., 2., 3.]);
    }
}
