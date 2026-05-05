pub(super) struct ServoPacket<'a> {
    pub(super) magic:        u16,
    /// AP param: SIM_RATE_HZ
    pub(super) frame_rate:   u16,
    pub(super) _frame_count: u32,
    pwm_bytes:               &'a [u8],
}

impl<'a> ServoPacket<'a> {
    pub(super) fn from_bytes(bytes: &'a [u8]) -> Self {
        Self {
            magic:        u16::from_le_bytes([bytes[0], bytes[1]]),
            frame_rate:   u16::from_le_bytes([bytes[2], bytes[3]]),
            _frame_count: u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
            pwm_bytes:    &bytes[8..],
        }
    }

    #[inline(always)]
    pub(super) fn pwm(&self, idx: usize) -> u16 {
        let offset = idx * 2;
        u16::from_le_bytes([self.pwm_bytes[offset], self.pwm_bytes[offset + 1]])
    }
}
