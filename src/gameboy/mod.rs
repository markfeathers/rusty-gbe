pub use registers::{Registers};

//pub use self::gameboy::

mod cpu;
mod registers;
mod mmu;
mod gpu;
mod interrupts;
mod timers;

/* 
 * https://gbdev.io/gb-opcodes/optables/
 * https://gekkio.fi/files/gb-docs/gbctr.pdf
 * https://raw.githubusercontent.com/AntonioND/giibiiadvance/master/docs/TCAGBD.pdf
 * http://www.z80.info/zip/z80cpu_um.pdf
 * http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf
 * https://bgb.bircd.org/pandocs.htm#cpuregistersandflags 
 * http://imrannazar.com/GameBoy-Emulation-in-JavaScript:-The-CPU
 * https://gbdev.gg8.se/files/roms/blargg-gb-tests/
 * http://www.codeslinger.co.uk/pages/projects/gameboy/graphics.html
 */

pub struct GBEmulator {
    mem: [u8; 0x10000],
    pub regs: Registers,
    in_bios: bool,
    bios: Vec<u8>,
    rom: Vec<u8>,
    debug: bool,
    stopped: bool,
    halted: bool,
    interrupts_en: bool,
    pub frame_hz: u32,
    gpu_frame_cycles: u32,
    pub framebuffer: [u8; 160*144*4], /* RGB for each pixel */
}

impl GBEmulator {
    pub fn new(bios: Vec<u8>, rom: Vec<u8>)  -> GBEmulator {
        let mut gb = GBEmulator {
            mem: [0; 0x10000],
            regs: Registers::default(),
            in_bios: true,
            bios,
            rom,
            debug: false,
            stopped: false,
            halted: false,
            interrupts_en: false,
            frame_hz: 60,
            gpu_frame_cycles: 456,
            framebuffer: [0; 160*144*4],
        };

        gb.mmu_write8(0xFF41, 0x84); /* STAT */
        gb.mmu_write8(0xFF47, 0xFC); /* BGP */
        gb.mmu_write8(0xFF48, 0xFF); /* OBP0 */
        gb.mmu_write8(0xFF49, 0xFF); /* OBG1 */

        gb
    }
}
