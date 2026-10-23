use super::GBEmulator;

#[derive(Default)]
pub(super) struct Apu {
    powered: bool,
    enabled: [bool; 4],
    length: [u16; 4],
    length_enabled: [bool; 4],
    step: u8,
}

impl GBEmulator {
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
        if !self.apu.powered {
            return;
        }
        self.mem[addr] = value;
    }
}
