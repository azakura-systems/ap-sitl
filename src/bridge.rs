use std::{
    io::ErrorKind::WouldBlock,
    net::{SocketAddr, UdpSocket},
    path::Path,
};

use anyhow::{Context, Result, anyhow};
use sdk::ActCmds;

use super::{cfg::ServoPktCfg, packet::ServoPacket, state::SitlState};

const SITL_LISTEN_PORT: &str = "9002";
const SEND_BUF_CAPACITY: usize = 512;
const PWM_MIN: u16 = 1000;
const PWM_RANGE: u16 = 1000;

pub struct SitlBridge {
    socket:              UdpSocket,
    sitl_addr:           SocketAddr,
    recv_buf:            Vec<u8>,
    send_buf:            Vec<u8>,
    config:              ServoPktCfg,
    pub(super) frame_dt: f64,
}

impl SitlBridge {
    pub fn new(addr: &str, cfg_path: impl AsRef<Path>) -> Result<Self> {
        let config = ServoPktCfg::from_path(cfg_path)?;
        let bind_addr = format!("{addr}:{SITL_LISTEN_PORT}");
        let socket = UdpSocket::bind(&bind_addr).context(format!("Failed to bind to {}", bind_addr))?;
        let mut recv_buf = vec![0u8; config.pwm_size];
        let (sitl_addr, frame_dt) = loop {
            let (size, addr) = socket.recv_from(&mut recv_buf)?;
            if size != recv_buf.len() {
                continue;
            }
            let packet = ServoPacket::from_bytes(&recv_buf);
            if packet.magic != config.magic {
                continue;
            }
            break (addr, 1.0 / f64::from(packet.frame_rate.max(1)));
        };
        socket.set_nonblocking(true)?;
        Ok(Self { socket, sitl_addr, recv_buf, send_buf: Vec::with_capacity(SEND_BUF_CAPACITY), config, frame_dt })
    }

    pub fn receive_servo(&mut self) -> Result<Option<ActCmds>> {
        let mut latest_cmds = None;
        loop {
            match self.socket.recv_from(&mut self.recv_buf) {
                Ok((size, addr)) => {
                    if addr == self.sitl_addr {
                        if let Ok(cmds) = self.parse_packet(size) {
                            latest_cmds = Some(cmds);
                        }
                    }
                }
                Err(e) if e.kind() == WouldBlock => {
                    return Ok(latest_cmds);
                }
                Err(e) => return Err(e.into()),
            }
        }
    }

    fn parse_packet(&mut self, size: usize) -> Result<ActCmds> {
        if size != self.recv_buf.len() {
            return Err(anyhow!("Invalid packet size: expected {}, got {}", self.recv_buf.len(), size));
        }
        let packet = ServoPacket::from_bytes(&self.recv_buf);
        if packet.magic != self.config.magic {
            return Err(anyhow!("Invalid magic: expected {}, got {}", self.config.magic, packet.magic));
        }
        self.frame_dt = 1.0 / f64::from(packet.frame_rate);
        let pwm_to_f64 = |val: u16| (val.saturating_sub(PWM_MIN) as f64) / (PWM_RANGE as f64);
        Ok(ActCmds {
            fl: pwm_to_f64(packet.pwm(self.config.servo_mapping.fl)),
            fr: pwm_to_f64(packet.pwm(self.config.servo_mapping.fr)),
            rl: pwm_to_f64(packet.pwm(self.config.servo_mapping.rl)),
            rr: pwm_to_f64(packet.pwm(self.config.servo_mapping.rr)),
        })
    }

    pub fn send_state(&mut self, state: SitlState) -> Result<()> {
        self.send_buf.clear();
        serde_json::to_writer(&mut self.send_buf, &state)?;
        self.send_buf.push(b'\n');
        self.socket.send_to(&self.send_buf, self.sitl_addr)?;
        Ok(())
    }
}
