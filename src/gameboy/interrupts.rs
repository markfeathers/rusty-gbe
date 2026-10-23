use super::GBEmulator;

const IF: u16     = 0xFF0F;
const IE: u16     = 0xFFFF;

impl GBEmulator {
    pub fn request_irq(&mut self, irq: u8) {
        let value = self.mmu_read8(IF) | (1 << irq);
        self.mmu_write8(IF, value);
    }

    pub fn handle_irqs(&mut self) {
        if self.mmu_read8(IF) & self.mmu_read8(IE) & 0x1F != 0 {
            self.halted = false;
        }
        if !self.interrupts_en {
            return;
        }
        let irq_flags = self.mmu_read8(IF);
        let irqs = irq_flags & self.mmu_read8(IE) & 0x1F;

        for bit in 0..5 {
            if irqs & (1 << bit) != 0 {
                self.advance_cycles(8);
                self.stack_push(self.regs.pc);
                self.regs.pc = match bit {
                    0 => 0x40, /* V-Blank */
                    1 => 0x48, /* LCD-STATE */
                    2 => 0x50, /* Timer */
                    3 => 0x58, /* Serial */
                    4 => 0x60, /* Joypad */
                    _ => panic!("Invalid interrupt {} fired", bit),
                };
                self.interrupts_en = false;
                self.mmu_write8(IF, irq_flags & !(1 << bit));
                break;
            }
        }
    }
}
