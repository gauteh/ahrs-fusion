# include "wrapper.h"
# include "Adafruit_AHRS_NXPFusion.h"

void * create(float sample_freq) {
  Adafruit_NXPSensorFusion fusion;
  fusion.begin(sample_freq);

  return &fusion;
}
