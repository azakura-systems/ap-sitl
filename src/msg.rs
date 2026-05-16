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
    /// `timestamp (s)` physics time.
    timestamp:     f64,
    /// `imu` measurement group.
    imu:           Imu,
    /// `position(north, east, down) (m)` earth frame.
    position:      [f64; 3],
    /// `velocity(north, east, down) (m/s)` earth frame.
    velocity:      [f64; 3],
    /// `quaternion(q1, q2, q3, q4)`.
    quaternion:    [f64; 4],
    /// `rng_1 (m)` rangefinder distance for driver instance 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    rng_1:         Option<f64>,
    /// `rng_2 (m)` rangefinder distance for driver instance 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    rng_2:         Option<f64>,
    /// `rng_3 (m)` rangefinder distance for driver instance 3.
    #[serde(skip_serializing_if = "Option::is_none")]
    rng_3:         Option<f64>,
    /// `rng_4 (m)` rangefinder distance for driver instance 4.
    #[serde(skip_serializing_if = "Option::is_none")]
    rng_4:         Option<f64>,
    /// `rng_5 (m)` rangefinder distance for driver instance 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    rng_5:         Option<f64>,
    /// `rng_6 (m)` rangefinder distance for driver instance 6.
    #[serde(skip_serializing_if = "Option::is_none")]
    rng_6:         Option<f64>,
    /// `windvane` apparent wind measurement.
    #[serde(skip_serializing_if = "Option::is_none")]
    windvane:      Option<Windvane>,
    /// `velocity_wind` 3D wind vector in m/s NED frame.
    #[serde(skip_serializing_if = "Option::is_none")]
    velocity_wind: Option<[f64; 3]>,
    /// `airspeed (m/s)`.
    #[serde(skip_serializing_if = "Option::is_none")]
    airspeed:      Option<f64>,
    /// `rc` optional R/C input data, up to 12 channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    rc:            Option<RcInput>,
    /// `battery` voltage and current measurement.
    #[serde(skip_serializing_if = "Option::is_none")]
    battery:       Option<Battery>,
}

#[derive(Serialize)]
struct Imu {
    /// `gyro(roll, pitch, yaw) (radians/sec)` body frame.
    gyro:       [f64; 3],
    /// `accel_body(x, y, z) (m/s^2)` body frame.
    accel_body: [f64; 3],
}

impl Imu {
    fn new(gyro: [f64; 3], acc: [f64; 3]) -> Self {
        Self {
            gyro,
            accel_body: acc,
        }
    }
}

/// Windvane sensor measurement.
#[derive(Serialize)]
pub struct Windvane {
    /// `direction (radians)` clockwise relative to the front, 0 = head to
    /// wind.
    direction: f64,
    /// `speed (m/s)`.
    speed:     f64,
}

impl Windvane {
    /// Creates a windvane measurement.
    ///
    /// # Arguments
    /// * `direction` (radians) - clockwise relative to the front, 0 = head to
    ///   wind.
    /// * `speed` - speed of wind in m/s.
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
    /// `voltage` in Volts.
    voltage: f64,
    /// `current` in Amps.
    current: f64,
}

impl Battery {
    /// Creates a battery measurement.
    pub fn new(voltage: f64, current: f64) -> Self {
        Self { voltage, current }
    }
}

impl Msg {
    /// Creates a JSON input frame with all required fields.
    ///
    /// # Arguments
    /// * `timestamp` - `(s)` physics time.
    /// * `gyro` - `(roll, pitch, yaw) (radians/sec)` body frame.
    /// * `accel_body` - `(x, y, z) (m/s^2)` body frame.
    /// * `position` - `(north, east, down) (m)` earth frame.
    /// * `velocity` - `(north, east, down) (m/s)` earth frame.
    /// * `quaternion` - `(q1, q2, q3, q4)`.
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
    /// * `idx` - Driver instance index, mapping to `rng_1` through `rng_6`.
    /// * `meters` - rangefinder distance.
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

    /// Sets apparent wind data in the `windvane` field.
    ///
    /// # Arguments
    /// * `direction` - `(radians)` clockwise relative to the front, 0 = head to
    ///   wind.
    /// * `speed` - `(m/s)`.
    pub fn with_windvane(mut self, direction: f64, speed: f64) -> Self {
        self.windvane = Some(Windvane::new(direction, speed));
        self
    }

    /// Sets 3D wind in `velocity_wind`.
    ///
    /// # Arguments
    /// * `velocity` - 3D wind vector in m/s NED frame.
    pub fn with_wind_velocity(mut self, velocity: [f64; 3]) -> Self {
        self.velocity_wind = Some(velocity);
        self
    }

    /// Sets `airspeed (m/s)`.
    pub fn with_airspeed(mut self, airspeed: f64) -> Self {
        self.airspeed = Some(airspeed);
        self
    }

    /// Sets optional R/C input data.
    ///
    /// # Arguments
    /// * `channels` - Raw PWM values serialized as `rc_1` through `rc_12`.
    pub fn with_rc(mut self, channels: [u16; 12]) -> Self {
        self.rc = Some(RcInput::new(channels));
        self
    }

    /// Sets the battery measurement.
    ///
    /// # Arguments
    /// * `voltage` - Battery voltage in Volts.
    /// * `current` - Battery current in Amps.
    pub fn with_battery(mut self, voltage: f64, current: f64) -> Self {
        self.battery = Some(Battery::new(voltage, current));
        self
    }
}
