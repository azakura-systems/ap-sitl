mod msg;
mod packet;

use std::io::ErrorKind::WouldBlock;
use std::net::{SocketAddr, UdpSocket};

use anyhow::{Context, Result};
pub use msg::Msg;
use packet::RecvPacket;
pub use packet::{Pwm, Pwm16, Pwm32};

const SITL_LISTEN_PORT: &str = "9002";
const SEND_BUF_CAPACITY: usize = 512;

/// UDP bridge for ArduPilot's JSON SITL backend.
///
/// Binds to the JSON SITL input port, waits for the first PWM packet from
/// ArduPilot, then sends [`Msg`] state packets back to that same endpoint.
/// Servo packets can be drained with [`Self::receive_servos`].
pub struct Sitl<P: Pwm> {
    socket:    UdpSocket,
    sitl_addr: SocketAddr,
    recv_buf:  Vec<u8>,
    send_buf:  Vec<u8>,
    pwm:       RecvPacket<P>,
}

impl<P: Pwm> Sitl<P> {
    /// Connects to an ArduPilot JSON SITL instance on `addr`.
    ///
    /// Blocks until a valid PWM packet is received, then switches the socket to
    /// nonblocking mode for runtime servo polling.
    ///
    /// # Arguments
    /// * `addr` - Interface/address to bind on, usually `"127.0.0.1"`.
    pub fn connect(addr: &str) -> Result<Self> {
        let bind_addr = format!("{addr}:{SITL_LISTEN_PORT}");
        let socket =
            UdpSocket::bind(&bind_addr).context(format!("Failed to bind to {}", bind_addr))?;
        let pwm = RecvPacket::<P>::new();
        let mut recv_buf = vec![0u8; P::SIZE];
        let sitl_addr = loop {
            let (size, addr) = socket.recv_from(&mut recv_buf)?;
            if pwm.parse(&recv_buf[..size]).is_err() {
                continue;
            }
            break addr;
        };
        socket.set_nonblocking(true)?;
        Ok(Self {
            socket,
            sitl_addr,
            recv_buf,
            send_buf: Vec::with_capacity(SEND_BUF_CAPACITY),
            pwm,
        })
    }

    /// Sends one JSON state message to ArduPilot.
    ///
    /// # Note
    /// ArduPilot expects this to be called continuously at the SITL sensor
    /// update rate. Sending a single message and exiting will usually make AP
    /// report a link timeout.
    pub fn send(&mut self, msg: impl Into<Msg>) -> Result<()> {
        self.send_buf.clear();
        serde_json::to_writer(&mut self.send_buf, &msg.into())?;
        self.send_buf.push(b'\n');
        self.socket.send_to(&self.send_buf, self.sitl_addr)?;
        Ok(())
    }

    /// Drains pending servo packets and returns the latest normalized values.
    ///
    /// Returns `None` when no new packet is available. Values are normalized
    /// from PWM into roughly `[0, 1]`; unused channels are zero for PWM16.
    pub fn receive_servos(&mut self) -> Result<Option<[f64; 32]>> {
        let mut latest_servos = None;
        loop {
            match self.socket.recv_from(&mut self.recv_buf) {
                Ok((size, addr)) => {
                    if addr == self.sitl_addr && size == self.recv_buf.len() {
                        latest_servos = Some(self.pwm.servos(&self.recv_buf));
                    }
                },
                Err(e) if e.kind() == WouldBlock => {
                    return Ok(latest_servos);
                },
                Err(e) => return Err(e.into()),
            }
        }
    }
}
