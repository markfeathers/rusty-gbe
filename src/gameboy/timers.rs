use super::GBEmulator;

const DIV: u16  = 0xFF03;
const TIMA: u16 = 0xFF05;
const TIM: u16  = 0xFF06;
const TAC: u16  = 0xFF07;

impl GBEmulator {
    pub fn timers_run(&mut self, cycles: u32) {
        /* DIV always counts at 16khz
         * This is the 4MHz/256 
         * */
        let mut div = self.mmu_read8(DIV);
        div = div.wrapping_add(cycles as u8);
        self.mmu_write8(DIV, div);

        let tac = self.mmu_read8(TAC);

    }
}