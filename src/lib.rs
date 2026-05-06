mod packet;
mod state;

use std::{
    io::ErrorKind::WouldBlock,
    net::{SocketAddr, UdpSocket},
};

use anyhow::{Context, Result};

use packet::Pwm;
use state::SitlState;

const SITL_LISTEN_PORT: &str = "9002";
const SEND_BUF_CAPACITY: usize = 512;

pub struct Ap {
    socket:    UdpSocket,
    sitl_addr: SocketAddr,
    recv_buf:  Vec<u8>,
    send_buf:  Vec<u8>,
    pwm:       Pwm,
}

impl Ap {
    pub fn connect(addr: &str, pwm_ver: usize) -> Result<Self> {
        let bind_addr = format!("{addr}:{SITL_LISTEN_PORT}");
        let socket = UdpSocket::bind(&bind_addr).context(format!("Failed to bind to {}", bind_addr))?;
        let pwm = Pwm::new(pwm_ver)?;
        let mut recv_buf = vec![0u8; pwm.size];
        let sitl_addr = loop {
            let (size, addr) = socket.recv_from(&mut recv_buf)?;
            if pwm.parse(&recv_buf[..size]).is_err() {
                continue;
            }
            break addr;
        };
        socket.set_nonblocking(true)?;
        Ok(Self { socket, sitl_addr, recv_buf, send_buf: Vec::with_capacity(SEND_BUF_CAPACITY), pwm })
    }

    /**
    gyro, acc, pos, vel order: y, x, -z

    quat order: w, j, i, -k
    */
    pub fn sync(
        &mut self,
        timestamp: f64,
        gyro: [f64; 3],
        accel_body: [f64; 3],
        position: [f64; 3],
        velocity: [f64; 3],
        quaternion: [f64; 4],
    ) -> Result<()> {
        self.send_buf.clear();
        serde_json::to_writer(
            &mut self.send_buf,
            &SitlState::new(timestamp, gyro, accel_body, position, velocity, quaternion),
        )?;
        self.send_buf.push(b'\n');
        self.socket.send_to(&self.send_buf, self.sitl_addr)?;
        Ok(())
    }

    pub fn receive_servos(&mut self) -> Result<Option<Vec<f64>>> {
        let mut latest_servos = None;
        loop {
            match self.socket.recv_from(&mut self.recv_buf) {
                Ok((size, addr)) => {
                    if addr == self.sitl_addr && size == self.recv_buf.len() {
                        latest_servos = Some(self.pwm.servos(&self.recv_buf));
                    }
                }
                Err(e) if e.kind() == WouldBlock => {
                    return Ok(latest_servos);
                }
                Err(e) => return Err(e.into()),
            }
        }
    }
}
