# AHRS Fusion

Port of Adafruit NXP sensor fusion algorithms based on Kalman filters for rust.

## Linking

This library is `no_std`, and e.g. on ARM you need to provide implementations
for `sinf(..)` and the likes. The easiest way to do that is to just extern link
to e.g.: [`cmsis_dsp`](https://github.com/samcrow/cmsis_dsp.rs#basic-c-math-functions)
(see this link for more explanation) and selecting one of the implementations to
provide the math functions.

To use `micromath`:

```sh
$ cargo add cmsis_dsp --features micromath
```

if you are not using the cmsis_dsp library, also add the following to your
crate:

```rust
extern crate cmsis_dsp;
```

## Using from the command-line

There is also a small command line utility that takes as its argument the
frequency, and reads accelerometer and gyroscope from stdin as CSV. It outputs
the rotated acceleration for every line. It can be built or installed by
enabling the `build-bin` feature flag:

```
$ cargo install --feature build-bin --path .
$ echo 1,2,3,4,5,6 | ahrs-csv 10
```

## Resources

* https://github.com/adafruit/Adafruit_AHRS
* https://www.nxp.com/docs/en/user-guide/NSFK_Prod_UG.pdf
* https://github.com/PaulStoffregen/NXPMotionSense
* https://github.com/memsindustrygroup/Open-Source-Sensor-Fusion
* https://se.mathworks.com/help/fusion/ref/ahrsfilter-system-object.html
