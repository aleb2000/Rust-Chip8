use arbitrary_int::u4;
use arbitrary_int::u12;

#[derive(Debug)]
pub enum Inst {
    SYS(u12),
    CLS,
    RET,
    JP(u12),
    CALL(u12),
    SE(u4, u8),
    SNE(u4, u8),
    SEV(u4, u4),
    LD(u4, u8),
    ADD(u4, u8),
    LDV(u4, u4),
    OR(u4, u4),
    AND(u4, u4),
    XOR(u4, u4),
    ADDV(u4, u4),
    SUB(u4, u4),
    SHR(u4, u4),
    SUBN(u4, u4),
    SHL(u4, u4),
    SNEV(u4, u4),
    LDI(u12),
    JPV(u12),
    RND(u4, u8),
    DRW(u4, u4, u4),
    SKP(u4),
    SKNP(u4),
    LDVDT(u4),
    LDVKEY(u4),
    LDDTV(u4),
    LDSTV(u4),
    ADDIV(u4),
    LDFV(u4),
    LDBV(u4),
    LDIV(u4),
    LDVI(u4),
}

impl Inst {
    pub fn decode(opcode: u16) -> Option<Inst> {
        let inst = match opcode {
            0x00E0 => Inst::CLS,
            0x00EE => Inst::RET,
            0x0000..=0x0FFF => Inst::SYS(u12::extract_u16(opcode, 0)),
            0x1000..=0x1FFF => Inst::JP(u12::extract_u16(opcode, 0)),
            0x2000..=0x2FFF => Inst::CALL(u12::extract_u16(opcode, 0)),
            0x3000..=0x3FFF => Inst::SE(u4::extract_u16(opcode, 8), (opcode & 0x00FF) as u8),
            0x4000..=0x4FFF => Inst::SNE(u4::extract_u16(opcode, 8), (opcode & 0x00FF) as u8),
            0x5000..=0x5FFF => Inst::SEV(u4::extract_u16(opcode, 8), u4::extract_u16(opcode, 4)),
            0x6000..=0x6FFF => Inst::LD(u4::extract_u16(opcode, 8), (opcode & 0x00FF) as u8),
            0x7000..=0x7FFF => Inst::ADD(u4::extract_u16(opcode, 8), (opcode & 0x00FF) as u8),
            0x8000..=0x8FFF => match opcode & 0x000F {
                0x0 => Inst::LDV(u4::extract_u16(opcode, 8), u4::extract_u16(opcode, 4)),
                0x1 => Inst::OR(u4::extract_u16(opcode, 8), u4::extract_u16(opcode, 4)),
                0x2 => Inst::AND(u4::extract_u16(opcode, 8), u4::extract_u16(opcode, 4)),
                0x3 => Inst::XOR(u4::extract_u16(opcode, 8), u4::extract_u16(opcode, 4)),
                0x4 => Inst::ADDV(u4::extract_u16(opcode, 8), u4::extract_u16(opcode, 4)),
                0x5 => Inst::SUB(u4::extract_u16(opcode, 8), u4::extract_u16(opcode, 4)),
                0x6 => Inst::SHR(u4::extract_u16(opcode, 8), u4::extract_u16(opcode, 4)),
                0x7 => Inst::SUBN(u4::extract_u16(opcode, 8), u4::extract_u16(opcode, 4)),
                0xE => Inst::SHL(u4::extract_u16(opcode, 8), u4::extract_u16(opcode, 4)),
                _ => return None,
            },
            0x9000..=0x9FFF => match opcode & 0x000F {
                0x0 => Inst::SNEV(u4::extract_u16(opcode, 8), u4::extract_u16(opcode, 4)),
                _ => return None,
            },
            0xA000..=0xAFFF => Inst::LDI(u12::extract_u16(opcode, 0)),
            0xB000..=0xBFFF => Inst::JPV(u12::extract_u16(opcode, 0)),
            0xC000..=0xCFFF => Inst::RND(u4::extract_u16(opcode, 8), (opcode & 0x00FF) as u8),
            0xD000..=0xDFFF => Inst::DRW(
                u4::extract_u16(opcode, 8),
                u4::extract_u16(opcode, 4),
                u4::extract_u16(opcode, 0),
            ),
            0xE000..=0xEFFF => match opcode & 0x00FF {
                0x9E => Inst::SKP(u4::extract_u16(opcode, 8)),
                0xA1 => Inst::SKNP(u4::extract_u16(opcode, 8)),
                _ => return None,
            },
            0xF000..=0xFFFF => match opcode & 0x00FF {
                0x07 => Inst::LDVDT(u4::extract_u16(opcode, 8)),
                0x0A => Inst::LDVKEY(u4::extract_u16(opcode, 8)),
                0x15 => Inst::LDDTV(u4::extract_u16(opcode, 8)),
                0x18 => Inst::LDSTV(u4::extract_u16(opcode, 8)),
                0x1E => Inst::ADDIV(u4::extract_u16(opcode, 8)),
                0x29 => Inst::LDFV(u4::extract_u16(opcode, 8)),
                0x33 => Inst::LDBV(u4::extract_u16(opcode, 8)),
                0x55 => Inst::LDIV(u4::extract_u16(opcode, 8)),
                0x65 => Inst::LDVI(u4::extract_u16(opcode, 8)),
                _ => return None,
            }
        };

        Some(inst)
    }
}
