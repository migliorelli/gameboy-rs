pub mod opcodes;

use crate::utils::{merge_bytes, ByteOps, check_c_borrow_u8, check_c_carry_u8, check_h_carry_u8, check_h_borrow_u8};

#[derive(Copy, Clone)]
pub enum Regs8 {
    A, B, C, D, E, F, H, L, HL
}

#[derive(Copy, Clone)]
pub enum Regs16 {
    AF, BC, DE, HL, SP,
}

pub enum Flags {
    Z, N, H, C,
}

pub struct CPU {
    pc: u16,
    sp: u16,
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            pc: 0x0000,
            sp: 0x0000,
            a: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            f: 0x00,
            h: 0x00,
            l: 0x00,
        }
    }

    pub fn fetch(&mut self) -> u8 {
        let value = self.read_ram(self.pc);
        self.pc += 1;
        value
    }

    pub fn fetch_u16(&mut self) -> u16 {
        let lo = self.fetch();
        let hi = self.fetch();
        let value = merge_bytes(hi, lo);
        value
    }

    pub fn read_ram(&self, addr: u16) -> u8 {
        todo!();
    }

    pub fn write_ram(&self, addr: u16, value: u8) {
        todo!();
    }

    pub fn dec_r8(&mut self, r: Regs8) {
        let value = self.get_r8(r);
        let dec = value.wrapping_sub(1);
        let half = check_h_borrow_u8(value, 1);

        self.set_r8(r, dec);
        self.set_flag(Flags::N, true);
        self.set_flag(Flags::Z, dec == 0);
        self.set_flag(Flags::H, half);
    }
    pub fn inc_r8(&mut self, r: Regs8) {
        let value = self.get_r8(r);
        let inc = value.wrapping_add(1);
        let half = check_h_carry_u8(value, 1);

        self.set_r8(r, inc);
        self.set_flag(Flags::N, false);
        self.set_flag(Flags::Z, inc == 0);
        self.set_flag(Flags::H, half);
    }

    pub fn dec_r16(&mut self, r: Regs16) {
        let value = self.get_r16(r);
        let dec = value.wrapping_sub(1);
        self.set_r16(r, dec);
    }

    pub fn inc_r16(&mut self, r: Regs16) {
        let value = self.get_r16(r);
        let inc = value.wrapping_add(1);
        self.set_r16(r, inc);
    }

    pub fn get_r8(&self, r: Regs8) -> u8 {
        match r {
            Regs8::A => { self.a },
            Regs8::B => { self.b },
            Regs8::C => { self.c },
            Regs8::D => { self.d },
            Regs8::E => { self.e },
            Regs8::F => { self.f },
            Regs8::H => { self.h },
            Regs8::L => { self.l },
            Regs8::HL => {
                let addr = self.get_r16(Regs16::HL);
                self.read_ram(addr)
            }
        }
    }

    pub fn set_r8(&mut self, r: Regs8, value: u8) {
        match r {
            Regs8::A => { self.a = value },
            Regs8::B => { self.b = value },
            Regs8::C => { self.c = value },
            Regs8::D => { self.d = value },
            Regs8::E => { self.e = value },
            Regs8::F => { self.f = value & 0xF0 /* bottom nibble is always 0 */ },
            Regs8::H => { self.h = value },
            Regs8::L => { self.l = value },
            Regs8::HL => {
                let addr = self.get_r16(Regs16::HL);
                self.write_ram(addr, value);
            }
        }
    }

    pub fn get_r16(&self, r: Regs16) -> u16 {
        match r {
            Regs16::AF => { merge_bytes(self.a, self.f) },
            Regs16::BC => { merge_bytes(self.b, self.c) },
            Regs16::DE => { merge_bytes(self.d, self.e) },
            Regs16::HL => { merge_bytes(self.h, self.l) },
            Regs16::SP => { self.sp },
        }
    }

    pub fn set_r16(&mut self, r: Regs16, value: u16) {
        let hi = value.high_byte();
        let lo = value.low_byte();

        match r {
            Regs16::AF => {
                self.set_r8(Regs8::A, hi);
                self.set_r8(Regs8::F, lo);
            },
            Regs16::BC => {
                self.set_r8(Regs8::B, hi);
                self.set_r8(Regs8::C, lo);
            },
            Regs16::DE => {
                self.set_r8(Regs8::D, hi);
                self    .set_r8(Regs8::E, lo);
            },
            Regs16::HL => {
                self.set_r8(Regs8::H, hi);
                self.set_r8(Regs8::L, lo);
            },
            Regs16::SP => {
                self.sp = value
            },
        }
    }

    pub fn get_flag(&self, f: Flags) -> bool {
        match f {
            Flags::Z => { (self.f >> 7) & 1 == 1 },
            Flags::N => { (self.f >> 6) & 1 == 1 },
            Flags::H => { (self.f >> 5) & 1 == 1 },
            Flags::C => { (self.f >> 4) & 1 == 1 },
        }
    }

    pub fn set_flag(&mut self, f: Flags, value: bool) {
        if value {
            match f {
                Flags::Z => { self.f |= 1 << 7 },
                Flags::N => { self.f |= 1 << 6 },
                Flags::H => { self.f |= 1 << 5 },
                Flags::C => { self.f |= 1 << 4 },
            }
        } else {
            match f {
                Flags::Z => { self.f &= !(1 << 7) },
                Flags::N => { self.f &= !(1 << 6) },
                Flags::H => { self.f &= !(1 << 5) },
                Flags::C => { self.f &= !(1 << 4) },
            }
        }
    }
}
