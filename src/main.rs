
/* 
 * https://bgb.bircd.org/pandocs.htm#cpuregistersandflags 
 * http://imrannazar.com/GameBoy-Emulation-in-JavaScript:-The-CPU */

pub struct Gameboy {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    f: u8,
    sp: u16,
    pc: u16,

    op_clocks: u32,
    total_clocks: u32,
    op_code_count: u32,

    rom: Vec<u8>,
    work: Vec<u8>,
}

impl Gameboy {
    pub fn mmu_peek8(&self, addr: &u16) -> u8 {
        match addr {
            /* Cartridge, fixed bank 00 */
            0x0    ..= 0x3FFF => self.rom[*addr as usize],
            /* Cartridge, selectable bank */
            0x4000 ..= 0x7FFF => { Self::mmu_notimplemented(&addr); 0x0 },
            /* 8 KiB VRAM, switchable bank 0/1 */
            0x8000 ..= 0x9FFF => { Self::mmu_notimplemented(&addr); 0x0 },
            /* 8 KiB External RAM, in catridge with switchable banks */
            0xA000 ..= 0xBFFF => { Self::mmu_notimplemented(&addr); 0x0 },
            /* 4 KiB Work RAM bank 0 */
            0xC000 ..= 0xCFFF => self.work[(addr-0xC000) as usize],
            /* 4 KiB Work RAM bank 1 */
            0xD000 ..= 0xDFFF => self.work[(addr-0xC000) as usize],
            /* Alises 0xC000-DDFF */
            0xE000 ..= 0xFDFF => Self::mmu_peek8(self, &(addr - 0x2000)),
            /* Sprite Attribute Table (OAM) */
            0xFE00 ..= 0xFE9F => { Self::mmu_notimplemented(&addr); 0x0 },
            /* Reserved, does nothing */
            0xFEA0 ..= 0xFEFF => { Self::mmu_notimplemented(&addr); 0x0 },
            /* IO Ports */
            0xFF00 ..= 0xFF7F => { Self::mmu_notimplemented(&addr); 0x0 },
            /* High RAM (HRAM) */
            0xFF80 ..= 0xFFFE => { Self::mmu_notimplemented(&addr); 0x0 },
            /* Interrupt Enable Register */
            0xFFFF            => { Self::mmu_notimplemented(&addr); 0x0 },
        }
    }

    pub fn mmu_poke8(&mut self, addr: &u16, value: &u8) {
        match addr {
            /* Cartridge, fixed bank 00 */
            0x0    ..= 0x3FFF => { self.rom[*addr as usize] = *value; },
            /* Cartridge, selectable bank */
            0x4000 ..= 0x7FFF => { Self::mmu_notimplemented(&addr); },
            /* 8 KiB VRAM, switchable bank 0/1 */
            0x8000 ..= 0x9FFF => { Self::mmu_notimplemented(&addr); },
            /* 8 KiB External RAM, in catridge with switchable banks */
            0xA000 ..= 0xBFFF => { Self::mmu_notimplemented(&addr); },
            /* 4 KiB Work RAM bank 0 */
            0xC000 ..= 0xCFFF => self.work[(addr-0xC000) as usize] = *value,
            /* 4 KiB Work RAM bank 1 */
            0xD000 ..= 0xDFFF => self.work[(addr-0xC000) as usize] = *value,
            /* Alises 0xC000-DDFF */
            0xE000 ..= 0xFDFF => Self::mmu_poke8(self, &(addr - 0x2000), &value),
            /* Sprite Attribute Table (OAM) */
            0xFE00 ..= 0xFE9F => { Self::mmu_notimplemented(&addr); },
            /* Reserved, does nothing */
            0xFEA0 ..= 0xFEFF => { Self::mmu_notimplemented(&addr); },
            /* IO Ports */
            0xFF00 ..= 0xFF7F => { Self::mmu_notimplemented(&addr); },
            /* High RAM (HRAM) */
            0xFF80 ..= 0xFFFE => { Self::mmu_notimplemented(&addr); },
            /* Interrupt Enable Register */
            0xFFFF            => { Self::mmu_notimplemented(&addr); },
        };
    }

    #[cfg(feature="debug-asm")]
    fn cpu_debugasm(&self, ins: &str, len: u16) {
        eprint!("{0: <10}", ins);
        if len != 0 {
            eprint!(":");
        }
        for arg in 0..len {
            eprint!(" {:#04X}", Self::mmu_peek8(&self, &(self.pc + arg)))
        }
        eprintln!();
    }

    #[cfg(not(feature="debug-asm"))]
    fn cpu_debugasm(_ins: &str, _len: u8) {
        
    }

    fn mmu_notimplemented(addr: &u16) {
        panic!("Didn't implement addr {} yet", addr);
    }

    pub fn mmu_peek16(&self, addr: &u16) -> u16 {
        ((Self::mmu_peek8(&self, &(addr + 1)) as u16) << 8) | (Self::mmu_peek8(&self, &addr) as u16) 
    }

    pub fn mmu_poke16(&mut self, addr: &u16, value: &u16) {
        Self::mmu_poke8(self, &addr, &(*value as u8));
        Self::mmu_poke8(self, &(addr + 1), &((*value >> 8) as u8));
    }

    pub fn new(path: &str) -> Result<Gameboy, Box<dyn std::error::Error + 'static>> {
        let work: Vec<u8> = vec![0; 32768];
        
        Ok(Gameboy {
            a: 1,
            b: 0,
            c: 0x13,
            d: 0,
            e: 0xd8,
            h: 0x01,
            l: 0x4d,
            f: 0,
            sp: 0xFFFE,
            pc: 0x100, /* Skip Bios */
            op_clocks: 0,
            total_clocks: 0,
            op_code_count: 0,

            work,
            rom: std::fs::read(&path)?,
        })
    }

    fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    fn set_flags(is_zero: bool,
                 is_negative: bool) -> u8 {
        ((is_zero as u8) << 7) | 
        (is_negative as u8) << 6
    }

    fn pc_peek8(&mut self) -> u8 {
        let val = Self::mmu_peek8(&self, &self.pc);
        self.pc += 1;
        val
    }

    pub fn dispatch(&mut self) {
        let op = Self::pc_peek8(self);
        self.op_code_count += 1;

        /* Until I know better, then based on this discussion I'm using match to go through opcodes:
         * https://users.rust-lang.org/t/why-is-a-lookup-table-faster-than-a-match-expression/24233
         * 
         * There is a recommendation to let branch prediction do its thing, and
         * while a lookup table may win in microbenchmarks, I'm predicting many games
         * are running a tight consistent loop and branch prediction should perform well. */
        self.op_clocks = match op {
            /* https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
             * http://www.z80.info/zip/z80cpu_um.pdf
             */

            /* NOP */
            0x00 => {
                Self::cpu_debugasm(&self, "nop", 0);
                4
            },

            /* DEC B */
            0x05 => {
                Self::cpu_debugasm(&self, "dec b", 1);
                let (value, overflow) = self.b.overflowing_sub(Self::pc_peek8(self));
                self.b = value;
                self.f = Self::set_flags(self.b == 0, overflow);
                4
            },

            /* LD B */
            0x06 => {
                Self::cpu_debugasm(&self, "ld b", 1);
                self.b = Self::pc_peek8(self);
                8
            }

            /* LD C */
            0x0E => {
                Self::cpu_debugasm(&self, "ld c", 1);
                self.c = Self::pc_peek8(self);
                8
            },

            /* LD HL, d16 */
            0x21 => {
                Self::cpu_debugasm(&self, "ld hl", 2);
                self.l = Self::pc_peek8(self);
                self.h = Self::pc_peek8(self);
                8
            },

            /* LD HL-, A */
            0x32 => {
                Self::cpu_debugasm(&self, "ld hl-, a", 1);
                let addr = &Self::get_hl(self);
                let a = self.a;
                Self::mmu_poke8(self, addr, &a);
                8
            },

            /* XOR A */
            0xAF => {
                Self::cpu_debugasm(&self, "xor a", 0);
                self.a ^= self.a;

                self.f = Self::set_flags(self.b == 0, false);
                if self.a == 0 {
                    self.f = 1 << 7;
                } else {
                    self.f = 0;
                }
                4
            },

            /* JP a16 */
            0xC3 => {
                Self::cpu_debugasm(&self, "jp a16", 0);
                self.pc = Self::mmu_peek16(&self, &self.pc);
                16
            },

            _ => {
                panic!("Opcode {:#04X}:{:#06X} not implemented! Made it to {} instructions, {} clocks.", 
                    op,
                    Self::mmu_peek16(&self, &self.pc),
                    self.op_code_count,
                    self.total_clocks);
            },
        };

        self.total_clocks += self.op_clocks;
    }
}


fn main() {
    let mut gb = Gameboy::new("./src/Tetris.gb").unwrap();

    loop {
        gb.dispatch();
    }
}
