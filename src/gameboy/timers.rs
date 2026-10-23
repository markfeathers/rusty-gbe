use super::GBEmulator;

const DIV: usize = 0xFF04;
const TIMA: usize = 0xFF05;
const TMA: usize = 0xFF06;
const TAC: usize = 0xFF07;

impl GBEmulator {
    pub fn timers_run(&mut self, cycles: u32) {
        let tac = self.mem[TAC];
        let bit = [9, 3, 5, 7][(tac & 3) as usize];
        for _ in 0..cycles {
            let previous = self.divider;
            self.divider = self.divider.wrapping_add(1);
            self.mem[DIV] = (self.divider >> 8) as u8;
            // TIMA advances on a falling edge of the selected divider bit.
            if tac & 4 != 0 && previous & (1 << bit) != 0
                && self.divider & (1 << bit) == 0
            {
                let (value, overflow) = self.mem[TIMA].overflowing_add(1);
                self.mem[TIMA] = if overflow { self.mem[TMA] } else { value };
                if overflow {
                    self.request_irq(2);
                }
            }
        }
    }
}
