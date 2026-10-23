use super::GBEmulator;

#[derive(Default)]
pub(super) struct Apu {
    powered: bool,
    enabled: [bool; 4],
    length: [u16; 4],
    length_enabled: [bool; 4],
    step: u8,
    sweep_frequency: u16,
    sweep_timer: u8,
    sweep_enabled: bool,
    sweep_negated: bool,
}

impl GBEmulator {
    fn apu_clock_length(&mut self, channel: usize) {
        if self.apu.length[channel] > 0 {
            self.apu.length[channel] -= 1;
            if self.apu.length[channel] == 0 { self.apu.enabled[channel] = false; }
        }
    }

    pub(super) fn apu_frame_tick(&mut self) {
        if !self.apu.powered { return; }
        if self.apu.step & 1 == 0 {
            for channel in 0..4 {
                if self.apu.length_enabled[channel] { self.apu_clock_length(channel); }
            }
        }
        if self.apu.step == 2 || self.apu.step == 6 {
            if self.apu.sweep_timer > 0 { self.apu.sweep_timer -= 1; }
            if self.apu.sweep_timer == 0 {
                let period = (self.mem[0xFF10] >> 4) & 7;
                self.apu.sweep_timer = if period == 0 { 8 } else { period };
                if self.apu.sweep_enabled && period != 0 {
                    let frequency = self.apu_sweep_calculate();
                    if frequency <= 2047 && self.mem[0xFF10] & 7 != 0 {
                        self.apu.sweep_frequency = frequency;
                        self.mem[0xFF13] = frequency as u8;
                        self.mem[0xFF14] = (self.mem[0xFF14] & !7) | (frequency >> 8) as u8;
                        self.apu_sweep_calculate();
                    }
                }
            }
        }
        self.apu.step = (self.apu.step + 1) & 7;
    }

    fn apu_sweep_calculate(&mut self) -> u16 {
        let delta = self.apu.sweep_frequency >> (self.mem[0xFF10] & 7);
        let frequency = if self.mem[0xFF10] & 8 != 0 {
            self.apu.sweep_negated = true;
            self.apu.sweep_frequency - delta
        } else {
            self.apu.sweep_frequency + delta
        };
        if frequency > 2047 { self.apu.enabled[0] = false; }
        frequency
    }

    pub(super) fn apu_read8(&self, addr: usize) -> u8 {
        const MASKS: [u8; 32] = [
            0x80, 0x3F, 0, 0xFF, 0xBF, 0xFF, 0x3F, 0, 0xFF, 0xBF,
            0x7F, 0xFF, 0x9F, 0xFF, 0xBF, 0xFF, 0xFF, 0, 0, 0xBF,
            0, 0, 0x70, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        ];
        if addr == 0xFF26 {
            let mut value = 0x70 | ((self.apu.powered as u8) << 7);
            for channel in 0..4 {
                value |= (self.apu.enabled[channel] as u8) << channel;
            }
            value
        } else if addr < 0xFF30 {
            self.mem[addr] | MASKS[addr - 0xFF10]
        } else {
            self.mem[addr]
        }
    }

    pub(super) fn apu_write8(&mut self, addr: usize, value: u8) {
        if addr >= 0xFF30 {
            self.mem[addr] = value;
            return;
        }
        if addr == 0xFF26 {
            let powered = value & 0x80 != 0;
            if self.apu.powered && !powered {
                self.mem[0xFF10..0xFF26].fill(0);
                self.apu.enabled = [false; 4];
                self.apu.length_enabled = [false; 4];
            }
            if !self.apu.powered && powered {
                self.apu.step = 0;
            }
            self.apu.powered = powered;
            return;
        }
        let length_channel = match addr {
            0xFF11 => Some(0), 0xFF16 => Some(1),
            0xFF1B => Some(2), 0xFF20 => Some(3), _ => None,
        };
        if let Some(channel) = length_channel {
            let maximum = if channel == 2 { 256 } else { 64 };
            self.apu.length[channel] = maximum - (value as u16 & (maximum - 1));
        }
        if !self.apu.powered {
            return;
        }
        if addr == 0xFF10 && self.apu.sweep_negated && value & 8 == 0 {
            self.apu.enabled[0] = false;
        }
        self.mem[addr] = value;
        let dac_channel = match addr {
            0xFF12 => Some(0), 0xFF17 => Some(1),
            0xFF1A => Some(2), 0xFF21 => Some(3), _ => None,
        };
        if let Some(channel) = dac_channel {
            let mask = if channel == 2 { 0x80 } else { 0xF8 };
            if value & mask == 0 { self.apu.enabled[channel] = false; }
        }
        let trigger_channel = match addr {
            0xFF14 => Some(0), 0xFF19 => Some(1),
            0xFF1E => Some(2), 0xFF23 => Some(3), _ => None,
        };
        if let Some(channel) = trigger_channel {
            let enabled = value & 0x40 != 0;
            let extra_clock = self.apu.step & 1 != 0;
            if enabled && !self.apu.length_enabled[channel] && extra_clock {
                self.apu_clock_length(channel);
            }
            self.apu.length_enabled[channel] = enabled;
            if value & 0x80 != 0 {
                let dac_addr = [0xFF12, 0xFF17, 0xFF1A, 0xFF21][channel];
                let mask = if channel == 2 { 0x80 } else { 0xF8 };
                self.apu.enabled[channel] = self.mem[dac_addr] & mask != 0;
                if channel == 0 {
                    self.apu.sweep_frequency = self.mem[0xFF13] as u16 | ((value as u16 & 7) << 8);
                    let period = (self.mem[0xFF10] >> 4) & 7;
                    self.apu.sweep_timer = if period == 0 { 8 } else { period };
                    self.apu.sweep_enabled = self.mem[0xFF10] & 0x77 != 0;
                    self.apu.sweep_negated = false;
                    if self.mem[0xFF10] & 7 != 0 { self.apu_sweep_calculate(); }
                }
                if self.apu.length[channel] == 0 {
                    self.apu.length[channel] = if channel == 2 { 256 } else { 64 };
                    if enabled && extra_clock { self.apu_clock_length(channel); }
                }
            }
        }
    }
}
