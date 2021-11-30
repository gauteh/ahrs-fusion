# include "nxp.hpp"
# include "nxp_impl.hpp"

Nxp nxp_c_begin(float sampleFrequency) {
  return nxp_begin(sampleFrequency);
}
void nxp_c_update(Nxp *nxp, float gx, float gy, float gz, float ax, float ay,
                float az, float mx, float my, float mz) {
  nxp_update(nxp, gx, gy, gz, ax, ay, az, mx, my, mz);
}
