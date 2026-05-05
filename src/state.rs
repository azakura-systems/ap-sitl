use nalgebra::{UnitQuaternion, Vector3};
use sdk::{Imu, Rb};
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct SitlState {
    timestamp:  f64,
    imu:        SitlImu,
    position:   Vec3Json,
    velocity:   Vec3Json,
    quaternion: QuatJson,
}

#[derive(Serialize)]
struct SitlImu {
    gyro:       Vec3Json,
    accel_body: Vec3Json,
}

impl SitlImu {
    fn new(gyro: Vector3<f64>, acc: Vector3<f64>) -> Self {
        Self { gyro: vec3_to_sitl(gyro), accel_body: vec3_to_sitl(acc) }
    }
}

impl SitlState {
    pub fn new(timestamp: f64, imu: &Imu, rb: &Rb) -> Self {
        Self {
            timestamp,
            imu: SitlImu::new(imu.w_b(), imu.a_b()),
            position: vec3_to_sitl(rb.p_wb()),
            velocity: vec3_to_sitl(rb.v_w()),
            quaternion: quat_to_sitl(rb.q_wb()),
        }
    }
}

type Vec3Json = [f64; 3];
type QuatJson = [f64; 4];

#[inline(always)]
fn vec3_to_sitl(v: Vector3<f64>) -> Vec3Json {
    [v.y, v.x, -v.z]
}

#[inline(always)]
fn quat_to_sitl(q: UnitQuaternion<f64>) -> QuatJson {
    [q.w, q.j, q.i, -q.k]
}
