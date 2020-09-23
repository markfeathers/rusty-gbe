use super::GBEmulator;

impl GBEmulator {
    pub fn mmu_read8(&self, addr: u16) -> u8 {
        let addr = addr as usize;
        match addr {
            /* Cartridge, fixed bank 00 */
            0x0    ..= 0xFF => {
                if self.in_bios {
                    self.bios[addr]
                } else {
                    self.rom[addr]
                }
            },
            0x100    ..= 0x3FFF => self.rom[addr],
            /* Cartridge, selectable bank */
            0x4000 ..= 0x7FFF => { self.rom[addr] },
            /* 8 KiB VRAM, switchable bank 0/1 */
            0x8000 ..= 0x9FFF => { self.mem[addr] },
            /* 8 KiB External RAM, in catridge with switchable banks */
            0xA000 ..= 0xBFFF => { self.mem[addr] },
            /* 4 KiB Work RAM bank 0 */
            0xC000 ..= 0xCFFF => { self.mem[addr] },
            /* 4 KiB Work RAM bank 1 */
            0xD000 ..= 0xDFFF => { self.mem[addr] },
            /* Alises 0xC000-DDFF */
            0xE000 ..= 0xFDFF => self.mmu_read8((addr - 0x2000) as u16),
            /* Sprite Attribute Table (OAM) */
            0xFE00 ..= 0xFE9F => { self.mem[addr] },
            /* Reserved, does nothing */
            0xFEA0 ..= 0xFEFF => { 0x0 },
            /* IO Ports */
            0xFF00 ..= 0xFF7F => { self.mem[addr] },
            /* High RAM (HRAM) */
            0xFF80 ..= 0xFFFE => { self.mem[addr] },
            /* Interrupt Enable Register */
            0xFFFF            => { self.mem[addr] },
            _                 => panic!("Tried to access memory outside the MMU"),
        }
    }

    pub fn mmu_write8(&mut self, addr: u16, value: u8) {
        let addr = addr as usize;
        match addr {
            /* Cartridge, fixed bank 00 */
            0x0    ..= 0x3FFF => {  },
            /* Cartridge, selectable bank */
            0x4000 ..= 0x7FFF => {  },
            /* 8 KiB VRAM, switchable bank 0/1 */
            0x8000 ..= 0x9FFF => { self.mem[addr] = value },
            /* 8 KiB External RAM, in catridge with switchable banks */
            0xA000 ..= 0xBFFF => { self.mem[addr] = value },
            /* 4 KiB Work RAM bank 0 */
            0xC000 ..= 0xCFFF => { self.mem[addr] = value },
            /* 4 KiB Work RAM bank 1 */
            0xD000 ..= 0xDFFF => { self.mem[addr] = value },
            /* Alises 0xC000-DDFF */
            0xE000 ..= 0xFDFF => self.mmu_write8((addr - 0x2000) as u16, value),
            /* Sprite Attribute Table (OAM) */
            0xFE00 ..= 0xFE9F => { self.mem[addr] = value },
            /* Reserved, does nothing */
            0xFEA0 ..= 0xFEFF => {  },
            /* IO Ports */
            0xFF00 ..= 0xFF03 => { self.mem[addr] = value },
            0xFF04            => { self.mem[addr] = 0 },
            0xFF05 ..= 0xFF45 => { self.mem[addr] = value },
            0xFF46            => { self.dma_transfer(value) },
            0xFF47 ..= 0xFF4F => { self.mem[addr] = value },
            0xFF50            => { self.in_bios = false },
            0xFF51 ..= 0xFF7F => { self.mem[addr] = value },
            /* High RAM (HRAM) */
            0xFF80 ..= 0xFFFE => { self.mem[addr] = value },
            /* Interrupt Enable Register */
            0xFFFF            => { self.mem[addr] = value },
            _                 => panic!("Tried to access memory outside the MMU: {:#06X} at pc {:#06X}", addr, self.regs.pc),
        };
    }

    fn dma_transfer(&mut self, value: u8) {
        let addr = (value as u16) << 8;
        for offset in 0..0xA0 {
            self.mmu_write8(0xFE00 + offset, self.mmu_read8(addr + offset));
        }
    }

    pub fn mmu_read16(&self, addr: u16) -> u16 {
        ((self.mmu_read8(addr + 1) as u16) << 8) | (self.mmu_read8(addr) as u16)
    }

    pub fn mmu_write16(&mut self, addr: u16, value: u16) {
        self.mmu_write8(addr, value as u8);
        self.mmu_write8(addr + 1, (value >> 8) as u8);
    }

    pub fn pc_read8(&mut self) -> u8 {
        let val = self.mmu_read8(self.regs.pc);
        self.regs.pc += 1;
        val
    }

    pub fn pc_read16(&mut self) -> u16 {
        let val = self.mmu_read16(self.regs.pc);
        self.regs.pc += 2;
        val
    }
}