use std::marker::PhantomData;

use anyhow::{Result, anyhow};

const PWM_MIN: u16 = 1000;
const PWM_CONVERT: f64 = 1000.0;

/// ArduPilot PWM packet layout.
pub trait Pwm {
    /// Packet size in bytes.
    const SIZE: usize;
    /// Packet magic value.
    const MAGIC: u16;
}

/// ArduPilot 16-channel PWM packet layout.
pub struct Pwm16;

impl Pwm for Pwm16 {
    const SIZE: usize = 40;
    const MAGIC: u16 = 18458;
}

/// ArduPilot 32-channel PWM packet layout.
pub struct Pwm32;

impl Pwm for Pwm32 {
    const SIZE: usize = 72;
    const MAGIC: u16 = 29569;
}

pub(super) struct RecvPacket<P> {
    _pwm: PhantomData<P>,
}

impl<P: Pwm> RecvPacket<P> {
    pub(super) fn new() -> Self {
        Self { _pwm: PhantomData }
    }

    pub(super) fn parse(&self, bytes: &[u8]) -> Result<()> {
        if bytes.len() != P::SIZE {
            return Err(anyhow!(
                "Invalid packet size: expected {}, got {}",
                P::SIZE,
                bytes.len()
            ));
        }
        let packet_magic = u16::from_le_bytes([bytes[0], bytes[1]]);
        if packet_magic != P::MAGIC {
            return Err(anyhow!(
                "Invalid magic: expected {}, got {packet_magic}",
                P::MAGIC
            ));
        }
        Ok(())
    }

    pub(super) fn servos(&self, bytes: &[u8]) -> [f64; 32] {
        let mut servos = [0.0; 32];
        for (servo, pwm) in servos.iter_mut().zip(bytes[8..P::SIZE].chunks_exact(2)) {
            let val = u16::from_le_bytes([pwm[0], pwm[1]]);
            *servo = val.saturating_sub(PWM_MIN) as f64 / PWM_CONVERT;
        }
        servos
    }
}
