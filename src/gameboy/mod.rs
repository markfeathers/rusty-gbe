pub use registers::{Registers};

//pub use self::gameboy::

mod cpu;
mod registers;
mod mmu;
mod gpu;
mod interrupts;
mod timers;
mod apu;

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
    rom_bank: u8,
    debug: bool,
    stopped: bool,
    halted: bool,
    halt_bug: bool,
    ime_delay: u8,
    interrupts_en: bool,
    divider: u16,
    cpu_cycles: u32,
    cgb_mode: bool,
    double_speed: bool,
    apu: apu::Apu,
    pub frame_hz: u32,
    gpu_frame_cycles: u32,
    pub framebuffer: [u8; 160*144*4], /* RGB for each pixel */
}

impl GBEmulator {
    /// Select Color hardware behavior; the caller still supplies the appropriate boot ROM.
    pub fn new_cgb(bios: Vec<u8>, rom: Vec<u8>) -> GBEmulator {
        let mut gb = Self::new(bios, rom);
        gb.cgb_mode = true;
        gb
    }

    pub fn new(bios: Vec<u8>, rom: Vec<u8>)  -> GBEmulator {
        let mut gb = GBEmulator {
            mem: [0; 0x10000],
            regs: Registers::default(),
            in_bios: true,
            bios,
            rom,
            rom_bank: 1,
            debug: false,
            stopped: false,
            halted: false,
            halt_bug: false,
            ime_delay: 0,
            interrupts_en: false,
            divider: 0,
            cpu_cycles: 0,
            cgb_mode: false,
            double_speed: false,
            apu: apu::Apu::default(),
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
