use anyhow::{Result, anyhow};

const PWM_MIN: u16 = 1000;
const PWM_CONVERT: f64 = 1000.0;

const PWM_16_SIZE: usize = 40;
const PWM_32_SIZE: usize = 72;

const PWM_16_MAGIC: u16 = 18458;
const PWM_32_MAGIC: u16 = 29569;

pub(super) struct Pwm {
    pub(super) size: usize,
    magic:           u16,
}

impl Pwm {
    pub(super) fn new(ver: usize) -> Result<Self> {
        match ver {
            16 => Ok(Self { size: PWM_16_SIZE, magic: PWM_16_MAGIC }),
            32 => Ok(Self { size: PWM_32_SIZE, magic: PWM_32_MAGIC }),
            _ => Err(anyhow!("Invalid pwm version: {ver}, expected 16 or 32")),
        }
    }

    pub(super) fn parse(&self, bytes: &[u8]) -> Result<()> {
        if bytes.len() != self.size {
            return Err(anyhow!("Invalid packet size: expected {}, got {}", self.size, bytes.len()));
        }
        let packet_magic = u16::from_le_bytes([bytes[0], bytes[1]]);
        if packet_magic != self.magic {
            return Err(anyhow!("Invalid magic: expected {}, got {packet_magic}", self.magic));
        }
        Ok(())
    }

    pub(super) fn servos(&self, bytes: &[u8]) -> Vec<f64> {
        bytes[8..self.size]
            .chunks_exact(2)
            .map(|pwm| {
                let val = u16::from_le_bytes([pwm[0], pwm[1]]);
                val.saturating_sub(PWM_MIN) as f64 / PWM_CONVERT
            })
            .collect()
    }
}
