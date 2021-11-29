/* C wrapper for Fusion lib */

struct Fusion {
}

void * create(float sample_freq);

void update(void * fusion, float gx, float gy, float gz, float ax, float ay, float az,
              float mx, float my, float mz);
