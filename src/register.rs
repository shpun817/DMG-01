pub(crate) struct Registers {
    pub(crate) a: u8,
    pub(crate) b: u8,
    pub(crate) c: u8,
    pub(crate) d: u8,
    pub(crate) e: u8,
    pub(crate) f: FlagsRegister,
    pub(crate) h: u8,
    pub(crate) l: u8,
}

impl Registers {
    pub(crate) fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | u8::from(self.f) as u16
    }

    pub(crate) fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = FlagsRegister::from((value & 0xFF) as u8);
    }

    pub(crate) fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub(crate) fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    pub(crate) fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub(crate) fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub(crate) fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub(crate) fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}

#[derive(Clone, Copy)]
pub(crate) struct FlagsRegister {
    /// Set to true if the result of the operation is equal to 0
    pub(crate) zero: bool,
    /// Set to true if the operation was a subtraction
    pub(crate) subtract: bool,
    /// Set to true if the operation resulted in an overflow from the lower nibble (a.k.a the lower four bits) to the upper nibble (a.k.a the upper four bits)
    pub(crate) half_carry: bool,
    /// Set to true if the operation resulted in an overflow
    pub(crate) carry: bool,
}

/*
   ┌-> Carry
 ┌-+> Subtraction
 | |
1111 0000
| |
└-+> Zero
  └-> Half Carry
*/

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

impl std::convert::From<FlagsRegister> for u8 {
    fn from(register: FlagsRegister) -> Self {
        (register.zero as u8) << ZERO_FLAG_BYTE_POSITION
            | (register.subtract as u8) << SUBTRACT_FLAG_BYTE_POSITION
            | (register.half_carry as u8) << HALF_CARRY_FLAG_BYTE_POSITION
            | (register.carry as u8) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        Self {
            zero: ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0,
            subtract: ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0,
            half_carry: ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0,
            carry: ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0,
        }
    }
}
