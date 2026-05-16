use anyhow::Result;
use ap_sitl::{Bridge, Msg};
use rt_timer::Timer;

fn main() -> Result<()> {
    let mut ap = Bridge::connect("127.0.0.1", 16)?;
    let mut timer = Timer::from_hz(720.0);

    loop {
        let _ = ap.receive_servos()?;

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
