pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
    pub flags: FlagsReg,
}

#[derive(Copy, Clone, Debug)]
pub struct FlagsReg {
    pub carry: bool,
    pub zero: bool,
    pub negative: bool,
    pub half_carry: bool,
}

impl std::convert::From<FlagsReg> for u8  {
    fn from(flag: FlagsReg) -> u8 {
        ((flag.zero as u8) << 7) |
        ((flag.negative as u8) << 6) |
        ((flag.half_carry as u8) << 5) |
        ((flag.carry as u8) << 4)
    }
}
impl std::convert::From<u8> for FlagsReg {
    fn from(val: u8) -> Self {
        FlagsReg {
            zero: (val & (1 << 7)) != 0,
            negative: (val & (1 << 6)) != 0,
            half_carry: (val & (1 << 5)) != 0,
            carry: (val & (1 << 4)) != 0,
        }
    }
}

impl Registers {
    pub fn default_no_bios() -> Registers {
        Registers {
            a: 1,
            b: 0,
            c: 0x13,
            d: 0,
            e: 0xd8,
            h: 0x01,
            l: 0x4d,
            sp: 0xFFFE,
            flags: FlagsReg::from(0xB0),
            pc: 0x100,
        }
    }
    pub fn default() -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            flags: FlagsReg::from(0x00),
            pc: 0,
        }
    }

    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn set_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = val as u8;
    }

    pub fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | u8::from(self.flags)  as u16
    }

    pub fn set_af(&mut self, val: u16) {
        self.a = (val >> 8) as u8;
        self.flags = (val as u8).into();
    }

    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }
    
    pub fn set_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = val as u8;
    }

    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn set_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = val as u8;
    }
}