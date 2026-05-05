mod bridge;
mod cfg;
mod packet;
mod state;

use std::path::Path;

use anyhow::{Ok, Result};

use bridge::SitlBridge;
use sdk::{ActCmds, Imu, Rb};
use state::SitlState;

pub struct Ap {
    bridge:      SitlBridge,
    timestamp:   f64,
    pub t_accum: f64,
    state_accum: f64,
    prev_cmds:   ActCmds,
}

impl Ap {
    pub fn connect(addr: &str, cfg_path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self {
            bridge:      SitlBridge::new(addr, cfg_path)?,
            timestamp:   0.0,
            t_accum:     0.0,
            state_accum: 0.0,
            prev_cmds:   ActCmds::default(),
        })
    }

    pub fn servo(&mut self) -> Result<ActCmds> {
        if let Some(cmds) = self.bridge.receive_servo()? {
            self.prev_cmds = cmds;
        }
        Ok(self.prev_cmds)
    }

    pub fn sync(&mut self, quad_dt: f64, imu: &Imu, rb: &Rb) -> Result<()> {
        self.state_accum += quad_dt;
        if self.state_accum >= self.bridge.frame_dt {
            self.bridge.send_state(SitlState::new(self.timestamp, imu, rb))?;
            self.state_accum = self.state_accum.rem_euclid(self.bridge.frame_dt);
        }
        self.timestamp += quad_dt;
        Ok(())
    }

    pub fn sim_rate_hz(&self) -> f64 {
        1.0 / self.bridge.frame_dt
    }
}
