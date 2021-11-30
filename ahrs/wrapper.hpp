# include "nxp.hpp"

Nxp nxp_c_begin(float sampleFrequency);
void nxp_c_update(Nxp *nxp, float gx, float gy, float gz, float ax, float ay,
                float az, float mx, float my, float mz);
