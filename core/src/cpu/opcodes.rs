use crate::cpu::*;
use crate::utils::*;

const OPCODES: [fn(&mut CPU) -> u8; 256] = [
//   0x00,   0x01,   0x02,   0x03,   0x04,   0x05,   0x06,   0x07,   0x08,   0x09,   0x0A,   0x0B,   0x0C,   0x0D,   0x0E,   0x0F
    nop_00, _todo_, _todo_, inc_03, inc_04, dec_05, _todo_, _todo_, _todo_, _todo_, _todo_, dec_0b, inc_0c, dec_0d, _todo_, _todo_, // 0x00
    _todo_, _todo_, _todo_, inc_13, inc_14, dec_15, _todo_, _todo_, _todo_, _todo_, _todo_, dec_1b, inc_1c, dec_1d, _todo_, _todo_, // 0x10
    _todo_, _todo_, _todo_, inc_23, inc_24, dec_25, _todo_, _todo_, _todo_, _todo_, _todo_, dec_2b, inc_2c, dec_2d, _todo_, _todo_, // 0x20
    _todo_, _todo_, _todo_, inc_33, inc_34, dec_35, _todo_, _todo_, _todo_, _todo_, _todo_, dec_3b, inc_3c, dec_3d, _todo_, _todo_, // 0x30
    lod_40, lod_41, lod_42, lod_43, lod_44, lod_45, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, // 0x40
    lod_50, lod_51, lod_52, lod_53, lod_54, lod_55, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, // 0x50
    lod_60, lod_61, lod_62, lod_63, lod_64, lod_65, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, // 0x60
    lod_70, lod_71, lod_72, lod_73, lod_74, lod_75, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, // 0x70
    _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, // 0x80
    _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, // 0x90
    _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, // 0xA0
    _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, // 0xB0
    _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, // 0xC0
    _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, // 0xD0
    _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, // 0xE0
    _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, _todo_, // 0xF0
];

fn _todo_(cpu: &mut CPU) -> u8 {
    todo!();
}

pub fn execute(cpu: &mut CPU) -> u8 {
    let op_index = cpu.fetch();
    return OPCODES[op_index as usize](cpu);
}

fn nop_00(_cpu: &mut CPU) -> u8 {
    1 // do nothing
}

fn inc_03(_cpu: &mut CPU) -> u8 {
    _cpu.inc_r16(Regs16::BC);
    2
}

fn inc_13(_cpu: &mut CPU) -> u8 {
    _cpu.inc_r16(Regs16::DE);
    2
}

fn inc_23(_cpu: &mut CPU) -> u8 {
    _cpu.inc_r16(Regs16::HL);
    2
}

fn inc_33(_cpu: &mut CPU) -> u8 {
    _cpu.inc_r16(Regs16::SP);
    2
}

fn inc_04(_cpu: &mut CPU) -> u8 {
    _cpu.inc_r8(Regs8::B);
    1
}

fn inc_14(_cpu: &mut CPU) -> u8 {
    _cpu.inc_r8(Regs8::D);
    1
}

fn inc_24(_cpu: &mut CPU) -> u8 {
    _cpu.inc_r8(Regs8::H);
    1
}

fn inc_34(_cpu: &mut CPU) -> u8 {
    _cpu.inc_r8(Regs8::HL);
    3
}

fn dec_05(_cpu: &mut CPU) -> u8 {
    _cpu.dec_r8(Regs8::B);
    1
}

fn dec_15(_cpu: &mut CPU) -> u8 {
    _cpu.dec_r8(Regs8::D);
    1
}

fn dec_25(_cpu: &mut CPU) -> u8 {
    _cpu.dec_r8(Regs8::H);
    1
}

fn dec_35(_cpu: &mut CPU) -> u8 {
    _cpu.dec_r8(Regs8::HL);
    3
}

fn dec_0b(_cpu: &mut CPU) -> u8 {
    _cpu.dec_r16(Regs16::BC);
    2
}

fn dec_1b(_cpu: &mut CPU) -> u8 {
    _cpu.dec_r16(Regs16::DE);
    2
}

fn dec_2b(_cpu: &mut CPU) -> u8 {
    _cpu.dec_r16(Regs16::HL);
    2
}

fn dec_3b(_cpu: &mut CPU) -> u8 {
    _cpu.dec_r16(Regs16::SP);
    2
}

fn inc_0c(_cpu: &mut CPU) -> u8 {
    _cpu.inc_r8(Regs8::C);
    1
}

fn inc_1c(_cpu: &mut CPU) -> u8 {
    _cpu.inc_r8(Regs8::E);
    1
}

fn inc_2c(_cpu: &mut CPU) -> u8 {
    _cpu.inc_r8(Regs8::L);
    1
}

fn inc_3c(_cpu: &mut CPU) -> u8 {
    _cpu.inc_r8(Regs8::A);
    1
}

fn dec_0d(_cpu: &mut CPU) -> u8 {
    _cpu.dec_r8(Regs8::C);
    1
}

fn dec_1d(_cpu: &mut CPU) -> u8 {
    _cpu.dec_r8(Regs8::E);
    1
}

fn dec_2d(_cpu: &mut CPU) -> u8 {
    _cpu.dec_r8(Regs8::L);
    1
}

fn dec_3d(_cpu: &mut CPU) -> u8 {
    _cpu.dec_r8(Regs8::A);
    1
}

// LD B,B
fn lod_40(_cpu: &mut CPU) -> u8 {
    1
}

// LD D,B
fn lod_50(_cpu: &mut CPU) -> u8 {
    let b = _cpu.get_r8(Regs8::B);
    _cpu.set_r8(Regs8::D, b);
    1
}

// LD H,B
fn lod_60(_cpu: &mut CPU) -> u8 {
    let b = _cpu.get_r8(Regs8::B);
    _cpu.set_r8(Regs8::H, b);
    1
}

// LD (HL),B
fn lod_70(_cpu: &mut CPU) -> u8 {
    let b = _cpu.get_r8(Regs8::B);
    _cpu.set_r8(Regs8::HL, b);
    2
}

// LD B,C
fn lod_41(_cpu: &mut CPU) -> u8 {
    let c = _cpu.get_r8(Regs8::C);
    _cpu.set_r8(Regs8::B, c);
    1
}

// LD D,C
fn lod_51(_cpu: &mut CPU) -> u8 {
    let c = _cpu.get_r8(Regs8::C);
    _cpu.set_r8(Regs8::D, c);
    1
}

// LD H,C
fn lod_61(_cpu: &mut CPU) -> u8 {
    let c = _cpu.get_r8(Regs8::C);
    _cpu.set_r8(Regs8::H, c);
    1
}

// LD (HL),C
fn lod_71(_cpu: &mut CPU) -> u8 {
    let c = _cpu.get_r8(Regs8::B);
    _cpu.set_r8(Regs8::HL, c);
    2
}

// LD B,D
fn lod_42(_cpu: &mut CPU) -> u8 {
    let d = _cpu.get_r8(Regs8::D);
    _cpu.set_r8(Regs8::B, d);
    1
}

// LD D,D
fn lod_52(_cpu: &mut CPU) -> u8 {
    1
}

// LD H,D
fn lod_62(_cpu: &mut CPU) -> u8 {
    let d = _cpu.get_r8(Regs8::D);
    _cpu.set_r8(Regs8::H, d);
    1
}

// LD (HL),D
fn lod_72(_cpu: &mut CPU) -> u8 {
    let d = _cpu.get_r8(Regs8::D);
    _cpu.set_r8(Regs8::HL, d);
    2
}

// LD B,E
fn lod_43(_cpu: &mut CPU) -> u8 {
    let e = _cpu.get_r8(Regs8::E);
    _cpu.set_r8(Regs8::B, e);
    1
}

// LD D,E
fn lod_53(_cpu: &mut CPU) -> u8 {
    let e = _cpu.get_r8(Regs8::E);
    _cpu.set_r8(Regs8::D, e);
}

// LD H,E
fn lod_63(_cpu: &mut CPU) -> u8 {
    let e = _cpu.get_r8(Regs8::E);
    _cpu.set_r8(Regs8::H, e);
    1
}

// LD (HL),E
fn lod_73(_cpu: &mut CPU) -> u8 {
    let e = _cpu.get_r8(Regs8::E);
    _cpu.set_r8(Regs8::HL, e);
    2
}

// LD B,H
fn lod_44(_cpu: &mut CPU) -> u8 {
    let h = _cpu.get_r8(Regs8::H);
    _cpu.set_r8(Regs8::B, h);
    1
}

// LD D,H
fn lod_54(_cpu: &mut CPU) -> u8 {
    let h = _cpu.get_r8(Regs8::H);
    _cpu.set_r8(Regs8::D, h);
}

// LD H,H
fn lod_64(_cpu: &mut CPU) -> u8 {
    1
}

// LD (HL),H
fn lod_74(_cpu: &mut CPU) -> u8 {
    let h = _cpu.get_r8(Regs8::H);
    _cpu.set_r8(Regs8::HL, h);
    2
}

// LD B,L
fn lod_45(_cpu: &mut CPU) -> u8 {
    let l = _cpu.get_r8(Regs8::L);
    _cpu.set_r8(Regs8::B, l);
    1
}

// LD D,L
fn lod_55(_cpu: &mut CPU) -> u8 {
    let l = _cpu.get_r8(Regs8::L);
    _cpu.set_r8(Regs8::D, l);
}

// LD H,L
fn lod_65(_cpu: &mut CPU) -> u8 {
    let l = _cpu.get_r8(Regs8::L);
    _cpu.set_r8(Regs8::H, l);
    1
}

// LD (HL),L
fn lod_75(_cpu: &mut CPU) -> u8 {
    let l = _cpu.get_r8(Regs8::L);
    _cpu.set_r8(Regs8::HL, l);
    2
}

