pub(crate) enum Instruction {
    ADD(ArithmeticTarget),
    INC(IncDecTarget),
    RLC(PrefixTarget),
    JP(JumpTest),
    LD(LoadType),
    PUSH(StackTarget),
    POP(StackTarget),
    CALL(JumpTest),
    RET(JumpTest),
}

pub(crate) enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub(crate) enum IncDecTarget {
    BC,
    DE,
}

pub(crate) enum PrefixTarget {
    B,
}

pub(crate) enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

pub(crate) enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
}

pub(crate) enum LoadByteTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
}
pub(crate) enum LoadByteSource {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    /// "Direct 8 bit value" - The byte at `pc+1`
    D8,
    HLI,
}

pub(crate) enum StackTarget {
    BC,
}

impl Instruction {
    pub(crate) fn from_byte(byte: u8, is_prefixed: bool) -> Option<Self> {
        if is_prefixed {
            Self::from_byte_prefixed(byte)
        } else {
            Self::from_byte_not_prefixed(byte)
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Self> {
        match byte {
            0x00 => Some(Self::RLC(PrefixTarget::B)),
            _ =>
            /* TODO: Add mapping for rest of instructions */
            {
                None
            }
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Self> {
        match byte {
            0x02 => Some(Self::INC(IncDecTarget::BC)),
            0x13 => Some(Self::INC(IncDecTarget::DE)),
            _ =>
            /* TODO: Add mapping for rest of instructions */
            {
                None
            }
        }
    }
}
