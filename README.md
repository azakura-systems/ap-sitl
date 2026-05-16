# ap_sitl

`ap_sitl` is a small Rust bridge for ArduPilot's JSON SITL backend. It binds to
the JSON SITL UDP port, receives PWM servo packets from ArduPilot, and sends
JSON aircraft state messages back to the simulator endpoint.

## Install

```toml
[dependencies]
ap_sitl = "0.1"
```

## Example

```rust,no_run
use anyhow::Result;
use ap_sitl::{Msg, Pwm16, Sitl};

fn main() -> Result<()> {
    let mut ap = Sitl::<Pwm16>::connect("127.0.0.1")?;

    let msg = Msg::new(
        0.0,
        [0.0, 0.0, 0.0],
        [0.0, 0.0, -9.81],
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0, 0.0],
    );

    ap.send(msg)?;

    if let Some(servos) = ap.receive_servos()? {
        println!("first servo: {}", servos[0]);
    }

    Ok(())
}
```

See `examples/example.rs` for a continuous update loop.

## License

Licensed under the MIT license. See `LICENSE`.
