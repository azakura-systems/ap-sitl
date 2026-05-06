use serde::Serialize;

/**
imu, pos, vel order: y, x, -z

quat order: w, j, i, -k
*/
#[derive(Serialize)]
pub struct SitlState {
    timestamp:  f64,
    imu:        SitlImu,
    position:   [f64; 3],
    velocity:   [f64; 3],
    quaternion: [f64; 4],
}

#[derive(Serialize)]
struct SitlImu {
    gyro:       [f64; 3],
    accel_body: [f64; 3],
}

impl SitlImu {
    fn new(gyro: [f64; 3], acc: [f64; 3]) -> Self {
        Self { gyro, accel_body: acc }
    }
}

impl SitlState {
    pub fn new(
        timestamp: f64,
        gyro: [f64; 3],
        accel_body: [f64; 3],
        position: [f64; 3],
        velocity: [f64; 3],
        quaternion: [f64; 4],
    ) -> Self {
        Self { timestamp, imu: SitlImu::new(gyro, accel_body), position, velocity, quaternion }
    }
}
