use std::io;
use super::GBEmulator;

impl GBEmulator {
    pub fn cpu_run_op(&mut self) -> u32 {
        if self.regs.pc > 0x100 {
            self.debug = true;
        } 
        //self.debug = true;
        if self.debug {
            self.dump_regs();
            //io::stdin().read_line(&mut String::new()).unwrap();
        }

        let op = self.pc_read8();

        if self.debug {
            println!("================================");
        }

        let op_clocks: u32 = match op {
            0x00 => { /* NOP */
                4
            },
            0x01 => {/* LD BC, u16 */
                let value = self.pc_read16();
                self.regs.set_bc(value);
                12
            },
            0x02 => { /* LD (BC), A */
                self.mmu_write8(self.regs.get_bc(), self.regs.a);
                8
            },
            0x03 => { /* INC BC */
                let result = self.increment_u16(self.regs.get_bc());
                self.regs.set_bc(result);
                8
            },
            0x04 => { /* INC B */
                self.regs.b = self.increment_u8(self.regs.b);
                4
            },
            0x05 => { /* DEC B */
                self.regs.b = self.decrement_u8(self.regs.b);
                4
            },
            0x06 => { /* LD B */
                self.regs.b = self.pc_read8();
                8
            },
            0x07 => { /* RLCA */
                self.regs.a = self.rotate_left(self.regs.a);
                self.regs.flags.zero = false;
                4
            },
            0x08 => { /* LD (u16), SP */
                let addr = self.pc_read16();
                self.mmu_write16(addr, self.regs.sp);
                20
            },
            0x09 => { /* ADD HL, BC */
                let result = self.add_u16(self.regs.get_hl(), self.regs.get_bc());
                self.regs.set_hl(result);
                8
            },
            0x0A => { /* LD A, (BC) */
                self.regs.a = self.mmu_read8(self.regs.get_bc());
                8
            },
            0x0B => { /* DEC BC */
                let value = self.decrement_u16(self.regs.get_bc());
                self.regs.set_bc(value);
                8
            },
            0x0C => { /* INC C */
                self.regs.c = self.increment_u8(self.regs.c);
                4
            },
            0x0D => { /* DEC C */
                self.regs.c = self.decrement_u8(self.regs.c);
                4
            },
            0x0E => { /* LD C */
                self.regs.c = self.pc_read8();
                8
            },
            0x0F => { /* RRCA */
                self.regs.a = self.regs.a.rotate_right(1);
                self.regs.flags.negative = false;
                self.regs.flags.half_carry = false;
                self.regs.flags.carry = self.regs.a & (1 << 7) != 0;
                self.regs.flags.zero = false;
                4
            },
            0x10 => { /* STOP */
                self.stopped = true;
                4
            },
            0x11 => { /* LD DE, D16 */
                let value = self.pc_read16();
                self.regs.set_de(value);
                12
            },
            0x12 => { /* LD (DE), A */
                self.mmu_write8(self.regs.get_de(), self.regs.a);
                8
            }
            0x13 => { /* INC DE */
                let result = self.increment_u16(self.regs.get_de());
                self.regs.set_de(result);
                8
            },
            0x14 => { /* INC D */
                self.regs.d = self.increment_u8(self.regs.d);
                4
            },
            0x15 => { /* DEC D */
                self.regs.d = self.decrement_u8(self.regs.d);
                4
            },
            0x16 => { /* LD D, u8 */
                self.regs.d = self.pc_read8();
                8
            },
            0x17 => { /* RLA */
                self.regs.a = self.rotate_left_carry(self.regs.a);
                4
            },
            0x18 => { /* JR i8 */
                let val = self.pc_read8() as i8;
                self.jump(val);
                12
            },
            0x19 => { /* ADD HL, DE */
                let result = self.add_u16(self.regs.get_hl(), self.regs.get_de());
                self.regs.set_hl(result);
                8
            },
            0x1A => { /* LD A, (DE) */
                self.regs.a = self.mmu_read8(self.regs.get_de());
                8
            },
            0x1B => { /* DEC DE */
                let value = self.decrement_u16(self.regs.get_de());
                self.regs.set_de(value);
                8
            },
            0x1C => { /* INC E */
                self.regs.e = self.increment_u8(self.regs.e);
                4
            },
            0x1D => { /* DEC E */
                self.regs.e = self.decrement_u8(self.regs.e);
                4
            },
            0x1E => { /* LD E, u8 */
                self.regs.e = self.pc_read8();
                8
            },
            0x1F => { /* RRA */
                self.regs.a = self.rotate_right_carry(self.regs.a);
                self.regs.flags.zero = false;
                4
            },
            0x20 => { /* JR NZ,r8 */
                let val = self.pc_read8() as i8;
                if self.regs.flags.zero {
                    8
                } else {
                    self.jump(val);
                    12
                }
            },
            0x21 => { /* LD HL, d16 */
                self.regs.l = self.pc_read8();
                self.regs.h = self.pc_read8();
                12
            },
            0x22 => { /* LD (HL+), A */
                let addr = self.regs.get_hl();
                self.mmu_write8(addr, self.regs.a);
                self.regs.set_hl(addr.wrapping_add(1));
                8
            },
            0x23 => { /* INC HL */
                let result = self.increment_u16(self.regs.get_hl());
                self.regs.set_hl(result);
                8
            },
            0x24 => { /* INC H */
                self.regs.h = self.increment_u8(self.regs.h);
                4
            },
            0x25 => { /* DEC H */
                self.regs.h = self.decrement_u8(self.regs.h);
                4
            },
            0x26 => { /* LD H, u8 */
                self.regs.h = self.pc_read8();
                8
            },
            0x27 => { /* DAA  - Welcome to the thunderdome */
                /* https://old.reddit.com/r/EmuDev/comments/cdtuyw/gameboy_emulator_fails_blargg_daa_test/ */
                let mut correction: u16 = if self.regs.flags.carry {
                    0x60
                } else {
                    0x00
                };
                
                if self.regs.flags.half_carry || (!self.regs.flags.negative && (self.regs.a & 0x0f) > 9) {
                    correction |= 0x06;
                }

                if self.regs.flags.carry | (!self.regs.flags.negative && (self.regs.a > 0x99)) {
                    correction |= 0x60;
                }

                self.regs.a = if self.regs.flags.negative {
                    self.regs.a.wrapping_sub(correction as u8)
                } else {
                    self.regs.a.wrapping_add(correction as u8)
                };

                if((correction << 2) & 0x100) != 0 {
                    self.regs.flags.carry = true;
                }
                self.regs.flags.half_carry = false;
                self.regs.flags.zero = self.regs.a == 0;

                4
            },
            0x28 => { /* JR Z, i8 */
                let val = self.pc_read8() as i8;
                if self.regs.flags.zero {
                    self.jump(val);
                    12
                } else {
                    8
                }
            },
            0x29 => { /* ADD HL, HL */
                let result = self.add_u16(self.regs.get_hl(), self.regs.get_hl());
                self.regs.set_hl(result);
                8
            },
            0x2A => { /* LD A, (HL+) */
                let addr = self.regs.get_hl();
                self.regs.a = self.mmu_read8(addr);
                self.regs.set_hl(addr.wrapping_add(1));
                8
            },
            0x2B => { /* DEC HL */
                let value = self.decrement_u16(self.regs.get_hl());
                self.regs.set_hl(value);
                8
            },
            0x2C => { /* INC L */
                self.regs.l = self.increment_u8(self.regs.l);
                4
            },
            0x2D => { /* DEC L */
                self.regs.l = self.decrement_u8(self.regs.l);
                4
            },
            0x2E => { /* LD L, d8 */
                self.regs.l = self.pc_read8();
                8
            },
            0x2F => { /* CPL */
                self.regs.a = !self.regs.a;
                self.regs.flags.half_carry = true;
                self.regs.flags.negative = true;
                4
            },
            0x30 => { /* JR NC, r8 */
                let value = self.pc_read8() as i8;
                if self.regs.flags.carry {
                    8
                } else {
                    self.jump(value);
                    12
                }
            },
            0x31 => { /* ld SP, d16 */
                self.regs.sp = self.pc_read16();
                12
            },
            0x32 => { /* LD HL-, A */
                let addr = self.regs.get_hl();
                self.mmu_write8(addr, self.regs.a);
                self.regs.set_hl(addr.wrapping_sub(1));
                8
            },
            0x33 => { /* INC SP */
                let result = self.increment_u16(self.regs.sp);
                self.regs.sp = result;
                8
            },
            0x34 => { /* INC (HL) */
                let addr = self.regs.get_hl();
                let result = self.increment_u8(self.mmu_read8(addr));
                self.mmu_write8(addr, result);
                12
            },
            0x35 => { /* DEC (HL) */
                let addr = self.regs.get_hl();
                let result = self.decrement_u8(self.mmu_read8(addr));
                self.mmu_write8(addr, result);
                12
            },
            0x36 => { /* LD (HL), d8 */
                let addr = self.regs.get_hl();
                let value = self.pc_read8();
                self.mmu_write8(addr, value);
                12
            },
            0x37 => { /* SCF */
                self.regs.flags.carry = true;
                4
            },
            0x38 => { /* JR C, i8 */
                let val = self.pc_read8() as i8;
                if self.regs.flags.carry {
                    self.jump(val);
                    12
                } else {
                    8
                }
            },
            0x39 => { /* ADD HL, SP */
                let result = self.add_u16(self.regs.get_hl(), self.regs.sp);
                self.regs.set_hl(result);
                8
            },
            0x3A => { /* LD A, (HL-) */
                let addr = self.regs.get_hl();
                self.regs.a = self.mmu_read8(addr);
                self.regs.set_hl(addr.wrapping_sub(1));
                8
            },
            0x3B => { /* DEC SP */
                let value = self.decrement_u16(self.regs.sp);
                self.regs.sp = value;
                8
            },
            0x3C => { /* INC A */
                self.regs.a = self.increment_u8(self.regs.a);
                4
            },
            0x3D => { /* DEC A */
                self.regs.a = self.decrement_u8(self.regs.a);
                4
            },
            0x3E => { /* LD A, d8 */
                self.regs.a = self.pc_read8();
                8
            },
            0x3F => { /* CCF */
                self.regs.flags.carry = !self.regs.flags.carry;
                4
            },
            0x40 => { /* LD B, B */
                self.regs.b = self.regs.b;
                4
            },
            0x41 => { /* LD B, C */
                self.regs.b = self.regs.c;
                4
            },
            0x42 => { /* LD B, D */
                self.regs.b = self.regs.d;
                4
            }, 
            0x43 => { /* LD B, E */
                self.regs.b = self.regs.e;
                4
            },
            0x44 => { /* LD B, H */
                self.regs.b = self.regs.h;
                4
            },
            0x45 => { /* LD B, L */
                self.regs.b = self.regs.l;
                4
            },
            0x46 => { /* LD B, HL */
                self.regs.b = self.mmu_read8(self.regs.get_hl());
                8
            },
            0x47 => { /* LD B, A */
                self.regs.b = self.regs.a;
                4
            },
            0x48 => {
                /* LD C, B */
                self.regs.c = self.regs.b;
                4
            },
            0x49 => { /* LD C, B */
                self.regs.c = self.regs.b;
                4
            },
            0x4A => { /* LD C, D */
                self.regs.c = self.regs.d;
                4
            },
            0x4B => { /* LD C, E */
                self.regs.c = self.regs.e;
                4
            },
            0x4C => { /* LD C, H */
                self.regs.c = self.regs.h;
                4
            },
            0x4D => { /* LD C, L */
                self.regs.c = self.regs.l;
                4
            },
            0x4E => { /* LD B, HL */
                self.regs.c = self.mmu_read8(self.regs.get_hl());
                8
            },
            0x4F => { /* LD C, L */
                self.regs.c = self.regs.a;
                4
            },
            0x50 => { /* LD D, B */
                self.regs.d = self.regs.b;
                4
            },
            0x51 => { /* LD D, C */
                self.regs.d = self.regs.c;
                4
            },
            0x52 => { /* LD D, D */
                self.regs.d = self.regs.d;
                4
            },
            0x53 => { /* LD D, E */
                self.regs.d = self.regs.e;
                4
            },
            0x54 => { /* LD D, H */
                self.regs.d = self.regs.h;
                4
            },
            0x55 => { /* LD D, L */
                self.regs.d = self.regs.l;
                4
            },
            0x56 => { /* LD D, HL */
                self.regs.d = self.mmu_read8(self.regs.get_hl());
                8
            },
            0x57 => { /* LD D, A */
                self.regs.d = self.regs.a;
                4
            },
            0x58 => { /* LD E, B */
                self.regs.e = self.regs.b;
                4
            },
            0x59 => { /* LD E, B */
                self.regs.e = self.regs.b;
                4
            },
            0x5A => { /* LD E, D */
                self.regs.e = self.regs.d;
                4
            },
            0x5B => { /* LD E, E */
                self.regs.e = self.regs.e;
                4
            },
            0x5C => { /* LD E, H */
                self.regs.e = self.regs.h;
                4
            },
            0x5D => { /* LD E, L */
                self.regs.e = self.regs.l;
                4
            },
            0x5E => { /* LD E, HL */
                self.regs.e = self.mmu_read8(self.regs.get_hl());
                8
            },
            0x5F => { /* LD E, A */
                self.regs.e = self.regs.a;
                4
            },
            0x60 => { /* LD H, B */
                self.regs.h = self.regs.b;
                4
            },
            0x61 => { /* LD H, C */
                self.regs.h = self.regs.c;
                4
            },
            0x62 => { /* LD H, D */
                self.regs.h = self.regs.d;
                4
            },
            0x63 => { /* LD H, E */
                self.regs.h = self.regs.e;
                4
            },            
            0x64 => { /* LD H, H */
                self.regs.h = self.regs.h;
                4
            },
            0x65 => { /* LD H, L */
                self.regs.h = self.regs.l;
                4
            },
            0x66 => { /* LD H, HL */
                self.regs.h = self.mmu_read8(self.regs.get_hl());
                8
            },
            0x67 => { /* LD H, A */
                self.regs.h = self.regs.a;
                4
            },
            0x68 => { /* LD L, B */
                self.regs.l = self.regs.b;
                4
            },
            0x69 => { /* LD L, B */
                self.regs.l = self.regs.b;
                4
            },
            0x6A => { /* LD L, D */
                self.regs.l = self.regs.d;
                4
            },
            0x6B => { /* LD L, E */
                self.regs.l = self.regs.e;
                4
            },
            0x6C => { /* LD L, H */
                self.regs.l = self.regs.h;
                4
            },
            0x6D => { /* LD L, L */
                self.regs.l = self.regs.l;
                4
            },
            0x6E => { /* LD L, HL */
                self.regs.l = self.mmu_read8(self.regs.get_hl());
                8
            },
            0x6F => { /* LD L, A */
                self.regs.l = self.regs.a;
                4
            },
            0x70 => { /* LD (HL), B */
                self.mmu_write8(self.regs.get_hl(), self.regs.b);
                8
            },
            0x71 => { /* LD (HL), C */
                self.mmu_write8(self.regs.get_hl(), self.regs.c);
                8
            },
            0x72 => { /* LD (HL), D */
                self.mmu_write8(self.regs.get_hl(), self.regs.d);
                8
            },
            0x73 => { /* LD (HL), E */
                self.mmu_write8(self.regs.get_hl(), self.regs.e);
                8
            },
            0x74 => { /* LD (HL), H */
                self.mmu_write8(self.regs.get_hl(), self.regs.h);
                8
            },
            0x75 => { /* LD (HL), L */
                self.mmu_write8(self.regs.get_hl(), self.regs.l);
                8
            },
            0x76 => { /* HALT */
                self.halted = true;
                4
            },
            0x77 => { /* LD (HL), A */
                self.mmu_write8(self.regs.get_hl(), self.regs.a);
                8
            },
            0x78 => { /* LD A, B */
                self.regs.a = self.regs.b;
                4
            },
            0x79 => { /* LD A, B */
                self.regs.a = self.regs.b;
                4
            },
            0x7A => { /* LD A, D */
                self.regs.a = self.regs.d;
                4
            },
            0x7B => { /* LD A, E */
                self.regs.a = self.regs.e;
                4
            },
            0x7C => { /* LD A, H */
                self.regs.a = self.regs.h;
                4
            },
            0x7D => { /* LD A, L */
                self.regs.a = self.regs.l;
                4
            },
            0x7E => { /* LD A, HL */
                self.regs.a = self.mmu_read8(self.regs.get_hl());
                8
            },
            0x7F => { /* LD A, A */
                self.regs.a = self.regs.a;
                4
            },
            0x80 => { /* ADD A, B */
                self.regs.a = self.add_u8(self.regs.a, self.regs.b);
                4
            },
            0x81 => { /* ADD A, C */
                self.regs.a = self.add_u8(self.regs.a, self.regs.c);
                4
            },
            0x82 => { /* ADD A, D */
                self.regs.a = self.add_u8(self.regs.a, self.regs.d);
                4
            },
            0x83 => { /* ADD A, E */
                self.regs.a = self.add_u8(self.regs.a, self.regs.e);
                4
            },
            0x84 => { /* ADD A, H */
                self.regs.a = self.add_u8(self.regs.a, self.regs.h);
                4
            },
            0x85 => { /* ADD A, L */
                self.regs.a = self.add_u8(self.regs.a, self.regs.l);
                4
            },
            0x86 => { /* ADD A, (HL) */
                let value = self.mmu_read8(self.regs.get_hl());
                self.regs.a = self.add_u8(self.regs.a, value);
                8
            },
            0x87 => { /* ADD A, A */
                self.regs.a = self.add_u8(self.regs.a, self.regs.a);
                4
            },
            0x88 => { /* ADC A, B */
                self.regs.a = self.adc(self.regs.b);
                4
            },
            0x89 => { /* ADC A, C */
                self.regs.a = self.adc(self.regs.c);
                4
            },
            0x8A => { /* ADC A, D */
                self.regs.a = self.adc(self.regs.d);
                4
            },
            0x8B => { /* ADC A, E */
                self.regs.a = self.adc(self.regs.e);
                4
            },
            0x8C => { /* ADC A, H */
                self.regs.a = self.adc(self.regs.h);
                4
            },
            0x8D => { /* ADC A, L */
                self.regs.a = self.adc(self.regs.l);
                4
            },
            0x8E => { /* ADC A, (HL) */
                let value = self.mmu_read8(self.regs.get_hl());
                self.regs.a = self.adc(value);
                8
            },
            0x8F => { /* ADC A, A */
                self.regs.a = self.adc(self.regs.a);
                4
            },
            0x90 => { /* SUB B */
                self.regs.a = self.sub(self.regs.b);
                4
            },
            0x91 => { /* SUB C */
                self.regs.a = self.sub(self.regs.c);
                4
            },
            0x92 => { /* SUB D */
                self.regs.a = self.sub(self.regs.d);
                4
            },
            0x93 => { /* SUB E */
                self.regs.a = self.sub(self.regs.e);
                4
            },
            0x94 => { /* SUB H */
                self.regs.a = self.sub(self.regs.h);
                4
            },
            0x95 => { /* SUB L */
                self.regs.a = self.sub(self.regs.l);
                4
            },
            0x96 => { /* SUB HL */
                let value = self.mmu_read8(self.regs.get_hl());
                self.regs.a = self.sub(value);
                8
            },
            0x97 => { /* SUB A */
                self.regs.a = self.sub(self.regs.a);
                self.regs.flags.zero = true;
                4
            },
            0x98 => { /* SBC A, B */
                self.regs.a = self.sbc(self.regs.b);
                4
            },
            0x99 => { /* SBC A, C */
                self.regs.a = self.sbc(self.regs.b);
                4
            },
            0x9A => { /* SBC A, D */
                self.regs.a = self.sbc(self.regs.b);
                4
            },
            0x9B => { /* SBC A, E */
                self.regs.a = self.sbc(self.regs.b);
                4
            },
            0x9C => { /* SBC A, H */
                self.regs.a = self.sbc(self.regs.b);
                4
            },
            0x9D => { /* SBC A, L */
                self.regs.a = self.sbc(self.regs.b);
                4
            },
            0x9E => { /* SBC A, (HL) */
                self.regs.a = self.sbc(self.regs.b);
                8
            },
            0x9F => { /* SBC A, A */
                self.regs.a = self.sbc(self.regs.b);
                4
            },
            0xA0 => { /* AND B */
                self.regs.a = self.and(self.regs.b);
                4
            },
            0xA1 => { /* AND C */
                self.regs.a = self.and(self.regs.c);
                4
            },
            0xA2 => { /* AND D */
                self.regs.a = self.and(self.regs.d);
                4
            },
            0xA3 => { /* AND E */
                self.regs.a = self.and(self.regs.e);
                4
            },
            0xA4 => { /* AND H */
                self.regs.a = self.and(self.regs.h);
                4
            },
            0xA5 => { /* AND L */
                self.regs.a = self.and(self.regs.l);
                4
            },
            0xA6 => { /* AND (HL) */
                let value = self.mmu_read8(self.regs.get_hl());
                self.regs.a = self.and(value);
                8
            },
            0xA7 => { /* AND A */
                self.regs.a = self.and(self.regs.a);
                4
            },
            0xA8 => { /* XOR B */
                self.regs.a = self.xor(self.regs.b);
                4
            },
            0xA9 => { /* XOR C */
                self.regs.a = self.xor(self.regs.c);
                4
            },
            0xAA => { /* XOR D */
                self.regs.a = self.xor(self.regs.d);
                4
            },
            0xAB => { /* XOR E */
                self.regs.a = self.xor(self.regs.e);
                4
            },
            0xAC => { /* XOR H */
                self.regs.a = self.xor(self.regs.h);
                4
            },
            0xAD => { /* XOR L */
                self.regs.a = self.xor(self.regs.l);
                4
            },
            0xAE => { /* XOR (HL) */
                let value = self.mmu_read8(self.regs.get_hl());
                self.regs.a = self.xor(value);
                8
            },
            0xAF => { /* XOR A */
                self.regs.a = self.xor(self.regs.a);
                4
            },
            0xB0 => { /* OR B */
                self.regs.a = self.or(self.regs.b);
                4
            },
            0xB1 => { /* OR C */
                self.regs.a = self.or(self.regs.c);
                4
            },
            0xB2 => { /* OR D */
                self.regs.a = self.or(self.regs.d);
                4
            },
            0xB3 => { /* OR E */
                self.regs.a = self.or(self.regs.e);
                4
            },
            0xB4 => { /* OR H */
                self.regs.a = self.or(self.regs.h);
                4
            },
            0xB5 => { /* OR L */
                self.regs.a = self.or(self.regs.l);
                4
            },
            0xB6 => { /* OR (HL) */
                let value = self.mmu_read8(self.regs.get_hl());
                self.regs.a = self.or(value);
                8
            },
            0xB7 => { /* OR A */
                self.regs.a = self.or(self.regs.a);
                4
            },
            0xB8 => { /* CP B */
                self.sub(self.regs.b);
                4
            },
            0xB9 => { /* CP C */
                self.sub(self.regs.c);
                4
            },
            0xBA => { /* CP D */
                self.sub(self.regs.d);
                4
            },
            0xBB => { /* CP E */
                self.sub(self.regs.e);
                4
            },
            0xBC => { /* CP H */
                self.sub(self.regs.h);
                4
            },
            0xBD => { /* CP L */
                self.sub(self.regs.l);
                4
            },
            0xBE => { /* CP (HL) */
                let value = self.mmu_read8(self.regs.get_hl());
                self.regs.a = self.sub(value);
                8
            },
            0xBF => { /* CP A */
                self.regs.a = self.sub(self.regs.a);
                4
            },
            0xC0 => { /* RET NZ */
                if self.regs.flags.zero {
                    8
                } else {
                    self.ret();
                    12
                }
            },
            0xC1 => { /* POP BC */
                let value = self.stack_pop();
                self.regs.set_bc(value);
                12
            },
            0xC2 => { /* JP NZ, a16 */
                let addr = self.pc_read16();
                if self.regs.flags.zero {
                    12
                } else {
                    self.regs.pc = addr;
                    16
                }
            },
            0xC3 => { /* JP u16 */
                self.regs.pc = self.mmu_read16(self.regs.pc);
                16
            },
            0xC4 => { /* CALL NZ, u16 */
                let addr = self.pc_read16();
                if self.regs.flags.zero {
                    12
                } else {
                    self.call(addr);
                    24
                }
            },
            0xC5 => { /* PUSH BC */
                self.stack_push(self.regs.get_bc());
                16
            },
            0xC6 => { /* ADD A, u8 */
                let value = self.pc_read8();
                self.regs.a = self.add_u8(self.regs.a, value);
                4
            },
            0xC7 => { /* RST 00H */
                self.stack_push(self.regs.pc);
                self.regs.pc = 0x00;
                16
            },
            0xC8 => { /* RET Z */
                if self.regs.flags.zero {
                    self.ret();
                    20
                } else {
                    8
                }
            },
            0xC9 => { /* Ret */
                self.ret();

                16
            }
            0xCA => { /* JP Z, u16 */
                if self.regs.flags.zero {
                    self.regs.pc = self.mmu_read16(self.regs.pc);
                    16
                } else {
                    12
                }
            },
            0xCB => { /* Prefix */
                let prefix_op = self.pc_read8();
                let prefix_op_clocks: u32 = match prefix_op {
                    0x10 => { /* RL B */
                        self.regs.b = self.rotate_left_carry(self.regs.b);
                        8
                    },
                    0x11 => { /* RL C */
                        self.regs.c = self.rotate_left_carry(self.regs.c);
                        8
                    },
                    0x12 => { /* RL D */
                        self.regs.d = self.rotate_left_carry(self.regs.d);
                        8
                    },
                    0x13 => { /* RL E */
                        self.regs.e = self.rotate_left_carry(self.regs.e);
                        8
                    },
                    0x14 => { /* RL H */
                        self.regs.h = self.rotate_left_carry(self.regs.h);
                        8
                    },
                    0x15 => { /* RL L */
                        self.regs.l = self.rotate_left_carry(self.regs.l);
                        8
                    },
                    0x16 => { /* RL (HL) */
                        let mut value = self.mmu_read8(self.regs.get_hl());
                        value = self.rotate_left_carry(value);
                        self.mmu_write8(self.regs.get_hl(), value);
                        16
                    },
                    0x17 => { /* RL A */
                        self.regs.a = self.rotate_left_carry(self.regs.a);
                        8
                    },
                    0x18 => { /* RR B */
                        self.regs.b = self.rotate_right_carry(self.regs.b);
                        8
                    },
                    0x19 => { /* RR C */
                        self.regs.c = self.rotate_right_carry(self.regs.c);
                        8
                    },
                    0x1a => { /* RR D */
                        self.regs.d = self.rotate_right_carry(self.regs.d);
                        8
                    },
                    0x1b => { /* RR E */
                        self.regs.e = self.rotate_right_carry(self.regs.e);
                        8
                    },
                    0x1c => { /* RR H */
                        self.regs.h = self.rotate_right_carry(self.regs.h);
                        8
                    },
                    0x1d => { /* RR L */
                        self.regs.l = self.rotate_right_carry(self.regs.l);
                        8
                    },
                    0x1e => { /* RR (HL) */
                        let mut value = self.mmu_read8(self.regs.get_hl());
                        value = self.rotate_right_carry(value);
                        self.mmu_write8(self.regs.get_hl(), value);
                        16
                    },
                    0x1f => { /* RR A */
                        self.regs.a = self.rotate_right_carry(self.regs.a);
                        8
                    },
                    0x20 => { /* SLA B */
                        self.regs.b = self.sla(self.regs.b);
                        8
                    },
                    0x21 => { /* SLA C */
                        self.regs.c = self.sla(self.regs.c);
                        8
                    },
                    0x22 => { /* SLA D */
                        self.regs.d = self.sla(self.regs.d);
                        8
                    },
                    0x23 => { /* SLA E */
                        self.regs.e = self.sla(self.regs.e);
                        8
                    },
                    0x24 => { /* SLA H */
                        self.regs.h = self.sla(self.regs.h);
                        8
                    },
                    0x25 => { /* SLA L */
                        self.regs.l = self.sla(self.regs.l);
                        8
                    },
                    0x26 => { /* SLA (HL) */
                        let mut value = self.mmu_read8(self.regs.get_hl());
                        value = self.sla(value);
                        self.mmu_write8(self.regs.get_hl(), value);
                        12
                    },
                    0x27 => { /* SLA A */
                        self.regs.a = self.sla(self.regs.a);
                        4
                    },

                    0x28 => { /* SRA B */
                        self.regs.b = self.sra(self.regs.b);
                        8
                    },
                    0x29 => { /* SRA C */
                        self.regs.c = self.sra(self.regs.c);
                        8
                    },
                    0x2A => { /* SRA D */
                        self.regs.d = self.sra(self.regs.d);
                        8
                    },
                    0x2B => { /* SRA E */
                        self.regs.e = self.sra(self.regs.e);
                        8
                    },
                    0x2C => { /* SRA H */
                        self.regs.h = self.sra(self.regs.h);
                        8
                    },
                    0x2D => { /* SRA L */
                        self.regs.l = self.sra(self.regs.l);
                        8
                    },
                    0x2E => { /* SRA (HL) */
                        let mut value = self.mmu_read8(self.regs.get_hl());
                        value = self.sra(value);
                        self.mmu_write8(self.regs.get_hl(), value);
                        12
                    },
                    0x2F => { /* SRA A */
                        self.regs.a = self.sra(self.regs.a);
                        4
                    },

                    0x30 => { /* SWAP B */
                        self.regs.b = self.swap(self.regs.b);
                        8
                    },
                    0x31 => { /* SWAP C */
                        self.regs.c = self.swap(self.regs.c);
                        8
                    },
                    0x32 => { /* SWAP D */
                        self.regs.d = self.swap(self.regs.d);
                        8
                    },
                    0x33 => { /* SWAP E */
                        self.regs.e = self.swap(self.regs.e);
                        8
                    },
                    0x34 => { /* SWAP H */
                        self.regs.h = self.swap(self.regs.h);
                        8
                    },
                    0x35 => { /* SWAP L */
                        self.regs.l = self.swap(self.regs.l);
                        8
                    },
                    0x36 => { /* SWAP (HL) */
                        let mut value = self.mmu_read8(self.regs.get_hl());
                        value = self.swap(value);
                        self.mmu_write8(self.regs.get_hl(), value);
                        12
                    },
                    0x37 => { /* SWAP A */
                        self.regs.a = self.swap(self.regs.a);
                        4
                    },
                    0x38 => { /* SRL B */
                        self.regs.b = self.srl(self.regs.b);
                        8
                    },
                    0x39 => { /* SRL C */
                        self.regs.c = self.srl(self.regs.c);
                        8
                    },
                    0x3A => { /* SRL D */
                        self.regs.d = self.srl(self.regs.d);
                        8
                    },
                    0x3B => { /* SRL E */
                        self.regs.e = self.srl(self.regs.e);
                        8
                    },
                    0x3C => { /* SRL H */
                        self.regs.h = self.srl(self.regs.h);
                        8
                    },
                    0x3D => { /* SRL L */
                        self.regs.l = self.srl(self.regs.l);
                        8
                    },
                    0x3E => { /* SRL (HL) */
                        let mut value = self.mmu_read8(self.regs.get_hl());
                        value = self.srl(value);
                        self.mmu_write8(self.regs.get_hl(), value);
                        12
                    },
                    0x3F => { /* SRL A */
                        self.regs.a = self.srl(self.regs.a);
                        4
                    },
                    0x40 => { /* BIT 0, B */
                        self.bit(self.regs.b, 0);
                        8
                    },
                    0x41 => { /* BIT 0, C */
                        self.bit(self.regs.c, 0);
                        8
                    },
                    0x42 => { /* BIT 0, D */
                        self.bit(self.regs.d, 0);
                        8
                    },
                    0x43 => { /* BIT 0, E */
                        self.bit(self.regs.e, 0);
                        8
                    },
                    0x44 => { /* BIT 0, H */
                        self.bit(self.regs.h, 0);
                        8
                    },
                    0x45 => { /* BIT 0, L */
                        self.bit(self.regs.l, 0);
                        8
                    },
                    0x46 => { /* BIT 0, (HL) */
                        let value = self.mmu_read8(self.regs.get_hl());
                        self.bit(value, 0);
                        12
                    },
                    0x47 => { /* BIT 0, A */
                        self.bit(self.regs.a, 0);
                        8
                    },
                    0x48 => { /* BIT 1, B */
                        self.bit(self.regs.b, 1);
                        8
                    },
                    0x49 => { /* BIT 1, C */
                        self.bit(self.regs.c, 1);
                        8
                    },
                    0x4a => { /* BIT 1, D */
                        self.bit(self.regs.d, 1);
                        8
                    },
                    0x4b => { /* BIT 1, E */
                        self.bit(self.regs.e, 1);
                        8
                    },
                    0x4c => { /* BIT 1, H */
                        self.bit(self.regs.h, 1);
                        8
                    },
                    0x4d => { /* BIT 1, L */
                        self.bit(self.regs.l, 1);
                        8
                    },
                    0x4e => { /* BIT 1, (HL) */
                        let value = self.mmu_read8(self.regs.get_hl());
                        self.bit(value, 1);
                        12
                    },
                    0x4f => { /* BIT 1, A */
                        self.bit(self.regs.a, 1);
                        8
                    },
                    0x50 => { /* BIT 2, B */
                        self.bit(self.regs.b, 2);
                        8
                    },
                    0x51 => { /* BIT 2, C */
                        self.bit(self.regs.c, 2);
                        8
                    },
                    0x52 => { /* BIT 2, D */
                        self.bit(self.regs.d, 2);
                        8
                    },
                    0x53 => { /* BIT 2, E */
                        self.bit(self.regs.e, 2);
                        8
                    },
                    0x54 => { /* BIT 2, H */
                        self.bit(self.regs.h, 2);
                        8
                    },
                    0x55 => { /* BIT 2, L */
                        self.bit(self.regs.l, 2);
                        8
                    },
                    0x56 => { /* BIT 2, (HL) */
                        let value = self.mmu_read8(self.regs.get_hl());
                        self.bit(value, 2);
                        12
                    },
                    0x57 => { /* BIT 2, A */
                        self.bit(self.regs.a, 2);
                        8
                    },
                    0x58 => { /* BIT 3, B */
                        self.bit(self.regs.b, 3);
                        8
                    },
                    0x59 => { /* BIT 3, C */
                        self.bit(self.regs.c, 3);
                        8
                    },
                    0x5a => { /* BIT 3, D */
                        self.bit(self.regs.d, 3);
                        8
                    },
                    0x5b => { /* BIT 3, E */
                        self.bit(self.regs.e, 3);
                        8
                    },
                    0x5c => { /* BIT 3, H */
                        self.bit(self.regs.h, 3);
                        8
                    },
                    0x5d => { /* BIT 3, L */
                        self.bit(self.regs.l, 3);
                        8
                    },
                    0x5e => { /* BIT 3, (HL) */
                        let value = self.mmu_read8(self.regs.get_hl());
                        self.bit(value, 3);
                        12
                    },
                    0x5f => { /* BIT 3, A */
                        self.bit(self.regs.a, 3);
                        8
                    },
                    0x60 => { /* BIT 4, B */
                        self.bit(self.regs.b, 4);
                        8
                    },
                    0x61 => { /* BIT 4, C */
                        self.bit(self.regs.c, 4);
                        8
                    },
                    0x62 => { /* BIT 4, D */
                        self.bit(self.regs.d, 4);
                        8
                    },
                    0x63 => { /* BIT 4, E */
                        self.bit(self.regs.e, 4);
                        8
                    },
                    0x64 => { /* BIT 4, H */
                        self.bit(self.regs.h, 4);
                        8
                    },
                    0x65 => { /* BIT 4, L */
                        self.bit(self.regs.l, 4);
                        8
                    },
                    0x66 => { /* BIT 4, (HL) */
                        let value = self.mmu_read8(self.regs.get_hl());
                        self.bit(value, 4);
                        12
                    },
                    0x67 => { /* BIT 4, A */
                        self.bit(self.regs.a, 4);
                        8
                    },
                    0x68 => { /* BIT 5, B */
                        self.bit(self.regs.b, 5);
                        8
                    },
                    0x69 => { /* BIT 5, C */
                        self.bit(self.regs.c, 5);
                        8
                    },
                    0x6a => { /* BIT 5, D */
                        self.bit(self.regs.d, 5);
                        8
                    },
                    0x6b => { /* BIT 5, E */
                        self.bit(self.regs.e, 5);
                        8
                    },
                    0x6c => { /* BIT 5, H */
                        self.bit(self.regs.h, 5);
                        8
                    },
                    0x6d => { /* BIT 5, L */
                        self.bit(self.regs.l, 5);
                        8
                    },
                    0x6e => { /* BIT 5, (HL) */
                        let value = self.mmu_read8(self.regs.get_hl());
                        self.bit(value, 5);
                        12
                    },
                    0x6f => { /* BIT 5, A */
                        self.bit(self.regs.a, 5);
                        8
                    },
                    0x70 => { /* BIT 6, B */
                        self.bit(self.regs.b, 6);
                        8
                    },
                    0x71 => { /* BIT 6, C */
                        self.bit(self.regs.c, 6);
                        8
                    },
                    0x72 => { /* BIT 6, D */
                        self.bit(self.regs.d, 6);
                        8
                    },
                    0x73 => { /* BIT 6, E */
                        self.bit(self.regs.e, 6);
                        8
                    },
                    0x74 => { /* BIT 6, H */
                        self.bit(self.regs.h, 6);
                        8
                    },
                    0x75 => { /* BIT 6, L */
                        self.bit(self.regs.l, 6);
                        8
                    },
                    0x76 => { /* BIT 6, (HL) */
                        let value = self.mmu_read8(self.regs.get_hl());
                        self.bit(value, 6);
                        12
                    },
                    0x77 => { /* BIT 6, A */
                        self.bit(self.regs.a, 6);
                        8
                    },
                    0x78 => { /* BIT 7, B */
                        self.bit(self.regs.b, 7);
                        8
                    },
                    0x79 => { /* BIT 7, C */
                        self.bit(self.regs.c, 7);
                        8
                    },
                    0x7a => { /* BIT 7, D */
                        self.bit(self.regs.d, 7);
                        8
                    },
                    0x7b => { /* BIT 7, E */
                        self.bit(self.regs.e, 7);
                        8
                    },
                    0x7c => { /* BIT 7, H */
                        self.bit(self.regs.h, 7);
                        8
                    },
                    0x7d => { /* BIT 7, L */
                        self.bit(self.regs.l, 7);
                        8
                    },
                    0x7e => { /* BIT 7, (HL) */
                        let value = self.mmu_read8(self.regs.get_hl());
                        self.bit(value, 7);
                        12
                    },
                    0x7f => { /* BIT 7, A */
                        self.bit(self.regs.a, 7);
                        8
                    },
                    0x80 => { /* RES 0, B */
                        self.regs.b  &= !(1 << 0);
                        8
                    },
                    0x81 => { /* RES 0, C */
                        self.regs.c &= !(1 << 0);
                        8
                    },
                    0x82 => { /* RES 0, D */
                        self.regs.d &= !(1 << 0);
                        8
                    },
                    0x83 => { /* RES 0, E */
                        self.regs.e &= !(1 << 0);
                        8
                    },
                    0x84 => { /* RES 0, H */
                        self.regs.h &= !(1 << 0);
                        8
                    },
                    0x85 => { /* RES 0, L */
                        self.regs.l &= !(1 << 0);
                        8
                    },
                    0x86 => { /* RES 0, (HL) */
                        let addr = self.regs.get_hl();
                        let mut value = self.mmu_read8(addr);
                        value &= !(1 << 0);
                        self.mmu_write8(addr, value);
                        16
                    },
                    0x87 => { /* RES 0, A */
                        self.regs.a &= !(1 << 0);
                        8
                    },
                    0x88 => { /* RES 1, B */
                        self.regs.b &= !(1 << 1);
                        8
                    },
                    0x89 => { /* RES 1, C */
                        self.regs.c &= !(1 << 1);
                        8
                    },
                    0x8a => { /* RES 1, D */
                        self.regs.d &= !(1 << 1);
                        8
                    },
                    0x8b => { /* RES 1, E */
                        self.regs.e &= !(1 << 1);
                        8
                    },
                    0x8c => { /* RES 1, H */
                        self.regs.h &= !(1 << 1);
                        8
                    },
                    0x8d => { /* RES 1, L */
                        self.regs.l &= !(1 << 1);
                        8
                    },
                    0x8e => { /* RES 1, (HL) */
                        let addr = self.regs.get_hl();
                        let mut value = self.mmu_read8(addr);
                        value &= !(1 << 1);
                        self.mmu_write8(addr, value);
                        16
                    },
                    0x8f => { /* RES 1, A */
                        self.regs.l &= !(1 << 1);
                        8
                    },
                    0x90 => { /* RES 2, B */
                        self.regs.b  &= !(1 << 2);
                        8
                    },
                    0x91 => { /* RES 2, C */
                        self.regs.c &= !(1 << 2);
                        8
                    },
                    0x92 => { /* RES 2, D */
                        self.regs.d &= !(1 << 2);
                        8
                    },
                    0x93 => { /* RES 2, E */
                        self.regs.e &= !(1 << 2);
                        8
                    },
                    0x94 => { /* RES 2, H */
                        self.regs.h &= !(1 << 2);
                        8
                    },
                    0x95 => { /* RES 2, L */
                        self.regs.l &= !(1 << 2);
                        8
                    },
                    0x96 => { /* RES 2, (HL) */
                        let addr = self.regs.get_hl();
                        let mut value = self.mmu_read8(addr);
                        value &= !(1 << 2);
                        self.mmu_write8(addr, value);
                        16
                    },
                    0x97 => { /* RES 2, A */
                        self.regs.a &= !(1 << 2);
                        8
                    },
                    0x98 => { /* RES 3, B */
                        self.regs.b &= !(1 << 3);
                        8
                    },
                    0x99 => { /* RES 3, C */
                        self.regs.c &= !(1 << 3);
                        8
                    },
                    0x9a => { /* RES 3, D */
                        self.regs.d &= !(1 << 3);
                        8
                    },
                    0x9b => { /* RES 3, E */
                        self.regs.e &= !(1 << 3);
                        8
                    },
                    0x9c => { /* RES 3, H */
                        self.regs.h &= !(1 << 3);
                        8
                    },
                    0x9d => { /* RES 3, L */
                        self.regs.l &= !(1 << 3);
                        8
                    },
                    0x9e => { /* RES 3, (HL) */
                        let addr = self.regs.get_hl();
                        let mut value = self.mmu_read8(addr);
                        value &= !(1 << 3);
                        self.mmu_write8(addr, value);
                        16
                    },
                    0x9f => { /* RES 3, A */
                        self.regs.l &= !(1 << 3);
                        8
                    },
                    
                    _ => {
                        self.dump_regs();
                        panic!("Prefix Opcode {:#04X}:{:#04X}:{:#04X} not implemented!", 
                            op,
                            self.mmu_read8(self.regs.pc.wrapping_sub(1)),
                            self.mmu_read8(self.regs.pc));
                    },
                };

                4 + prefix_op_clocks
            }
            0xCC => { /*  CALL Z, a16  */
                let addr = self.pc_read16();
                if self.regs.flags.zero {
                    self.call(addr);
                    24
                } else {
                    12
                }
            },
            0xCD => { /* Call nn */
                let addr = self.pc_read16();
                self.call(addr);

                24
            },
            0xCE => { /* ADC A, D8 */
                let value = self.pc_read8();
                self.regs.a = self.adc(value);
                8
            },
            0xCF => { /* RST 08H */
                self.stack_push(self.regs.pc);
                self.regs.pc = 0x08;
                16
            },
            0xD0 => { /* RET NC */
                if self.regs.flags.carry {
                    8
                } else {
                    self.ret();
                    20
                }
            },
            0xD1 => { /* POP DE */
                let value = self.stack_pop();
                self.regs.set_de(value);
                12
            },
            0xD2 => { /* JP NC, A16 */
                let addr = self.pc_read16();
                if self.regs.flags.carry {
                    12
                } else {
                    self.regs.pc = addr;
                    16
                }
            },
            0xD3 => {
                panic!("0xD3 is not a valid instruction. PC: {:#06X}", self.regs.pc);
            },
            0xD4 => { /*  CALL NC, a16  */
                let addr = self.pc_read16();
                if self.regs.flags.carry {
                    12
                } else {
                    self.call(addr);
                    24
                }
            },
            0xD5 => { /* PUSH DE */
                self.stack_push(self.regs.get_de());
                16
            },
            0xD6 => { /* SUB D8 */
                let value = self.pc_read8();
                self.regs.a = self.sub(value);
                8
            },
            0xD7 => { /* RST 10H */
                self.stack_push(self.regs.pc);
                self.regs.pc = 0x10;
                16
            },
            0xD8 => { /* RET C */
                if self.regs.flags.carry {
                    self.ret();
                    20
                } else {
                    8
                }
            },
            0xD9 => { /* RETI */
                self.ret();
                self.interrupts_en = true;
                16
            },
            0xDA => { /* JP C, A16 */
                if self.regs.flags.zero {
                    self.regs.pc = self.mmu_read16(self.regs.pc);
                    16
                } else {
                    12
                }
            },
            0xDB => {
                panic!("0xDB is not a valid instruction. PC: {:#06X}", self.regs.pc);
            },
            0xDC => { /* CALL C, a16 */
                let addr = self.pc_read16();
                if self.regs.flags.carry {
                    self.call(addr);
                    24
                } else {
                    12
                }
            }
            0xDD => {
                panic!("0xDD is not a valid instruction. PC: {:#06X}", self.regs.pc);
            }
            0xDE => { /* CALL C, A16  */
                let addr = self.pc_read16();
                if self.regs.flags.carry {
                    self.call(addr);
                    24
                } else {
                    12
                }
            },
            0xDF => { /* RST 18H */
                self.stack_push(self.regs.pc);
                self.regs.pc = 0x18;
                16
            },
            0xE0 => { /* LDH (a8), A */
                let addr = 0xff00 as u16 | self.pc_read8() as u16;
                self.mmu_write8(addr, self.regs.a);
                12
            },
            0xE1 => { /* POP HL */
                let value = self.stack_pop();
                self.regs.set_hl(value);
                12
            },
            0xE2 => { /* LD (0xff00+c), a */
                let addr = 0xFF00 | self.regs.c as u16;
                self.mmu_write8(addr, self.regs.a);
                8
            },
            0xE3 => {
                panic!("0xE3 is not a valid instruction.");
            },
            0xE4 => {
                panic!("0xE4 is not a valid instruction.");
            },
            0xE5 => { /* PUSH HL */
                self.stack_push(self.regs.get_hl());
                16
            },
            0xE6 => { /* AND d8 */
                let value = self.pc_read8();
                self.regs.a = self.and(value);
                8
            },
            0xE7 => { /* RST 20H */
                self.stack_push(self.regs.pc);
                self.regs.pc = 0x20;
                16
            },
            0xE8 => {
                let value = self.pc_read8();
                self.regs.sp = self.add_u16(self.regs.sp, value as u16);
                16
            },
            0xE9 => { /* JP HL */
                self.regs.pc = self.regs.get_hl();
                4
            },
            0xEA => { /* LD (a16), A */
                let addr = self.pc_read16();
                self.mmu_write8(addr, self.regs.a);
                16
            },
            0xEB => {
                panic!("0xEB is not a valid instruction. PC: {:#06X}", self.regs.pc);
            },
            0xEC => {
                panic!("0xEC is not a valid instruction. PC: {:#06X}", self.regs.pc);
            },
            0xED => {
                panic!("0xED is not a valid instruction. PC: {:#06X}", self.regs.pc);
            },
            0xEE => { /* XOR d8 */
                let value = self.pc_read8();
                self.regs.a = self.xor(value);
                8
            },
            0xEF => { /* RST 28H */
                self.stack_push(self.regs.pc);
                self.regs.pc = 0x28;
                16
            },
            0xF0 => { /* LDH A, a8 */
                let addr = 0xff00 as u16 | self.pc_read8() as u16;
                self.regs.a = self.mmu_read8(addr);
                12
            },
            0xF1 => { /* POP AF */
                let value = self.stack_pop();
                self.regs.set_af(value);
                12
            },
            0xF2 => { /* LD A, (C) */
                let addr = 0xFF00 | self.regs.c as u16;
                self.regs.a = self.mmu_read8(addr);
                8
            },
            0xF3 => { /* DI */
                self.interrupts_en = false;
                4
            },
            0xF4 => {
                panic!("0xED is not a valid instruction. PC: {:#06X}", self.regs.pc);
            },
            0xF5 => {
                self.stack_push(self.regs.get_af());
                16
            },
            0xF6 => {
                let value = self.pc_read8();
                self.regs.a = self.or(value);
                8
            },
            0xF7 => { /* RST 30H */
                self.stack_push(self.regs.pc);
                self.regs.pc = 0x30;
                16
            },
            0xF8 => { /* LD HL, SP + r8 */
                let value = self.pc_read8() as i8;
                if value.is_negative() {
                    self.regs.sp = self.sub_u16(self.regs.sp, value.abs() as u16);
                } else {
                    self.regs.sp = self.add_u16(self.regs.sp, value as u16);
                };

                12
            },
            0xF9 => { /* LD HL, SP */
                self.regs.set_hl(self.regs.sp);
                8
            },
            0xFA => { /* LD A, (a16) */
                let addr = self.pc_read16();
                self.regs.a = self.mmu_read8(addr);
                16
            },
            0xFB => { /* EI */
                self.interrupts_en = true;
                4
            },
            0xFC => {
                panic!("0xFC is not a valid instruction. PC: {:#06X}", self.regs.pc);
            },
            0xFD => {
                panic!("0xFD is not a valid instruction. PC: {:#06X}", self.regs.pc);
            },
            0xFE => { /* CP d8 */
                let value = self.pc_read8();
                self.sub(value);

                8
            },
            0xFF => { /* RST 38H */
                self.stack_push(self.regs.pc);
                self.regs.pc = 0x38;
                16
            },

            /*_ => {
                panic!("Opcode {:#04X}:{:#06X} not implemented! Made it to {} instructions, {} clocks.", 
                    op,
                    self.mmu_read16(self.regs.pc),
                    self.op_code_count,
                    self.total_clocks);
            },*/
        };

        op_clocks
    }

    fn add_u8(&mut self, value1: u8, value2: u8) -> u8 {
        let (result, overflow) = value1.overflowing_add(value2);

        self.regs.flags.negative = false;
        self.regs.flags.zero = result == 0;
        self.regs.flags.half_carry = (result & 0x0F) == 0x00;
        self.regs.flags.carry = overflow;

        result
    }

    fn adc(&mut self, value: u8) -> u8 {
        let (mut result, mut overflow) = self.regs.a.overflowing_add(value);
        if self.regs.flags.carry {
            let (c_result, c_overflow) = result.overflowing_add(1);
            if c_overflow {
                overflow = true;
            }
            result = c_result;
        }

        self.regs.flags.zero = result == 0;
        self.regs.flags.carry = overflow;
        self.regs.flags.half_carry = (result & 0x0F) == 0x00;
        self.regs.flags.negative = false;

        result
    }

    fn sbc(&mut self, value: u8) -> u8 {
        let (mut result, mut overflow) = self.regs.a.overflowing_sub(value);
        if self.regs.flags.carry {
            let (c_result, c_overflow) = result.overflowing_sub(1);
            if c_overflow {
                overflow = true;
            }
            result = c_result;
        }

        self.regs.flags.zero = result == 0;
        self.regs.flags.carry = overflow;
        self.regs.flags.half_carry = result & 0x0F == 0x0F;
        self.regs.flags.negative = false;

        result
    }

    fn increment_u8(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);

        self.regs.flags.negative = false;
        self.regs.flags.zero = result == 0;
        self.regs.flags.half_carry = (result & 0x0F) == 0x00;

        result
    }

    fn increment_u16(&mut self, value: u16) -> u16 {
        value.wrapping_add(1)
    }

    fn decrement_u8(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);
        self.regs.flags.negative = true;
        self.regs.flags.zero = result == 0;
        self.regs.flags.half_carry = result & 0x0F == 0x0F;

        result
    }

    fn decrement_u16(&mut self, value: u16) -> u16 {
        value.wrapping_sub(1)
    }

    fn sub(&mut self, value: u8) -> u8 {
        if value > self.regs.a {
            self.regs.flags.carry = true;
        } else {
            self.regs.flags.carry = false;
        }
        if value & 0x0f > self.regs.a & 0x0f {
            self.regs.flags.half_carry = true;
        } else {
            self.regs.flags.half_carry = false;
        }
        let result = self.regs.a.wrapping_sub(value);
        self.regs.flags.zero = result == 0;
        self.regs.flags.negative = true;

        result
    }

    fn and(&mut self, value: u8) -> u8 {
        let result = self.regs.a & value;
        self.regs.flags.zero = result == 0;
        self.regs.flags.half_carry = true;
        self.regs.flags.carry = false;
        self.regs.flags.negative = false;

        result
    }

    fn or(&mut self, value: u8) -> u8 {
        let result = self.regs.a | value;
        self.regs.flags.zero = result == 0;
        self.regs.flags.half_carry = false;
        self.regs.flags.carry = false;
        self.regs.flags.negative = false;
        result
    }

    fn add_u16(&mut self, value1: u16, value2: u16) -> u16 {
        let (result, overflow) = value1.overflowing_add(value2);
        self.regs.flags.negative = false;
        self.regs.flags.zero = false;
        self.regs.flags.carry = overflow;
        self.regs.flags.half_carry = (value1 & 0xfff) + (value2 & 0xfff) > 0xfff;

        result
    }

    fn sub_u16(&mut self, value1: u16, value2: u16) -> u16 {
        let (result, overflow) = value1.overflowing_sub(value2);
        self.regs.flags.negative = false;
        self.regs.flags.carry = overflow;
        self.regs.flags.half_carry = (value1 & 0xfff) + (value2 & 0xfff) > 0xfff;

        result
    }

    fn xor(&mut self, value: u8) -> u8 {
        let result = self.regs.a ^ value; 
        self.regs.flags.zero = result == 0;
        self.regs.flags.negative = false;
        self.regs.flags.half_carry = false;
        self.regs.flags.carry = false;

        result
    }

    fn swap(&mut self, value: u8) -> u8 {
        let result = value.rotate_right(4);
        self.regs.flags.negative = false;
        self.regs.flags.half_carry = false;
        self.regs.flags.carry = false;
        self.regs.flags.zero = result == 0;

        result
    }

    fn srl(&mut self, value: u8) -> u8 {
        self.regs.flags.carry = value & 0x1 != 0;
        let result = (value & 0x7f) >> 1;
        self.regs.flags.negative = false;
        self.regs.flags.half_carry = false;
        self.regs.flags.zero = result == 0;

        result
    }

    fn sra(&mut self, value: u8) -> u8 {
        self.regs.flags.carry = value & 0x1 != 0;
        let result = value >> 1;
        self.regs.flags.negative = false;
        self.regs.flags.half_carry = false;
        self.regs.flags.zero = result == 0;

        result
    }

    fn sla(&mut self, value: u8) -> u8 {
        self.regs.flags.carry = value & (1 << 7) != 0;
        let result = value << 1;
        self.regs.flags.negative = false;
        self.regs.flags.half_carry = false;
        self.regs.flags.zero = result == 0;

        result
    }

    fn jump(&mut self, value: i8) {
        if value.is_negative() {
            self.regs.pc = self.regs.pc.wrapping_sub(value.abs() as u16);
        } else {
            self.regs.pc = self.regs.pc.wrapping_add(value as u16);
        }
    }

    /* Rotate left shifting out the MSB, and replace the LSB
     * with the existing carry flag.  */
    fn rotate_left_carry(&mut self, value: u8) -> u8 {
        let carry = self.regs.flags.carry;
        self.regs.flags.carry = value & (1 << 7) != 0;
        let mut result = value << 1;
        if carry {
            result |= 0x1;
        }
        self.regs.flags.zero = value == 0;
        self.regs.flags.negative = false;
        self.regs.flags.half_carry = false;
        result
    }

    /* Rotate right shifting out the LSB, and replace the MSB
     * with the existing carry flag.  */
    fn rotate_right_carry(&mut self, value: u8) -> u8 {
        let carry = self.regs.flags.carry;
        self.regs.flags.carry = value & 0x1 != 0;
        let mut result = value >> 1;
        if carry {
            result |= 0x1;
        }
        self.regs.flags.zero = value == 0;
        self.regs.flags.negative = false;
        self.regs.flags.half_carry = false;
        result
    }

    /* Rotate right, wrapping the LSB to the MSB */
    fn rotate_right(&mut self, value: u8) -> u8 {
        let result = value.rotate_right(1);

        self.regs.flags.zero = value == 0;
        self.regs.flags.half_carry = false;
        self.regs.flags.carry = value & 0x1 != 0;
        self.regs.flags.negative = false;
        
        result
    }

    /* Rotate left, wrapping the MSB to the LSB */
    fn rotate_left(&mut self, value: u8) -> u8 {
        let result = value.rotate_left(1);

        self.regs.flags.zero = value == 0;
        self.regs.flags.half_carry = false;
        self.regs.flags.carry = value & (1 << 7) != 0;
        self.regs.flags.negative = false;
        
        result
    }

    fn bit(&mut self, value: u8, bit: u8) {
        self.regs.flags.zero = (value & (1 << bit)) == 0;
        self.regs.flags.negative = false;
        self.regs.flags.half_carry = true;
    } 

    pub fn stack_push(&mut self, val: u16) {
        self.regs.sp = self.regs.sp.wrapping_sub(2);
        self.mmu_write16(self.regs.sp, val);
    }

    pub fn stack_pop(&mut self) -> u16 {
        let value = self.mmu_read16(self.regs.sp);
        self.regs.sp = self.regs.sp.wrapping_add(2);
        value
    }

    fn call(&mut self, addr: u16) {
        self.stack_push(self.regs.pc);
        self.regs.pc = addr;
    }

    fn ret(&mut self) {
        self.regs.pc = self.stack_pop();
    }

    fn dump_regs(&self) {
        println!("af: {:#06X}", self.regs.get_af());
        println!("bc: {:#06X}", self.regs.get_bc());
        println!("de: {:#06X}", self.regs.get_de());
        println!("hl: {:#06X}", self.regs.get_hl());
        println!("sp: {:#06X}", self.regs.sp);
        println!("pc: {:#06X}", self.regs.pc);
        println!("ly: {:#04X}", self.mmu_read8(0xFF44));
    }
}