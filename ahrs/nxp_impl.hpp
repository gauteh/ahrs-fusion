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

/* float getRoll() { return PhiPl; } */
/* float getPitch() { return ThePl; } */
/* float getYaw() { return PsiPl; } */

/* void getQuaternion(float *w, float *x, float *y, float *z) { */
/*   *w = qPl.q0; */
/*   *x = qPl.q1; */
/*   *y = qPl.q2; */
/*   *z = qPl.q3; */
/* } */

/* void setQuaternion(float w, float x, float y, float z) { */
/*   qPl.q0 = w; */
/*   qPl.q1 = x; */
/*   qPl.q2 = y; */
/*   qPl.q3 = z; */
/* } */
/**************************************************************************/
/*!
 * @brief Get the linear acceleration part of the acceleration value given to
 * update.
 *
 * @param x The pointer to write the linear acceleration x axis to. In g.
 * @param y The pointer to write the linear acceleration y axis to. In g.
 * @param z The pointer to write the linear acceleration z axis to. In g.
 */
/**************************************************************************/
/* void getLinearAcceleration(float *x, float *y, float *z) const { */
/*   *x = aSePl[0]; */
/*   *y = aSePl[1]; */
/*   *z = aSePl[2]; */
/* } */

/**************************************************************************/
/*!
 * @brief Get the gravity vector from the gyroscope values.
 *
 * @param x The pointer to write the gravity vector x axis to. In g.
 * @param y The pointer to write the gravity vector y axis to. In g.
 * @param z The pointer to write the gravity vector z axis to. In g.
 */
/**************************************************************************/
/* void getGravityVector(float *x, float *y, float *z) const { */
/*   *x = gSeGyMi[0]; */
/*   *y = gSeGyMi[1]; */
/*   *z = gSeGyMi[2]; */
/* } */

/**************************************************************************/
/*!
 * @brief Get the geomagnetic vector in global frame.
 *
 * @param x The pointer to write the geomagnetic vector x axis to. In uT.
 * @param y The pointer to write the geomagnetic vector y axis to. In uT.
 * @param z The pointer to write the geomagnetic vector z axis to. In uT.
 */
/**************************************************************************/
/* void getGeomagneticVector(float *x, float *y, float *z) const { */
/*   *x = mGl[0]; */
/*   *y = mGl[1]; */
/*   *z = mGl[2]; */
/* } */

