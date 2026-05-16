use serde::Serialize;

/// JSON state message sent to ArduPilot SITL.
///
/// Stores the aircraft state and optional simulated sensor inputs using the
/// field names expected by ArduPilot's JSON backend.
///
/// # Frame conversion
/// If the source simulation uses the Sora right-handed frame, convert vectors
/// before constructing this message:
///
/// * gyro, accel, position, velocity: `[x, y, z] -> [y, x, -z]`
/// * quaternion: `[w, i, j, k] -> [w, j, i, -k]`
#[derive(Serialize)]
pub struct Msg {
    /// Simulation timestamp (seconds).
    timestamp:     f64,
    /// IMU measurements.
    imu:           Imu,
    /// Vehicle position in ArduPilot JSON frame (m).
    position:      [f64; 3],
    /// Vehicle velocity in ArduPilot JSON frame (m/s).
    velocity:      [f64; 3],
    /// Vehicle attitude quaternion in ArduPilot JSON order.
    quaternion:    [f64; 4],
    /// Rangefinder 1 distance (m).
    #[serde(skip_serializing_if = "Option::is_none")]
    rng_1:         Option<f64>,
    /// Rangefinder 2 distance (m).
    #[serde(skip_serializing_if = "Option::is_none")]
    rng_2:         Option<f64>,
    /// Rangefinder 3 distance (m).
    #[serde(skip_serializing_if = "Option::is_none")]
    rng_3:         Option<f64>,
    /// Rangefinder 4 distance (m).
    #[serde(skip_serializing_if = "Option::is_none")]
    rng_4:         Option<f64>,
    /// Rangefinder 5 distance (m).
    #[serde(skip_serializing_if = "Option::is_none")]
    rng_5:         Option<f64>,
    /// Rangefinder 6 distance (m).
    #[serde(skip_serializing_if = "Option::is_none")]
    rng_6:         Option<f64>,
    /// Optional windvane measurement.
    #[serde(skip_serializing_if = "Option::is_none")]
    windvane:      Option<Windvane>,
    /// Optional wind velocity vector in ArduPilot JSON frame (m/s).
    #[serde(skip_serializing_if = "Option::is_none")]
    velocity_wind: Option<[f64; 3]>,
    /// Optional airspeed measurement (m/s).
    #[serde(skip_serializing_if = "Option::is_none")]
    airspeed:      Option<f64>,
    /// Optional RC input channels (PWM).
    #[serde(skip_serializing_if = "Option::is_none")]
    rc:            Option<RcInput>,
    /// Optional battery measurement.
    #[serde(skip_serializing_if = "Option::is_none")]
    battery:       Option<Battery>,
}

#[derive(Serialize)]
struct Imu {
    /// Angular velocity in ArduPilot JSON frame (rad/s).
    gyro:       [f64; 3],
    /// Body-frame acceleration in ArduPilot JSON frame (m/s^2).
    accel_body: [f64; 3],
}

impl Imu {
    fn new(gyro: [f64; 3], acc: [f64; 3]) -> Self {
        Self { gyro, accel_body: acc }
    }
}

/// Windvane sensor measurement.
#[derive(Serialize)]
pub struct Windvane {
    /// Wind direction.
    direction: f64,
    /// Wind speed (m/s).
    speed:     f64,
}

impl Windvane {
    /// Creates a windvane measurement.
    ///
    /// # Arguments
    /// * `direction` - Wind direction in ArduPilot's expected convention.
    /// * `speed` - Wind speed (m/s).
    pub fn new(direction: f64, speed: f64) -> Self {
        Self { direction, speed }
    }
}

/// RC channel override packet.
///
/// Channels are serialized as `rc_1` through `rc_12` using raw PWM values.
#[derive(Serialize)]
pub struct RcInput {
    rc_1:  u16,
    rc_2:  u16,
    rc_3:  u16,
    rc_4:  u16,
    rc_5:  u16,
    rc_6:  u16,
    rc_7:  u16,
    rc_8:  u16,
    rc_9:  u16,
    rc_10: u16,
    rc_11: u16,
    rc_12: u16,
}

impl RcInput {
    /// Creates RC input from twelve PWM channels.
    pub fn new(channels: [u16; 12]) -> Self {
        Self {
            rc_1:  channels[0],
            rc_2:  channels[1],
            rc_3:  channels[2],
            rc_4:  channels[3],
            rc_5:  channels[4],
            rc_6:  channels[5],
            rc_7:  channels[6],
            rc_8:  channels[7],
            rc_9:  channels[8],
            rc_10: channels[9],
            rc_11: channels[10],
            rc_12: channels[11],
        }
    }
}

/// Battery sensor measurement.
#[derive(Serialize)]
pub struct Battery {
    /// Battery voltage (V).
    voltage: f64,
    /// Battery current (A).
    current: f64,
}

impl Battery {
    /// Creates a battery measurement.
    pub fn new(voltage: f64, current: f64) -> Self {
        Self { voltage, current }
    }
}

impl Msg {
    /// Creates a message with all required aircraft state fields.
    ///
    /// # Arguments
    /// * `timestamp` - Simulation timestamp (seconds).
    /// * `gyro` - Angular velocity in ArduPilot JSON frame (rad/s).
    /// * `accel_body` - Body-frame acceleration in ArduPilot JSON frame
    ///   (m/s^2).
    /// * `position` - Position in ArduPilot JSON frame (m).
    /// * `velocity` - Velocity in ArduPilot JSON frame (m/s).
    /// * `quaternion` - Attitude quaternion in ArduPilot JSON order.
    pub fn new(
        timestamp: f64,
        gyro: [f64; 3],
        accel_body: [f64; 3],
        position: [f64; 3],
        velocity: [f64; 3],
        quaternion: [f64; 4],
    ) -> Self {
        Self {
            timestamp,
            imu: Imu::new(gyro, accel_body),
            position,
            velocity,
            quaternion,
            rng_1: None,
            rng_2: None,
            rng_3: None,
            rng_4: None,
            rng_5: None,
            rng_6: None,
            windvane: None,
            velocity_wind: None,
            airspeed: None,
            rc: None,
            battery: None,
        }
    }

    /// Sets one rangefinder distance.
    ///
    /// # Arguments
    /// * `idx` - Rangefinder index, `1..=6`.
    /// * `meters` - Distance reading (m).
    ///
    /// # Panics
    /// Panics if `idx` is outside `1..=6`.
    pub fn with_rangefinder(mut self, idx: usize, meters: f64) -> Self {
        match idx {
            1 => self.rng_1 = Some(meters),
            2 => self.rng_2 = Some(meters),
            3 => self.rng_3 = Some(meters),
            4 => self.rng_4 = Some(meters),
            5 => self.rng_5 = Some(meters),
            6 => self.rng_6 = Some(meters),
            _ => panic!("rangefinder index must be 1..=6"),
        }
        self
    }

    /// Sets the windvane measurement.
    pub fn with_windvane(mut self, direction: f64, speed: f64) -> Self {
        self.windvane = Some(Windvane::new(direction, speed));
        self
    }

    /// Sets the wind velocity vector in ArduPilot JSON frame (m/s).
    pub fn with_wind_velocity(mut self, velocity: [f64; 3]) -> Self {
        self.velocity_wind = Some(velocity);
        self
    }

    /// Sets the airspeed measurement (m/s).
    pub fn with_airspeed(mut self, airspeed: f64) -> Self {
        self.airspeed = Some(airspeed);
        self
    }

    /// Sets twelve RC input channels as raw PWM values.
    pub fn with_rc(mut self, channels: [u16; 12]) -> Self {
        self.rc = Some(RcInput::new(channels));
        self
    }

    /// Sets the battery measurement.
    ///
    /// # Arguments
    /// * `voltage` - Battery voltage (V).
    /// * `current` - Battery current (A).
    pub fn with_battery(mut self, voltage: f64, current: f64) -> Self {
        self.battery = Some(Battery::new(voltage, current));
        self
    }
}
