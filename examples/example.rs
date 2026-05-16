use anyhow::Result;
use ap_sitl::{Msg, Pwm16, Sitl};
use rt_timer::Timer;

fn main() -> Result<()> {
    let mut ap = Sitl::<Pwm16>::connect("127.0.0.1")?;

    // ArduPilot's 400 Hz loop expects physics updates at least 1.8x faster.
    let mut timer = Timer::from_hz(720.0);

    loop {
        if let Some(servos) = ap.receive_servos()? {
            println!(
                "servos: {}, {}, {}, {}",
                servos[0], servos[1], servos[2], servos[3]
            );
        };

        let timestamp = timer.elapsed().as_secs_f64();
        let gyro = [0.0, 0.0, 0.0];
        let accel_body = [0.0, 0.0, -9.81];
        let position = [0.0, 0.0, 0.0];
        let velocity = [0.0, 0.0, 0.0];
        let quaternion = [1.0, 0.0, 0.0, 0.0];

        ap.send(
            Msg::new(timestamp, gyro, accel_body, position, velocity, quaternion),
            // .with_rangefinder(1, 2.4)
            // .with_windvane(0.0, 3.2)
            // .with_wind_velocity([3.2, 0.0, -0.7])
            // .with_airspeed(12.0)
            // .with_battery(16.2, 22.0);
        )?;

        timer.wait();
    }
}
