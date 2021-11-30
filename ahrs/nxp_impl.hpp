# pragma once

# include "nxp.hpp"

/**************************************************************************/
/*!
 * @brief Initializes the 9DOF Kalman filter.
 *
 * @param sampleFrequency The sensor sample rate in herz(samples per second).
 */
/**************************************************************************/
Nxp nxp_begin(float sampleFrequency = 100.0f);

/*!
 * @brief Kalman/NXP Fusion algorithm.
 */

/**************************************************************************/
/*!
 * @brief Updates the filter with new gyroscope, accelerometer, and
 * magnetometer data. For roll, pitch, and yaw the accelerometer values can be
 * either m/s^2 or g, but for linear acceleration they have to be in g.
 *
 * 9DOF orientation function implemented using a 12 element Kalman filter
 *
 * void fRun_9DOF_GBY_KALMAN(SV_9DOF_GBY_KALMAN_t *SV,
 * const AccelSensor_t *Accel, const MagSensor_t *Mag,
 * const GyroSensor_t *Gyro, const MagCalibration_t *MagCal)
 *
 * @param gx The gyroscope x axis. In DPS.
 * @param gy The gyroscope y axis. In DPS.
 * @param gz The gyroscope z axis. In DPS.
 * @param ax The accelerometer x axis. In g.
 * @param ay The accelerometer y axis. In g.
 * @param az The accelerometer z axis. In g.
 * @param mx The magnetometer x axis. In uT.
 * @param my The magnetometer y axis. In uT.
 * @param mz The magnetometer z axis. In uT.
 */
/**************************************************************************/
void nxp_update(Nxp *nxp, float gx, float gy, float gz, float ax, float ay,
                float az, float mx, float my, float mz);


