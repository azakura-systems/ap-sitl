use anyhow::Result;
use ap_sitl::{Bridge, Msg};
use rt_timer::Timer;

fn main() -> Result<()> {
    let mut ap = Bridge::connect("127.0.0.1", 16)?;
    let mut timer = Timer::from_hz(20.0);

    loop {
        ap.send(
            Msg::new(timer.elapsed().as_secs_f64()).with_state(
                [0.0, 0.0, 0.0],
                [0.0, 0.0, -9.81],
                [0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0, 0.0],
            ), // .with_rangefinder(1, 2.4)
               // .with_windvane(0.0, 3.2)
               // .with_wind_velocity([3.2, 0.0, -0.7])
               // .with_airspeed(12.0)
               // .with_battery(16.2, 22.0);
        )?;

        timer.wait();
    }
}
