use std::{fs::read_to_string, path::Path};

use anyhow::{Result, anyhow};
use serde::Deserialize;

const PWM_16_SIZE: usize = 40;
const PWM_32_SIZE: usize = 72;

const PWM_16_MAGIC: u16 = 18458;
const PWM_32_MAGIC: u16 = 29569;

const SERVO_NAMES: [&str; 4] = ["fl", "fr", "rl", "rr"];

#[derive(Deserialize)]
pub struct ServoMapping {
    pub fl: usize,
    pub fr: usize,
    pub rl: usize,
    pub rr: usize,
}

impl ServoMapping {
    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = usize> {
        [self.fl, self.fr, self.rl, self.rr].into_iter()
    }
}

#[derive(Deserialize)]
struct SitlCfg {
    pwm_ver: usize,
    servos:  ServoMapping,
}

pub struct ServoPktCfg {
    pub magic:         u16,
    pub pwm_size:      usize,
    pub servo_mapping: ServoMapping,
}

impl ServoPktCfg {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let sitl_cfg: SitlCfg = toml::from_str(&read_to_string(path)?)?;
        let (magic, pwm_size, chnl_idx) = match sitl_cfg.pwm_ver {
            16 => Ok((PWM_16_MAGIC, PWM_16_SIZE, 15)),
            32 => Ok((PWM_32_MAGIC, PWM_32_SIZE, 31)),
            _ => Err(anyhow!("Invalid pwm_ver: {}, expected 16 or 32", sitl_cfg.pwm_ver)),
        }?;
        for (idx, val) in sitl_cfg.servos.iter().enumerate() {
            if val > chnl_idx {
                Err(anyhow!(
                    "Invalid value at {} in config, found: {val}. Allowed range: 0 - {} for pwm_ver: {}",
                    SERVO_NAMES[idx],
                    chnl_idx,
                    sitl_cfg.pwm_ver
                ))?
            }
        }
        Ok(Self { magic, pwm_size, servo_mapping: sitl_cfg.servos })
    }
}
