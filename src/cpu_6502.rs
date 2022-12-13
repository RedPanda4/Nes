use crate::bus::Bus;
use std::collections::HashMap;
use std::ops::Add;

#[non_exhaustive]
pub(crate) struct Flags6502;
impl Flags6502 {
    pub const C: u8 = (1 << 0);
    // Carry Bit
    pub const Z: u8 = (1 << 1);
    // Zero
    pub const I: u8 = (1 << 2);
    // Disable Interrupts
    pub const D: u8 = (1 << 3);
    // Decimal Mode (unused in this implementation)
    pub const B: u8 = (1 << 4);
    // Break
    pub const U: u8 = (1 << 5);
    // Unused
    pub const V: u8 = (1 << 6);
    // Overflow
    pub const N: u8 = (1 << 7);
    // Negative
}
// enum Flags6502
// {
//     C = (1 << 0),
//     // Carry Bit
//     Z = (1 << 1),
//     // Zero
//     I = (1 << 2),
//     // Disable Interrupts
//     D = (1 << 3),
//     // Decimal Mode (unused in this implementation)
//     B = (1 << 4),
//     // Break
//     U = (1 << 5),
//     // Unused
//     V = (1 << 6),
//     // Overflow
//     N = (1 << 7),    // Negative
// }

struct Instruction {
    name: String,
    operate: fn(&mut Cpu6502) -> u8,
    addresmode: fn(&mut Cpu6502) -> u8,
    cyles: u8,
}

pub(crate) struct Cpu6502 {
    // accumulator
    pub(crate) a: u8,
    // register X
    pub(crate) x: u8,
    // register y
    pub(crate) y: u8,
    // program counter
    pub(crate) pc: u16,
    // stack pointer
    pub(crate) sp: u8,
    // status register
    pub(crate) sr: u8,

    // Bus
    pub(crate) bus: Bus,

    // // Flags
    // flags: Flags6502,

    // Represents the working input value to the ALU
    fetched: u8,
    // A convenience variable used everywhere
    temp: u16,
    // All used memory addresses end up in here
    addr_abs: u16,
    // Represents absolute address following a branch
    addr_rel: u16,
    // Is the instruction byte
    opcode: u8,
    // Counts how many cycles the instruction has remaining
    cycles: u8,

    lookup: [Instruction; 256],
}

impl Cpu6502 {
    pub fn new() -> Self {
        // let loukup_table: Vec<Instruction> = vec![
        //     Instruction { name: "BRK".to_string(), operate: Cpu6502::brk, addresmode:Cpu6502::imm, cyles:7 }, Instruction { name: "ORA".to_string(), operate: Cpu6502::ora, addresmode:Cpu6502::izx, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:8 }, Instruction { name: "???".to_string(), operate: Cpu6502::nop, addresmode:Cpu6502::imp, cyles:3 }, Instruction { name: "ORA".to_string(), operate: Cpu6502::ora, addresmode:Cpu6502::zp0, cyles:3 }, Instruction { name: "ASL".to_string(), operate: Cpu6502::asl, addresmode:Cpu6502::zp0, cyles:5 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:5 }, Instruction { name: "PHP".to_string(), operate: Cpu6502::php, addresmode:Cpu6502::imp, cyles:3 }, Instruction { name: "ORA".to_string(), operate: Cpu6502::ora, addresmode:Cpu6502::imm, cyles:2 }, Instruction { name: "ASL".to_string(), operate: Cpu6502::asl, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::nop, addresmode:Cpu6502::imp, cyles:4 }, Instruction { name: "ORA".to_string(), operate: Cpu6502::ora, addresmode:Cpu6502::abs, cyles:4 }, Instruction { name: "ASL".to_string(), operate: Cpu6502::asl, addresmode:Cpu6502::abs, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:6 },
        //     Instruction { name: "BPL".to_string(), operate: Cpu6502::bpl, addresmode:Cpu6502::rel, cyles:2 }, Instruction { name: "ORA".to_string(), operate: Cpu6502::ora, addresmode:Cpu6502::izy, cyles:5 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:8 }, Instruction { name: "???".to_string(), operate: Cpu6502::nop, addresmode:Cpu6502::imp, cyles:4 }, Instruction { name: "ORA".to_string(), operate: Cpu6502::ora, addresmode:Cpu6502::zpx, cyles:4 }, Instruction { name: "ASL".to_string(), operate: Cpu6502::asl, addresmode:Cpu6502::zpx, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:6 }, Instruction { name: "CLC".to_string(), operate: Cpu6502::clc, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "ORA".to_string(), operate: Cpu6502::ora, addresmode:Cpu6502::aby, cyles:4 }, Instruction { name: "???".to_string(), operate: Cpu6502::nop, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:7 }, Instruction { name: "???".to_string(), operate: Cpu6502::nop, addresmode:Cpu6502::imp, cyles:4 }, Instruction { name: "ORA".to_string(), operate: Cpu6502::ora, addresmode:Cpu6502::abx, cyles:4 }, Instruction { name: "ASL".to_string(), operate: Cpu6502::asl, addresmode:Cpu6502::abx, cyles:7 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:7 },
        //     Instruction { name: "JSR".to_string(), operate: Cpu6502::jsr, addresmode:Cpu6502::abs, cyles:6 }, Instruction { name: "AND".to_string(), operate: Cpu6502::and, addresmode:Cpu6502::izx, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:8 }, Instruction { name: "BIT".to_string(), operate: Cpu6502::bit, addresmode:Cpu6502::zp0, cyles:3 }, Instruction { name: "AND".to_string(), operate: Cpu6502::and, addresmode:Cpu6502::zp0, cyles:3 }, Instruction { name: "ROL".to_string(), operate: Cpu6502::rol, addresmode:Cpu6502::zp0, cyles:5 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:5 }, Instruction { name: "PLP".to_string(), operate: Cpu6502::plp, addresmode:Cpu6502::imp, cyles:4 }, Instruction { name: "AND".to_string(), operate: Cpu6502::and, addresmode:Cpu6502::imm, cyles:2 }, Instruction { name: "ROL".to_string(), operate: Cpu6502::rol, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "BIT".to_string(), operate: Cpu6502::bit, addresmode:Cpu6502::abs, cyles:4 }, Instruction { name: "AND".to_string(), operate: Cpu6502::and, addresmode:Cpu6502::abs, cyles:4 }, Instruction { name: "ROL".to_string(), operate: Cpu6502::rol, addresmode:Cpu6502::abs, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:6 },
        //     Instruction { name: "BMI".to_string(), operate: Cpu6502::bmi, addresmode:Cpu6502::rel, cyles:2 }, Instruction { name: "AND".to_string(), operate: Cpu6502::and, addresmode:Cpu6502::izy, cyles:5 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:8 }, Instruction { name: "???".to_string(), operate: Cpu6502::nop, addresmode:Cpu6502::imp, cyles:4 }, Instruction { name: "AND".to_string(), operate: Cpu6502::and, addresmode:Cpu6502::zpx, cyles:4 }, Instruction { name: "ROL".to_string(), operate: Cpu6502::rol, addresmode:Cpu6502::zpx, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:6 }, Instruction { name: "SEC".to_string(), operate: Cpu6502::sec, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "AND".to_string(), operate: Cpu6502::and, addresmode:Cpu6502::aby, cyles:4 }, Instruction { name: "???".to_string(), operate: Cpu6502::nop, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:7 }, Instruction { name: "???".to_string(), operate: Cpu6502::nop, addresmode:Cpu6502::imp, cyles:4 }, Instruction { name: "AND".to_string(), operate: Cpu6502::and, addresmode:Cpu6502::abx, cyles:4 }, Instruction { name: "ROL".to_string(), operate: Cpu6502::rol, addresmode:Cpu6502::abx, cyles:7 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:7 },
        //     Instruction { name: "RTI".to_string(), operate: Cpu6502::rti, addresmode:Cpu6502::imp, cyles:6 }, Instruction { name: "EOR".to_string(), operate: Cpu6502::eor, addresmode:Cpu6502::izx, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:8 }, Instruction { name: "???".to_string(), operate: Cpu6502::nop, addresmode:Cpu6502::imp, cyles:3 }, Instruction { name: "EOR".to_string(), operate: Cpu6502::eor, addresmode:Cpu6502::zp0, cyles:3 }, Instruction { name: "LSR".to_string(), operate: Cpu6502::lsr, addresmode:Cpu6502::zp0, cyles:5 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:5 }, Instruction { name: "PHA".to_string(), operate: Cpu6502::pha, addresmode:Cpu6502::imp, cyles:3 }, Instruction { name: "EOR".to_string(), operate: Cpu6502::eor, addresmode:Cpu6502::imm, cyles:2 }, Instruction { name: "LSR".to_string(), operate: Cpu6502::lsr, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "JMP".to_string(), operate: Cpu6502::jmp, addresmode:Cpu6502::abs, cyles:3 }, Instruction { name: "EOR".to_string(), operate: Cpu6502::eor, addresmode:Cpu6502::abs, cyles:4 }, Instruction { name: "LSR".to_string(), operate: Cpu6502::lsr, addresmode:Cpu6502::abs, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:6 },
        //     Instruction { name: "BVC".to_string(), operate: Cpu6502::bvc, addresmode:Cpu6502::rel, cyles:2 }, Instruction { name: "EOR".to_string(), operate: Cpu6502::eor, addresmode:Cpu6502::izy, cyles:5 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:8 }, Instruction { name: "???".to_string(), operate: Cpu6502::nop, addresmode:Cpu6502::imp, cyles:4 }, Instruction { name: "EOR".to_string(), operate: Cpu6502::eor, addresmode:Cpu6502::zpx, cyles:4 }, Instruction { name: "LSR".to_string(), operate: Cpu6502::lsr, addresmode:Cpu6502::zpx, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:6 }, Instruction { name: "CLI".to_string(), operate: Cpu6502::cli, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "EOR".to_string(), operate: Cpu6502::eor, addresmode:Cpu6502::aby, cyles:4 }, Instruction { name: "???".to_string(), operate: Cpu6502::nop, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:7 }, Instruction { name: "???".to_string(), operate: Cpu6502::nop, addresmode:Cpu6502::imp, cyles:4 }, Instruction { name: "EOR".to_string(), operate: Cpu6502::eor, addresmode:Cpu6502::abx, cyles:4 }, Instruction { name: "LSR".to_string(), operate: Cpu6502::lsr, addresmode:Cpu6502::abx, cyles:7 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:7 },
        //     Instruction { name: "RTS".to_string(), operate: Cpu6502::rts, addresmode:Cpu6502::imp, cyles:6 }, Instruction { name: "ADC".to_string(), operate: Cpu6502::adc, addresmode:Cpu6502::izx, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:8 }, Instruction { name: "???".to_string(), operate: Cpu6502::nop, addresmode:Cpu6502::imp, cyles:3 }, Instruction { name: "ADC".to_string(), operate: Cpu6502::adc, addresmode:Cpu6502::zp0, cyles:3 }, Instruction { name: "ROR".to_string(), operate: Cpu6502::ror, addresmode:Cpu6502::zp0, cyles:5 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:5 }, Instruction { name: "PLA".to_string(), operate: Cpu6502::pla, addresmode:Cpu6502::imp, cyles:4 }, Instruction { name: "ADC".to_string(), operate: Cpu6502::adc, addresmode:Cpu6502::imm, cyles:2 }, Instruction { name: "ROR".to_string(), operate: Cpu6502::ror, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "JMP".to_string(), operate: Cpu6502::jmp, addresmode:Cpu6502::ind, cyles:5 }, Instruction { name: "ADC".to_string(), operate: Cpu6502::adc, addresmode:Cpu6502::abs, cyles:4 }, Instruction { name: "ROR".to_string(), operate: Cpu6502::ror, addresmode:Cpu6502::abs, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:6 },
        //     Instruction { name: "BVS".to_string(), operate: Cpu6502::bvs, addresmode:Cpu6502::rel, cyles:2 }, Instruction { name: "ADC".to_string(), operate: Cpu6502::adc, addresmode:Cpu6502::izy, cyles:5 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:8 }, Instruction { name: "???".to_string(), operate: Cpu6502::nop, addresmode:Cpu6502::imp, cyles:4 }, Instruction { name: "ADC".to_string(), operate: Cpu6502::adc, addresmode:Cpu6502::zpx, cyles:4 }, Instruction { name: "ROR".to_string(), operate: Cpu6502::ror, addresmode:Cpu6502::zpx, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:6 }, Instruction { name: "SEI".to_string(), operate: Cpu6502::sei, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "ADC".to_string(), operate: Cpu6502::adc, addresmode:Cpu6502::aby, cyles:4 }, Instruction { name: "???".to_string(), operate: Cpu6502::nop, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:7 }, Instruction { name: "???".to_string(), operate: Cpu6502::nop, addresmode:Cpu6502::imp, cyles:4 }, Instruction { name: "ADC".to_string(), operate: Cpu6502::adc, addresmode:Cpu6502::abx, cyles:4 }, Instruction { name: "ROR".to_string(), operate: Cpu6502::ror, addresmode:Cpu6502::abx, cyles:7 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:7 },
        //     Instruction { name: "???".to_string(), operate: Cpu6502::nop, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "STA".to_string(), operate: Cpu6502::sta, addresmode:Cpu6502::izx, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::nop, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:6 }, Instruction { name: "STY".to_string(), operate: Cpu6502::sty, addresmode:Cpu6502::zp0, cyles:3 }, Instruction { name: "STA".to_string(), operate: Cpu6502::sta, addresmode:Cpu6502::zp0, cyles:3 }, Instruction { name: "STX".to_string(), operate: Cpu6502::stx, addresmode:Cpu6502::zp0, cyles:3 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:3 }, Instruction { name: "DEY".to_string(), operate: Cpu6502::dey, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::nop, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "TXA".to_string(), operate: Cpu6502::txa, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "STY".to_string(), operate: Cpu6502::sty, addresmode:Cpu6502::abs, cyles:4 }, Instruction { name: "STA".to_string(), operate: Cpu6502::sta, addresmode:Cpu6502::abs, cyles:4 }, Instruction { name: "STX".to_string(), operate: Cpu6502::stx, addresmode:Cpu6502::abs, cyles:4 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:4 },
        //     Instruction { name: "BCC".to_string(), operate: Cpu6502::bcc, addresmode:Cpu6502::rel, cyles:2 }, Instruction { name: "STA".to_string(), operate: Cpu6502::sta, addresmode:Cpu6502::izy, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:6 }, Instruction { name: "STY".to_string(), operate: Cpu6502::sty, addresmode:Cpu6502::zpx, cyles:4 }, Instruction { name: "STA".to_string(), operate: Cpu6502::sta, addresmode:Cpu6502::zpx, cyles:4 }, Instruction { name: "STX".to_string(), operate: Cpu6502::stx, addresmode:Cpu6502::zpy, cyles:4 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:4 }, Instruction { name: "TYA".to_string(), operate: Cpu6502::tya, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "STA".to_string(), operate: Cpu6502::sta, addresmode:Cpu6502::aby, cyles:5 }, Instruction { name: "TXS".to_string(), operate: Cpu6502::txs, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:5 }, Instruction { name: "???".to_string(), operate: Cpu6502::nop, addresmode:Cpu6502::imp, cyles:5 }, Instruction { name: "STA".to_string(), operate: Cpu6502::sta, addresmode:Cpu6502::abx, cyles:5 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:5 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:5 },
        //     Instruction { name: "LDY".to_string(), operate: Cpu6502::ldy, addresmode:Cpu6502::imm, cyles:2 }, Instruction { name: "LDA".to_string(), operate: Cpu6502::lda, addresmode:Cpu6502::izx, cyles:6 }, Instruction { name: "LDX".to_string(), operate: Cpu6502::ldx, addresmode:Cpu6502::imm, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:6 }, Instruction { name: "LDY".to_string(), operate: Cpu6502::ldy, addresmode:Cpu6502::zp0, cyles:3 }, Instruction { name: "LDA".to_string(), operate: Cpu6502::lda, addresmode:Cpu6502::zp0, cyles:3 }, Instruction { name: "LDX".to_string(), operate: Cpu6502::ldx, addresmode:Cpu6502::zp0, cyles:3 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:3 }, Instruction { name: "TAY".to_string(), operate: Cpu6502::tay, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "LDA".to_string(), operate: Cpu6502::lda, addresmode:Cpu6502::imm, cyles:2 }, Instruction { name: "TAX".to_string(), operate: Cpu6502::tax, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "LDY".to_string(), operate: Cpu6502::ldy, addresmode:Cpu6502::abs, cyles:4 }, Instruction { name: "LDA".to_string(), operate: Cpu6502::lda, addresmode:Cpu6502::abs, cyles:4 }, Instruction { name: "LDX".to_string(), operate: Cpu6502::ldx, addresmode:Cpu6502::abs, cyles:4 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:4 },
        //     Instruction { name: "BCS".to_string(), operate: Cpu6502::bcs, addresmode:Cpu6502::rel, cyles:2 }, Instruction { name: "LDA".to_string(), operate: Cpu6502::lda, addresmode:Cpu6502::izy, cyles:5 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:5 }, Instruction { name: "LDY".to_string(), operate: Cpu6502::ldy, addresmode:Cpu6502::zpx, cyles:4 }, Instruction { name: "LDA".to_string(), operate: Cpu6502::lda, addresmode:Cpu6502::zpx, cyles:4 }, Instruction { name: "LDX".to_string(), operate: Cpu6502::ldx, addresmode:Cpu6502::zpy, cyles:4 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:4 }, Instruction { name: "CLV".to_string(), operate: Cpu6502::clv, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "LDA".to_string(), operate: Cpu6502::lda, addresmode:Cpu6502::aby, cyles:4 }, Instruction { name: "TSX".to_string(), operate: Cpu6502::tsx, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:4 }, Instruction { name: "LDY".to_string(), operate: Cpu6502::ldy, addresmode:Cpu6502::abx, cyles:4 }, Instruction { name: "LDA".to_string(), operate: Cpu6502::lda, addresmode:Cpu6502::abx, cyles:4 }, Instruction { name: "LDX".to_string(), operate: Cpu6502::ldx, addresmode:Cpu6502::aby, cyles:4 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:4 },
        //     Instruction { name: "CPY".to_string(), operate: Cpu6502::cpy, addresmode:Cpu6502::imm, cyles:2 }, Instruction { name: "CMP".to_string(), operate: Cpu6502::cmp, addresmode:Cpu6502::izx, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::nop, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:8 }, Instruction { name: "CPY".to_string(), operate: Cpu6502::cpy, addresmode:Cpu6502::zp0, cyles:3 }, Instruction { name: "CMP".to_string(), operate: Cpu6502::cmp, addresmode:Cpu6502::zp0, cyles:3 }, Instruction { name: "DEC".to_string(), operate: Cpu6502::dec, addresmode:Cpu6502::zp0, cyles:5 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:5 }, Instruction { name: "INY".to_string(), operate: Cpu6502::iny, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "CMP".to_string(), operate: Cpu6502::cmp, addresmode:Cpu6502::imm, cyles:2 }, Instruction { name: "DEX".to_string(), operate: Cpu6502::dex, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "CPY".to_string(), operate: Cpu6502::cpy, addresmode:Cpu6502::abs, cyles:4 }, Instruction { name: "CMP".to_string(), operate: Cpu6502::cmp, addresmode:Cpu6502::abs, cyles:4 }, Instruction { name: "DEC".to_string(), operate: Cpu6502::dec, addresmode:Cpu6502::abs, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:6 },
        //     Instruction { name: "BNE".to_string(), operate: Cpu6502::bne, addresmode:Cpu6502::rel, cyles:2 }, Instruction { name: "CMP".to_string(), operate: Cpu6502::cmp, addresmode:Cpu6502::izy, cyles:5 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:8 }, Instruction { name: "???".to_string(), operate: Cpu6502::nop, addresmode:Cpu6502::imp, cyles:4 }, Instruction { name: "CMP".to_string(), operate: Cpu6502::cmp, addresmode:Cpu6502::zpx, cyles:4 }, Instruction { name: "DEC".to_string(), operate: Cpu6502::dec, addresmode:Cpu6502::zpx, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:6 }, Instruction { name: "CLD".to_string(), operate: Cpu6502::cld, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "CMP".to_string(), operate: Cpu6502::cmp, addresmode:Cpu6502::aby, cyles:4 }, Instruction { name: "NOP".to_string(), operate: Cpu6502::nop, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:7 }, Instruction { name: "???".to_string(), operate: Cpu6502::nop, addresmode:Cpu6502::imp, cyles:4 }, Instruction { name: "CMP".to_string(), operate: Cpu6502::cmp, addresmode:Cpu6502::abx, cyles:4 }, Instruction { name: "DEC".to_string(), operate: Cpu6502::dec, addresmode:Cpu6502::abx, cyles:7 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:7 },
        //     Instruction { name: "CPX".to_string(), operate: Cpu6502::cpx, addresmode:Cpu6502::imm, cyles:2 }, Instruction { name: "SBC".to_string(), operate: Cpu6502::sbc, addresmode:Cpu6502::izx, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::nop, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:8 }, Instruction { name: "CPX".to_string(), operate: Cpu6502::cpx, addresmode:Cpu6502::zp0, cyles:3 }, Instruction { name: "SBC".to_string(), operate: Cpu6502::sbc, addresmode:Cpu6502::zp0, cyles:3 }, Instruction { name: "INC".to_string(), operate: Cpu6502::inc, addresmode:Cpu6502::zp0, cyles:5 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:5 }, Instruction { name: "INX".to_string(), operate: Cpu6502::inx, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "SBC".to_string(), operate: Cpu6502::sbc, addresmode:Cpu6502::imm, cyles:2 }, Instruction { name: "NOP".to_string(), operate: Cpu6502::nop, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::sbc, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "CPX".to_string(), operate: Cpu6502::cpx, addresmode:Cpu6502::abs, cyles:4 }, Instruction { name: "SBC".to_string(), operate: Cpu6502::sbc, addresmode:Cpu6502::abs, cyles:4 }, Instruction { name: "INC".to_string(), operate: Cpu6502::inc, addresmode:Cpu6502::abs, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:6 },
        //     Instruction { name: "BEQ".to_string(), operate: Cpu6502::beq, addresmode:Cpu6502::rel, cyles:2 }, Instruction { name: "SBC".to_string(), operate: Cpu6502::sbc, addresmode:Cpu6502::izy, cyles:5 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:8 }, Instruction { name: "???".to_string(), operate: Cpu6502::nop, addresmode:Cpu6502::imp, cyles:4 }, Instruction { name: "SBC".to_string(), operate: Cpu6502::sbc, addresmode:Cpu6502::zpx, cyles:4 }, Instruction { name: "INC".to_string(), operate: Cpu6502::inc, addresmode:Cpu6502::zpx, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:6 }, Instruction { name: "SED".to_string(), operate: Cpu6502::sed, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "SBC".to_string(), operate: Cpu6502::sbc, addresmode:Cpu6502::aby, cyles:4 }, Instruction { name: "NOP".to_string(), operate: Cpu6502::nop, addresmode:Cpu6502::imp, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:7 }, Instruction { name: "???".to_string(), operate: Cpu6502::nop, addresmode:Cpu6502::imp, cyles:4 }, Instruction { name: "SBC".to_string(), operate: Cpu6502::sbc, addresmode:Cpu6502::abx, cyles:4 }, Instruction { name: "INC".to_string(), operate: Cpu6502::inc, addresmode:Cpu6502::abx, cyles:7 }, Instruction { name: "???".to_string(), operate: Cpu6502::xxx, addresmode:Cpu6502::imp, cyles:7 },
        // ];

        let loukup_table: [Instruction; 256] = [
            Instruction {
                name: "BRK".to_string(),
                operate: Cpu6502::brk,
                addresmode: Cpu6502::imm,
                cyles: 7,
            },
            Instruction {
                name: "ORA".to_string(),
                operate: Cpu6502::ora,
                addresmode: Cpu6502::izx,
                cyles: 6,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 8,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::nop,
                addresmode: Cpu6502::imp,
                cyles: 3,
            },
            Instruction {
                name: "ORA".to_string(),
                operate: Cpu6502::ora,
                addresmode: Cpu6502::zp0,
                cyles: 3,
            },
            Instruction {
                name: "ASL".to_string(),
                operate: Cpu6502::asl,
                addresmode: Cpu6502::zp0,
                cyles: 5,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 5,
            },
            Instruction {
                name: "PHP".to_string(),
                operate: Cpu6502::php,
                addresmode: Cpu6502::imp,
                cyles: 3,
            },
            Instruction {
                name: "ORA".to_string(),
                operate: Cpu6502::ora,
                addresmode: Cpu6502::imm,
                cyles: 2,
            },
            Instruction {
                name: "ASL".to_string(),
                operate: Cpu6502::asl,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::nop,
                addresmode: Cpu6502::imp,
                cyles: 4,
            },
            Instruction {
                name: "ORA".to_string(),
                operate: Cpu6502::ora,
                addresmode: Cpu6502::abs,
                cyles: 4,
            },
            Instruction {
                name: "ASL".to_string(),
                operate: Cpu6502::asl,
                addresmode: Cpu6502::abs,
                cyles: 6,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 6,
            },
            Instruction {
                name: "BPL".to_string(),
                operate: Cpu6502::bpl,
                addresmode: Cpu6502::rel,
                cyles: 2,
            },
            Instruction {
                name: "ORA".to_string(),
                operate: Cpu6502::ora,
                addresmode: Cpu6502::izy,
                cyles: 5,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 8,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::nop,
                addresmode: Cpu6502::imp,
                cyles: 4,
            },
            Instruction {
                name: "ORA".to_string(),
                operate: Cpu6502::ora,
                addresmode: Cpu6502::zpx,
                cyles: 4,
            },
            Instruction {
                name: "ASL".to_string(),
                operate: Cpu6502::asl,
                addresmode: Cpu6502::zpx,
                cyles: 6,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 6,
            },
            Instruction {
                name: "CLC".to_string(),
                operate: Cpu6502::clc,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "ORA".to_string(),
                operate: Cpu6502::ora,
                addresmode: Cpu6502::aby,
                cyles: 4,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::nop,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 7,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::nop,
                addresmode: Cpu6502::imp,
                cyles: 4,
            },
            Instruction {
                name: "ORA".to_string(),
                operate: Cpu6502::ora,
                addresmode: Cpu6502::abx,
                cyles: 4,
            },
            Instruction {
                name: "ASL".to_string(),
                operate: Cpu6502::asl,
                addresmode: Cpu6502::abx,
                cyles: 7,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 7,
            },
            Instruction {
                name: "JSR".to_string(),
                operate: Cpu6502::jsr,
                addresmode: Cpu6502::abs,
                cyles: 6,
            },
            Instruction {
                name: "AND".to_string(),
                operate: Cpu6502::and,
                addresmode: Cpu6502::izx,
                cyles: 6,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 8,
            },
            Instruction {
                name: "BIT".to_string(),
                operate: Cpu6502::bit,
                addresmode: Cpu6502::zp0,
                cyles: 3,
            },
            Instruction {
                name: "AND".to_string(),
                operate: Cpu6502::and,
                addresmode: Cpu6502::zp0,
                cyles: 3,
            },
            Instruction {
                name: "ROL".to_string(),
                operate: Cpu6502::rol,
                addresmode: Cpu6502::zp0,
                cyles: 5,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 5,
            },
            Instruction {
                name: "PLP".to_string(),
                operate: Cpu6502::plp,
                addresmode: Cpu6502::imp,
                cyles: 4,
            },
            Instruction {
                name: "AND".to_string(),
                operate: Cpu6502::and,
                addresmode: Cpu6502::imm,
                cyles: 2,
            },
            Instruction {
                name: "ROL".to_string(),
                operate: Cpu6502::rol,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "BIT".to_string(),
                operate: Cpu6502::bit,
                addresmode: Cpu6502::abs,
                cyles: 4,
            },
            Instruction {
                name: "AND".to_string(),
                operate: Cpu6502::and,
                addresmode: Cpu6502::abs,
                cyles: 4,
            },
            Instruction {
                name: "ROL".to_string(),
                operate: Cpu6502::rol,
                addresmode: Cpu6502::abs,
                cyles: 6,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 6,
            },
            Instruction {
                name: "BMI".to_string(),
                operate: Cpu6502::bmi,
                addresmode: Cpu6502::rel,
                cyles: 2,
            },
            Instruction {
                name: "AND".to_string(),
                operate: Cpu6502::and,
                addresmode: Cpu6502::izy,
                cyles: 5,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 8,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::nop,
                addresmode: Cpu6502::imp,
                cyles: 4,
            },
            Instruction {
                name: "AND".to_string(),
                operate: Cpu6502::and,
                addresmode: Cpu6502::zpx,
                cyles: 4,
            },
            Instruction {
                name: "ROL".to_string(),
                operate: Cpu6502::rol,
                addresmode: Cpu6502::zpx,
                cyles: 6,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 6,
            },
            Instruction {
                name: "SEC".to_string(),
                operate: Cpu6502::sec,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "AND".to_string(),
                operate: Cpu6502::and,
                addresmode: Cpu6502::aby,
                cyles: 4,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::nop,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 7,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::nop,
                addresmode: Cpu6502::imp,
                cyles: 4,
            },
            Instruction {
                name: "AND".to_string(),
                operate: Cpu6502::and,
                addresmode: Cpu6502::abx,
                cyles: 4,
            },
            Instruction {
                name: "ROL".to_string(),
                operate: Cpu6502::rol,
                addresmode: Cpu6502::abx,
                cyles: 7,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 7,
            },
            Instruction {
                name: "RTI".to_string(),
                operate: Cpu6502::rti,
                addresmode: Cpu6502::imp,
                cyles: 6,
            },
            Instruction {
                name: "EOR".to_string(),
                operate: Cpu6502::eor,
                addresmode: Cpu6502::izx,
                cyles: 6,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 8,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::nop,
                addresmode: Cpu6502::imp,
                cyles: 3,
            },
            Instruction {
                name: "EOR".to_string(),
                operate: Cpu6502::eor,
                addresmode: Cpu6502::zp0,
                cyles: 3,
            },
            Instruction {
                name: "LSR".to_string(),
                operate: Cpu6502::lsr,
                addresmode: Cpu6502::zp0,
                cyles: 5,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 5,
            },
            Instruction {
                name: "PHA".to_string(),
                operate: Cpu6502::pha,
                addresmode: Cpu6502::imp,
                cyles: 3,
            },
            Instruction {
                name: "EOR".to_string(),
                operate: Cpu6502::eor,
                addresmode: Cpu6502::imm,
                cyles: 2,
            },
            Instruction {
                name: "LSR".to_string(),
                operate: Cpu6502::lsr,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "JMP".to_string(),
                operate: Cpu6502::jmp,
                addresmode: Cpu6502::abs,
                cyles: 3,
            },
            Instruction {
                name: "EOR".to_string(),
                operate: Cpu6502::eor,
                addresmode: Cpu6502::abs,
                cyles: 4,
            },
            Instruction {
                name: "LSR".to_string(),
                operate: Cpu6502::lsr,
                addresmode: Cpu6502::abs,
                cyles: 6,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 6,
            },
            Instruction {
                name: "BVC".to_string(),
                operate: Cpu6502::bvc,
                addresmode: Cpu6502::rel,
                cyles: 2,
            },
            Instruction {
                name: "EOR".to_string(),
                operate: Cpu6502::eor,
                addresmode: Cpu6502::izy,
                cyles: 5,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 8,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::nop,
                addresmode: Cpu6502::imp,
                cyles: 4,
            },
            Instruction {
                name: "EOR".to_string(),
                operate: Cpu6502::eor,
                addresmode: Cpu6502::zpx,
                cyles: 4,
            },
            Instruction {
                name: "LSR".to_string(),
                operate: Cpu6502::lsr,
                addresmode: Cpu6502::zpx,
                cyles: 6,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 6,
            },
            Instruction {
                name: "CLI".to_string(),
                operate: Cpu6502::cli,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "EOR".to_string(),
                operate: Cpu6502::eor,
                addresmode: Cpu6502::aby,
                cyles: 4,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::nop,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 7,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::nop,
                addresmode: Cpu6502::imp,
                cyles: 4,
            },
            Instruction {
                name: "EOR".to_string(),
                operate: Cpu6502::eor,
                addresmode: Cpu6502::abx,
                cyles: 4,
            },
            Instruction {
                name: "LSR".to_string(),
                operate: Cpu6502::lsr,
                addresmode: Cpu6502::abx,
                cyles: 7,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 7,
            },
            Instruction {
                name: "RTS".to_string(),
                operate: Cpu6502::rts,
                addresmode: Cpu6502::imp,
                cyles: 6,
            },
            Instruction {
                name: "ADC".to_string(),
                operate: Cpu6502::adc,
                addresmode: Cpu6502::izx,
                cyles: 6,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 8,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::nop,
                addresmode: Cpu6502::imp,
                cyles: 3,
            },
            Instruction {
                name: "ADC".to_string(),
                operate: Cpu6502::adc,
                addresmode: Cpu6502::zp0,
                cyles: 3,
            },
            Instruction {
                name: "ROR".to_string(),
                operate: Cpu6502::ror,
                addresmode: Cpu6502::zp0,
                cyles: 5,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 5,
            },
            Instruction {
                name: "PLA".to_string(),
                operate: Cpu6502::pla,
                addresmode: Cpu6502::imp,
                cyles: 4,
            },
            Instruction {
                name: "ADC".to_string(),
                operate: Cpu6502::adc,
                addresmode: Cpu6502::imm,
                cyles: 2,
            },
            Instruction {
                name: "ROR".to_string(),
                operate: Cpu6502::ror,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "JMP".to_string(),
                operate: Cpu6502::jmp,
                addresmode: Cpu6502::ind,
                cyles: 5,
            },
            Instruction {
                name: "ADC".to_string(),
                operate: Cpu6502::adc,
                addresmode: Cpu6502::abs,
                cyles: 4,
            },
            Instruction {
                name: "ROR".to_string(),
                operate: Cpu6502::ror,
                addresmode: Cpu6502::abs,
                cyles: 6,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 6,
            },
            Instruction {
                name: "BVS".to_string(),
                operate: Cpu6502::bvs,
                addresmode: Cpu6502::rel,
                cyles: 2,
            },
            Instruction {
                name: "ADC".to_string(),
                operate: Cpu6502::adc,
                addresmode: Cpu6502::izy,
                cyles: 5,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 8,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::nop,
                addresmode: Cpu6502::imp,
                cyles: 4,
            },
            Instruction {
                name: "ADC".to_string(),
                operate: Cpu6502::adc,
                addresmode: Cpu6502::zpx,
                cyles: 4,
            },
            Instruction {
                name: "ROR".to_string(),
                operate: Cpu6502::ror,
                addresmode: Cpu6502::zpx,
                cyles: 6,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 6,
            },
            Instruction {
                name: "SEI".to_string(),
                operate: Cpu6502::sei,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "ADC".to_string(),
                operate: Cpu6502::adc,
                addresmode: Cpu6502::aby,
                cyles: 4,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::nop,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 7,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::nop,
                addresmode: Cpu6502::imp,
                cyles: 4,
            },
            Instruction {
                name: "ADC".to_string(),
                operate: Cpu6502::adc,
                addresmode: Cpu6502::abx,
                cyles: 4,
            },
            Instruction {
                name: "ROR".to_string(),
                operate: Cpu6502::ror,
                addresmode: Cpu6502::abx,
                cyles: 7,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 7,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::nop,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "STA".to_string(),
                operate: Cpu6502::sta,
                addresmode: Cpu6502::izx,
                cyles: 6,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::nop,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 6,
            },
            Instruction {
                name: "STY".to_string(),
                operate: Cpu6502::sty,
                addresmode: Cpu6502::zp0,
                cyles: 3,
            },
            Instruction {
                name: "STA".to_string(),
                operate: Cpu6502::sta,
                addresmode: Cpu6502::zp0,
                cyles: 3,
            },
            Instruction {
                name: "STX".to_string(),
                operate: Cpu6502::stx,
                addresmode: Cpu6502::zp0,
                cyles: 3,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 3,
            },
            Instruction {
                name: "DEY".to_string(),
                operate: Cpu6502::dey,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::nop,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "TXA".to_string(),
                operate: Cpu6502::txa,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "STY".to_string(),
                operate: Cpu6502::sty,
                addresmode: Cpu6502::abs,
                cyles: 4,
            },
            Instruction {
                name: "STA".to_string(),
                operate: Cpu6502::sta,
                addresmode: Cpu6502::abs,
                cyles: 4,
            },
            Instruction {
                name: "STX".to_string(),
                operate: Cpu6502::stx,
                addresmode: Cpu6502::abs,
                cyles: 4,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 4,
            },
            Instruction {
                name: "BCC".to_string(),
                operate: Cpu6502::bcc,
                addresmode: Cpu6502::rel,
                cyles: 2,
            },
            Instruction {
                name: "STA".to_string(),
                operate: Cpu6502::sta,
                addresmode: Cpu6502::izy,
                cyles: 6,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 6,
            },
            Instruction {
                name: "STY".to_string(),
                operate: Cpu6502::sty,
                addresmode: Cpu6502::zpx,
                cyles: 4,
            },
            Instruction {
                name: "STA".to_string(),
                operate: Cpu6502::sta,
                addresmode: Cpu6502::zpx,
                cyles: 4,
            },
            Instruction {
                name: "STX".to_string(),
                operate: Cpu6502::stx,
                addresmode: Cpu6502::zpy,
                cyles: 4,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 4,
            },
            Instruction {
                name: "TYA".to_string(),
                operate: Cpu6502::tya,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "STA".to_string(),
                operate: Cpu6502::sta,
                addresmode: Cpu6502::aby,
                cyles: 5,
            },
            Instruction {
                name: "TXS".to_string(),
                operate: Cpu6502::txs,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 5,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::nop,
                addresmode: Cpu6502::imp,
                cyles: 5,
            },
            Instruction {
                name: "STA".to_string(),
                operate: Cpu6502::sta,
                addresmode: Cpu6502::abx,
                cyles: 5,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 5,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 5,
            },
            Instruction {
                name: "LDY".to_string(),
                operate: Cpu6502::ldy,
                addresmode: Cpu6502::imm,
                cyles: 2,
            },
            Instruction {
                name: "LDA".to_string(),
                operate: Cpu6502::lda,
                addresmode: Cpu6502::izx,
                cyles: 6,
            },
            Instruction {
                name: "LDX".to_string(),
                operate: Cpu6502::ldx,
                addresmode: Cpu6502::imm,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 6,
            },
            Instruction {
                name: "LDY".to_string(),
                operate: Cpu6502::ldy,
                addresmode: Cpu6502::zp0,
                cyles: 3,
            },
            Instruction {
                name: "LDA".to_string(),
                operate: Cpu6502::lda,
                addresmode: Cpu6502::zp0,
                cyles: 3,
            },
            Instruction {
                name: "LDX".to_string(),
                operate: Cpu6502::ldx,
                addresmode: Cpu6502::zp0,
                cyles: 3,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 3,
            },
            Instruction {
                name: "TAY".to_string(),
                operate: Cpu6502::tay,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "LDA".to_string(),
                operate: Cpu6502::lda,
                addresmode: Cpu6502::imm,
                cyles: 2,
            },
            Instruction {
                name: "TAX".to_string(),
                operate: Cpu6502::tax,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "LDY".to_string(),
                operate: Cpu6502::ldy,
                addresmode: Cpu6502::abs,
                cyles: 4,
            },
            Instruction {
                name: "LDA".to_string(),
                operate: Cpu6502::lda,
                addresmode: Cpu6502::abs,
                cyles: 4,
            },
            Instruction {
                name: "LDX".to_string(),
                operate: Cpu6502::ldx,
                addresmode: Cpu6502::abs,
                cyles: 4,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 4,
            },
            Instruction {
                name: "BCS".to_string(),
                operate: Cpu6502::bcs,
                addresmode: Cpu6502::rel,
                cyles: 2,
            },
            Instruction {
                name: "LDA".to_string(),
                operate: Cpu6502::lda,
                addresmode: Cpu6502::izy,
                cyles: 5,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 5,
            },
            Instruction {
                name: "LDY".to_string(),
                operate: Cpu6502::ldy,
                addresmode: Cpu6502::zpx,
                cyles: 4,
            },
            Instruction {
                name: "LDA".to_string(),
                operate: Cpu6502::lda,
                addresmode: Cpu6502::zpx,
                cyles: 4,
            },
            Instruction {
                name: "LDX".to_string(),
                operate: Cpu6502::ldx,
                addresmode: Cpu6502::zpy,
                cyles: 4,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 4,
            },
            Instruction {
                name: "CLV".to_string(),
                operate: Cpu6502::clv,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "LDA".to_string(),
                operate: Cpu6502::lda,
                addresmode: Cpu6502::aby,
                cyles: 4,
            },
            Instruction {
                name: "TSX".to_string(),
                operate: Cpu6502::tsx,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 4,
            },
            Instruction {
                name: "LDY".to_string(),
                operate: Cpu6502::ldy,
                addresmode: Cpu6502::abx,
                cyles: 4,
            },
            Instruction {
                name: "LDA".to_string(),
                operate: Cpu6502::lda,
                addresmode: Cpu6502::abx,
                cyles: 4,
            },
            Instruction {
                name: "LDX".to_string(),
                operate: Cpu6502::ldx,
                addresmode: Cpu6502::aby,
                cyles: 4,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 4,
            },
            Instruction {
                name: "CPY".to_string(),
                operate: Cpu6502::cpy,
                addresmode: Cpu6502::imm,
                cyles: 2,
            },
            Instruction {
                name: "CMP".to_string(),
                operate: Cpu6502::cmp,
                addresmode: Cpu6502::izx,
                cyles: 6,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::nop,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 8,
            },
            Instruction {
                name: "CPY".to_string(),
                operate: Cpu6502::cpy,
                addresmode: Cpu6502::zp0,
                cyles: 3,
            },
            Instruction {
                name: "CMP".to_string(),
                operate: Cpu6502::cmp,
                addresmode: Cpu6502::zp0,
                cyles: 3,
            },
            Instruction {
                name: "DEC".to_string(),
                operate: Cpu6502::dec,
                addresmode: Cpu6502::zp0,
                cyles: 5,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 5,
            },
            Instruction {
                name: "INY".to_string(),
                operate: Cpu6502::iny,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "CMP".to_string(),
                operate: Cpu6502::cmp,
                addresmode: Cpu6502::imm,
                cyles: 2,
            },
            Instruction {
                name: "DEX".to_string(),
                operate: Cpu6502::dex,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "CPY".to_string(),
                operate: Cpu6502::cpy,
                addresmode: Cpu6502::abs,
                cyles: 4,
            },
            Instruction {
                name: "CMP".to_string(),
                operate: Cpu6502::cmp,
                addresmode: Cpu6502::abs,
                cyles: 4,
            },
            Instruction {
                name: "DEC".to_string(),
                operate: Cpu6502::dec,
                addresmode: Cpu6502::abs,
                cyles: 6,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 6,
            },
            Instruction {
                name: "BNE".to_string(),
                operate: Cpu6502::bne,
                addresmode: Cpu6502::rel,
                cyles: 2,
            },
            Instruction {
                name: "CMP".to_string(),
                operate: Cpu6502::cmp,
                addresmode: Cpu6502::izy,
                cyles: 5,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 8,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::nop,
                addresmode: Cpu6502::imp,
                cyles: 4,
            },
            Instruction {
                name: "CMP".to_string(),
                operate: Cpu6502::cmp,
                addresmode: Cpu6502::zpx,
                cyles: 4,
            },
            Instruction {
                name: "DEC".to_string(),
                operate: Cpu6502::dec,
                addresmode: Cpu6502::zpx,
                cyles: 6,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 6,
            },
            Instruction {
                name: "CLD".to_string(),
                operate: Cpu6502::cld,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "CMP".to_string(),
                operate: Cpu6502::cmp,
                addresmode: Cpu6502::aby,
                cyles: 4,
            },
            Instruction {
                name: "NOP".to_string(),
                operate: Cpu6502::nop,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 7,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::nop,
                addresmode: Cpu6502::imp,
                cyles: 4,
            },
            Instruction {
                name: "CMP".to_string(),
                operate: Cpu6502::cmp,
                addresmode: Cpu6502::abx,
                cyles: 4,
            },
            Instruction {
                name: "DEC".to_string(),
                operate: Cpu6502::dec,
                addresmode: Cpu6502::abx,
                cyles: 7,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 7,
            },
            Instruction {
                name: "CPX".to_string(),
                operate: Cpu6502::cpx,
                addresmode: Cpu6502::imm,
                cyles: 2,
            },
            Instruction {
                name: "SBC".to_string(),
                operate: Cpu6502::sbc,
                addresmode: Cpu6502::izx,
                cyles: 6,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::nop,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 8,
            },
            Instruction {
                name: "CPX".to_string(),
                operate: Cpu6502::cpx,
                addresmode: Cpu6502::zp0,
                cyles: 3,
            },
            Instruction {
                name: "SBC".to_string(),
                operate: Cpu6502::sbc,
                addresmode: Cpu6502::zp0,
                cyles: 3,
            },
            Instruction {
                name: "INC".to_string(),
                operate: Cpu6502::inc,
                addresmode: Cpu6502::zp0,
                cyles: 5,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 5,
            },
            Instruction {
                name: "INX".to_string(),
                operate: Cpu6502::inx,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "SBC".to_string(),
                operate: Cpu6502::sbc,
                addresmode: Cpu6502::imm,
                cyles: 2,
            },
            Instruction {
                name: "NOP".to_string(),
                operate: Cpu6502::nop,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::sbc,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "CPX".to_string(),
                operate: Cpu6502::cpx,
                addresmode: Cpu6502::abs,
                cyles: 4,
            },
            Instruction {
                name: "SBC".to_string(),
                operate: Cpu6502::sbc,
                addresmode: Cpu6502::abs,
                cyles: 4,
            },
            Instruction {
                name: "INC".to_string(),
                operate: Cpu6502::inc,
                addresmode: Cpu6502::abs,
                cyles: 6,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 6,
            },
            Instruction {
                name: "BEQ".to_string(),
                operate: Cpu6502::beq,
                addresmode: Cpu6502::rel,
                cyles: 2,
            },
            Instruction {
                name: "SBC".to_string(),
                operate: Cpu6502::sbc,
                addresmode: Cpu6502::izy,
                cyles: 5,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 8,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::nop,
                addresmode: Cpu6502::imp,
                cyles: 4,
            },
            Instruction {
                name: "SBC".to_string(),
                operate: Cpu6502::sbc,
                addresmode: Cpu6502::zpx,
                cyles: 4,
            },
            Instruction {
                name: "INC".to_string(),
                operate: Cpu6502::inc,
                addresmode: Cpu6502::zpx,
                cyles: 6,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 6,
            },
            Instruction {
                name: "SED".to_string(),
                operate: Cpu6502::sed,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "SBC".to_string(),
                operate: Cpu6502::sbc,
                addresmode: Cpu6502::aby,
                cyles: 4,
            },
            Instruction {
                name: "NOP".to_string(),
                operate: Cpu6502::nop,
                addresmode: Cpu6502::imp,
                cyles: 2,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 7,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::nop,
                addresmode: Cpu6502::imp,
                cyles: 4,
            },
            Instruction {
                name: "SBC".to_string(),
                operate: Cpu6502::sbc,
                addresmode: Cpu6502::abx,
                cyles: 4,
            },
            Instruction {
                name: "INC".to_string(),
                operate: Cpu6502::inc,
                addresmode: Cpu6502::abx,
                cyles: 7,
            },
            Instruction {
                name: "???".to_string(),
                operate: Cpu6502::xxx,
                addresmode: Cpu6502::imp,
                cyles: 7,
            },
        ];

        Self {
            a: 0x00,
            x: 0x00,
            y: 0x00,
            pc: 0x00,
            sp: 0x00,
            sr: 0x00,
            bus: Bus::new(),
            // flags: (),
            fetched: 0x00,
            temp: 0x00,
            addr_abs: 0x00,
            addr_rel: 0x00,
            opcode: 0x00,
            cycles: 0x00,
            lookup: loukup_table,
        }
    }

    // // Link this CPU to a communications bus
    // pub (crate) fn connect_bus(&mut self, n: Box<Bus>) {
    //     self.bus = Some(n);
    // }

    // Perform one clock cycle's worth of update
    pub(crate) fn clock(&mut self) {
        if self.cycles == 0 {
            self.opcode = self.read(self.pc);
            self.pc += 1;

            let instru: &Instruction = &self.lookup[self.opcode as usize];
            self.cycles = instru.cyles.clone();
            let addtion_cycle_1 = (instru.addresmode)(self);
            let instru2: &Instruction = &self.lookup[self.opcode as usize];
            let addtion_cycle_2 = (instru2.operate)(self);

            self.opcode += addtion_cycle_1 & addtion_cycle_2
        }

        self.cycles -= 1;
    }

    // Reset Interrupt - Forces CPU into known state
    pub(crate) fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xFD;
        self.sr = 0x00 | Flags6502::U;

        self.addr_abs = 0xFFFC;
        let lo: u16 = self.read(self.addr_abs + 0) as u16;
        let hi: u16 = self.read(self.addr_abs + 1) as u16;

        self.pc = (hi << 8) | lo;

        self.addr_rel = 0x0000;
        self.addr_abs = 0x0000;
        self.fetched = 0x00;

        self.cycles = 8;
    }
    // Interrupt Request - Executes an instruction at a specific location
    pub(crate) fn irq(&mut self) {
        if self.get_flag(Flags6502::I) == 0 {
            self.write(
                &(0x0100 + self.sp as u16),
                &(((self.pc >> 8) & 0x00FF) as u8),
            );
            self.sp -= 1;
            self.write(&(0x0100 + self.sp as u16), &((self.pc & 0x00FF) as u8));
            self.sp -= 1;

            self.set_flag(Flags6502::B, false);
            self.set_flag(Flags6502::U, true);
            self.set_flag(Flags6502::I, true);
            self.write(&(0x0100 + self.sp as u16), &self.sr.clone());
            self.sp -= 1;

            self.addr_abs = 0xFFFE;
            let lo: u16 = self.read(self.addr_abs + 0) as u16;
            let hi: u16 = self.read(self.addr_abs + 1) as u16;
            self.pc = (hi << 8) | lo;

            self.cycles = 7
        }
    }
    // Non-Maskable Interrupt Request - As above, but cannot be disabled
    pub(crate) fn nmi(&mut self) {
        self.write(
            &(0x0100 + self.sp as u16),
            &(((self.pc >> 8) & 0x00FF) as u8),
        );
        self.sp -= 1;
        self.write(&(0x0100 + self.sp as u16), &((self.pc & 0x00FF) as u8));
        self.sp -= 1;

        self.set_flag(Flags6502::B, false);
        self.set_flag(Flags6502::U, true);
        self.set_flag(Flags6502::I, true);
        self.write(&(0x0100 + self.sp as u16), &self.sr.clone());
        self.sp -= 1;

        self.addr_abs = 0xFFFA;
        let lo: u16 = self.read(self.addr_abs + 0) as u16;
        let hi: u16 = self.read(self.addr_abs + 1) as u16;
        self.pc = (hi << 8) | lo;

        self.cycles = 7
    }

    ///////////////////////////////////////////////////////////////////////////////
    // BUS CONNECTIVITY
    pub(crate) fn read(&self, addre: u16) -> u8 {
        self.bus.read(&addre, false)
    }

    fn write(&mut self, addre: &u16, data: &u8) {
        self.bus.write(addre, data)
    }

    ///////////////////////////////////////////////////////////////////////////////
    // FLAG FUNCTIONS

    // Returns the value of a specific bit of the status register
    fn get_flag(&mut self, f: u8) -> u8 {
        return if (self.sr & f) > 0 { 1 } else { 0 };
    }

    // Sets or clears a specific bit of the status register
    fn set_flag(&mut self, f: u8, v: bool) {
        if v {
            self.sr |= f;
        } else {
            self.sr &= !f;
        }
    }

    ///////////////////////////////////////////////////////////////////////////////
    // ADDRESSING MODES

    // The 6502 can address between 0x0000 - 0xFFFF. The high byte is often referred
    // to as the "page", and the low byte is the offset into that page. This implies
    // there are 256 pages, each containing 256 bytes.
    //
    // Several addressing modes have the potential to require an additional clock
    // cycle if they cross a page boundary. This is combined with several instructions
    // that enable this additional clock cycle. So each addressing function returns
    // a flag saying it has potential, as does each instruction. If both instruction
    // and address function return 1, then an additional clock cycle is required.

    // Address Mode: Implied
    // There is no additional data required for this instruction. The instruction
    // does something very simple like like sets a status bit. However, we will
    // target the accumulator, for instructions like PHA
    fn imp(&mut self) -> u8 {
        self.fetched = self.a;
        0
    }

    // Address Mode: Immediate
    // The instruction expects the next byte to be used as a value, so we'll prep
    // the read address to point to the next byte
    fn imm(&mut self) -> u8 {
        self.addr_abs = self.pc;
        self.pc += 1;
        return 0;
    }

    // Address Mode: Zero Page
    // To save program bytes, zero page addressing allows you to absolutely address
    // a location in first 0xFF bytes of address range. Clearly this only requires
    // one byte instead of the usual two.
    fn zp0(&mut self) -> u8 {
        self.addr_abs = self.read(self.pc) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        return 0;
    }

    // Address Mode: Zero Page with X Offset
    // Fundamentally the same as Zero Page addressing, but the contents of the X Register
    // is added to the supplied single byte address. This is useful for iterating through
    // ranges within the first page.
    fn zpx(&mut self) -> u8 {
        self.addr_abs = (self.read(self.pc) + self.x) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        0
    }

    // Address Mode: Zero Page with Y Offset
    // Same as above but uses Y Register for offset
    fn zpy(&mut self) -> u8 {
        self.addr_abs = (self.read(self.pc) + self.y) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        0
    }

    // Address Mode: Relative
    // This address mode is exclusive to branch instructions. The address
    // must reside within -128 to +127 of the branch instruction, i.e.
    // you cant directly branch to any address in the addressable range.
    fn rel(&mut self) -> u8 {
        self.addr_rel = self.read(self.pc) as u16;
        self.pc += 1;
        if (self.addr_rel & 0x80) != 0 {
            self.addr_rel |= 0xFF00;
        }
        return 0;
    }

    // Address Mode: Absolute
    // A full 16-bit address is loaded and used
    fn abs(&mut self) -> u8 {
        let lo: u16 = self.read(self.pc) as u16;
        self.pc += 1;
        let hi: u16 = self.read(self.pc) as u16;
        self.pc += 1;

        self.addr_abs = (hi << 8) | lo;

        0
    }

    // Address Mode: Absolute with X Offset
    // Fundamentally the same as absolute addressing, but the contents of the X Register
    // is added to the supplied two byte address. If the resulting address changes
    // the page, an additional clock cycle is required
    fn abx(&mut self) -> u8 {
        let lo: u16 = self.read(self.pc) as u16;
        self.pc += 1;
        let hi: u16 = self.read(self.pc) as u16;
        self.pc += 1;

        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.x as u16;

        if (self.addr_abs & 0xFF00) != (hi << 8) {
            1
        } else {
            0
        }
    }

    // Address Mode: Absolute with Y Offset
    // Fundamentally the same as absolute addressing, but the contents of the Y Register
    // is added to the supplied two byte address. If the resulting address changes
    // the page, an additional clock cycle is required
    fn aby(&mut self) -> u8 {
        let lo: u16 = self.read(self.pc) as u16;
        self.pc += 1;
        let hi: u16 = self.read(self.pc) as u16;
        self.pc += 1;

        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.y as u16;

        if (self.addr_abs & 0xFF00) != (hi << 8) {
            1
        } else {
            0
        }
    }

    // Note: The next 3 address modes use indirection (aka Pointers!)

    // Address Mode: Indirect
    // The supplied 16-bit address is read to get the actual 16-bit address. This is
    // instruction is unusual in that it has a bug in the hardware! To emulate its
    // function accurately, we also need to emulate this bug. If the low byte of the
    // supplied address is 0xFF, then to read the high byte of the actual address
    // we need to cross a page boundary. This doesnt actually work on the chip as
    // designed, instead it wraps back around in the same page, yielding an
    // invalid actual address
    fn ind(&mut self) -> u8 {
        let ptr_lo: u16 = self.read(self.pc) as u16;
        self.pc += 1;
        let ptr_hi: u16 = self.read(self.pc) as u16;
        self.pc += 1;

        let ptr: u16 = (ptr_hi << 8) | ptr_lo;

        if ptr_lo == 0x00FF
        // Simulate page boundary hardware bug
        {
            self.addr_abs = ((self.read(ptr & 0xFF00) as u16) << 8) | (self.read(ptr + 0) as u16);
        } else
        // Behave normally
        {
            self.addr_abs = ((self.read(ptr + 1) as u16) << 8) | (self.read(ptr + 0) as u16);
        }

        return 0;
    }

    // Address Mode: Indirect X
    // The supplied 8-bit address is offset by X Register to index
    // a location in page 0x00. The actual 16-bit address is read
    // from this location
    fn izx(&mut self) -> u8 {
        let t: u16 = self.read(self.pc) as u16;
        self.pc += 1;

        let lo: u16 = self.read(t + (self.x as u16) & 0x00FF) as u16;
        let hi: u16 = self.read((t + (self.x as u16) + 1) & 0x00FF) as u16;

        self.addr_abs = (hi << 8) | lo;

        0
    }

    // Address Mode: Indirect Y
    // The supplied 8-bit address indexes a location in page 0x00. From
    // here the actual 16-bit address is read, and the contents of
    // Y Register is added to it to offset it. If the offset causes a
    // change in page then an additional clock cycle is required.
    fn izy(&mut self) -> u8 {
        let t: u16 = self.read(self.pc) as u16;
        self.pc += 1;

        let lo: u16 = self.read(t & 0x00FF) as u16;
        let hi: u16 = self.read((t + 1) & 0x00FF) as u16;

        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.y as u16;

        if (self.addr_abs & 0xFF00) != (hi << 8) {
            1
        } else {
            0
        }
    }

    // Address Mode: Relative
    // This address mode is exclusive to branch instructions. The address
    // must reside within -128 to +127 of the branch instruction, i.e.
    // you cant directly branch to any address in the addressable range.
    fn red(&mut self) -> u8 {
        self.addr_rel = self.read(self.pc) as u16;
        self.pc += 1;
        if (self.addr_rel & 0x80) != 0 {
            self.addr_rel |= 0xFF00;
        }
        0
    }

    // This function sources the data used by the instruction into
    // a convenient numeric variable. Some instructions dont have to
    // fetch data as the source is implied by the instruction. For example
    // "INX" increments the X register. There is no additional data
    // required. For all other addressing modes, the data resides at
    // the location held within addr_abs, so it is read from there.
    // Immediate adress mode exploits this slightly, as that has
    // set addr_abs = self.pc + 1, so it fetches the data from the
    // next byte for example "LDA $FF" just loads the accumulator with
    // 256, i.e. no far reaching memory fetch is required. "fetched"
    // is a variable global to the CPU, and is set by calling this
    // function. It also returns it for convenience.
    fn fetch(&mut self) -> u8 {
        if (self.lookup[self.opcode as usize].addresmode) as usize != (Cpu6502::imp) as usize {
            self.fetched = self.read(self.addr_abs);
        }
        self.fetched
    }

    ///////////////////////////////////////////////////////////////////////////////
    // INSTRUCTION IMPLEMENTATIONS

    // Instruction: Add with Carry In
    // Function:    A = A + M + C
    // Flags Out:   C, V, N, Z
    //
    // Explanation:
    // The purpose of this function is to add a value to the accumulator and a carry bit. If
    // the result is > 255 there is an overflow setting the carry bit. Ths allows you to
    // chain together ADC instructions to add numbers larger than 8-bits. This in itself is
    // simple, however the 6502 supports the concepts of Negativity/Positivity and Signed Overflow.
    //
    // 10000100 = 128 + 4 = 132 in normal circumstances, we know this as unsigned and it allows
    // us to represent numbers between 0 and 255 (given 8 bits). The 6502 can also interpret
    // this word as something else if we assume those 8 bits represent the range -128 to +127,
    // i.e. it has become signed.
    //
    // Since 132 > 127, it effectively wraps around, through -128, to -124. This wraparound is
    // called overflow, and this is a useful to know as it indicates that the calculation has
    // gone outside the permissable range, and therefore no longer makes numeric sense.
    //
    // Note the implementation of ADD is the same in binary, this is just about how the numbers
    // are represented, so the word 10000100 can be both -124 and 132 depending upon the
    // context the programming is using it in. We can prove this!
    //
    //  10000100 =  132  or  -124
    // +00010001 = + 17      + 17
    //  ========    ===       ===     See, both are valid additions, but our interpretation of
    //  10010101 =  149  or  -107     the context changes the value, not the hardware!
    //
    // In principle under the -128 to 127 range:
    // 10000000 = -128, 11111111 = -1, 00000000 = 0, 00000000 = +1, 01111111 = +127
    // therefore negative numbers have the most significant set, positive numbers do not
    //
    // To assist us, the 6502 can set the overflow flag, if the result of the addition has
    // wrapped around. V <- ~(A^M) & A^(A+M+C) :D lol, let's work out why!
    //
    // Let's suppose we have A = 30, M = 10 and C = 0
    //          A = 30 = 00011110
    //          M = 10 = 00001010+
    //     RESULT = 40 = 00101000
    //
    // Here we have not gone out of range. The resulting significant bit has not changed.
    // So let's make a truth table to understand when overflow has occurred. Here I take
    // the MSB of each component, where R is RESULT.
    //
    // A  M  R | V | A^R | A^M |~(A^M) |
    // 0  0  0 | 0 |  0  |  0  |   1   |
    // 0  0  1 | 1 |  1  |  0  |   1   |
    // 0  1  0 | 0 |  0  |  1  |   0   |
    // 0  1  1 | 0 |  1  |  1  |   0   |  so V = ~(A^M) & (A^R)
    // 1  0  0 | 0 |  1  |  1  |   0   |
    // 1  0  1 | 0 |  0  |  1  |   0   |
    // 1  1  0 | 1 |  1  |  0  |   1   |
    // 1  1  1 | 0 |  0  |  0  |   1   |
    //
    // We can see how the above equation calculates V, based on A, M and R. V was chosen
    // based on the following hypothesis:
    //       Positive Number + Positive Number = Negative Result -> Overflow
    //       Negative Number + Negative Number = Positive Result -> Overflow
    //       Positive Number + Negative Number = Either Result -> Cannot Overflow
    //       Positive Number + Positive Number = Positive Result -> OK! No Overflow
    //       Negative Number + Negative Number = Negative Result -> OK! NO Overflow

    fn adc(&mut self) -> u8 {
        // Grab the data that we are adding to the accumulator
        self.fetch();

        // Add is performed in 16-bit domain for emulation to capture any
        // carry bit, which will exist in bit 8 of the 16-bit word
        self.temp = self.a as u16 + self.fetched as u16 + self.get_flag(Flags6502::C) as u16;

        // The carry flag out exists in the high byte bit 0
        self.set_flag(Flags6502::C, self.temp > 255);

        // The Zero flag is set if the result is 0
        self.set_flag(Flags6502::Z, (self.temp & 0x00FF) == 0);

        // The signed Overflow flag is set based on all that up there! :D
        self.set_flag(
            Flags6502::V,
            ((!(self.a as u16 ^ self.fetched as u16) & (self.a as u16 ^ self.temp as u16))
                & 0x0080)
                != 0,
        );

        // The negative flag is set to the most significant bit of the result
        self.set_flag(Flags6502::N, (self.temp & 0x80) != 0);

        // Load the result into the accumulator (it's 8-bit dont forget!)
        self.a = (self.temp & 0x00FF) as u8;

        // This instruction has the potential to require an additional clock cycle
        return 1;
    }

    // Instruction: Subtraction with Borrow In
    // Function:    A = A - M - (1 - C)
    // Flags Out:   C, V, N, Z
    //
    // Explanation:
    // Given the explanation for ADC above, we can reorganise our data
    // to use the same computation for addition, for subtraction by multiplying
    // the data by -1, i.e. make it negative
    //
    // A = A - M - (1 - C)  ->  A = A + -1 * (M - (1 - C))  ->  A = A + (-M + 1 + C)
    //
    // To make a signed positive number negative, we can invert the bits and add 1
    // (OK, I lied, a little bit of 1 and 2s complement :P)
    //
    //  5 = 00000101
    // -5 = 11111010 + 00000001 = 11111011 (or 251 in our 0 to 255 range)
    //
    // The range is actually unimportant, because if I take the value 15, and add 251
    // to it, given we wrap around at 256, the result is 10, so it has effectively
    // subtracted 5, which was the original intention. (15 + 251) % 256 = 10
    //
    // Note that the equation above used (1-C), but this got converted to + 1 + C.
    // This means we already have the +1, so all we need to do is invert the bits
    // of M, the data(!) therfore we can simply add, exactly the same way we did
    // before.

    fn sbc(&mut self) -> u8 {
        self.fetch();

        // Operating in 16-bit domain to capture carry out

        // We can invert the bottom 8 bits with bitwise xor
        let value: u16 = (self.fetched as u16) ^ 0x00FF;

        // Notice this is exactly the same as addition from here!
        self.temp = self.a as u16 + value + self.get_flag(Flags6502::C) as u16;
        self.set_flag(Flags6502::C, (self.temp & 0xFF00) != 0);
        self.set_flag(Flags6502::Z, (self.temp & 0x00FF) == 0);
        self.set_flag(
            Flags6502::V,
            ((self.temp ^ self.a as u16) & (self.temp ^ value) & 0x0080) != 0,
        );
        self.set_flag(Flags6502::N, (self.temp & 0x0080) != 0);
        self.a = (self.temp & 0x00FF) as u8;
        return 1;
    }

    // OK! Complicated operations are done! the following are much simpler
    // and conventional. The typical order of events is:
    // 1) Fetch the data you are working with
    // 2) Perform calculation
    // 3) Store the result in desired place
    // 4) Set Flags of the status register
    // 5) Return if instruction has potential to require additional
    //    clock cycle

    // Instruction: Bitwise Logic AND
    // Function:    A = A & M
    // Flags Out:   N, Z
    fn and(&mut self) -> u8 {
        self.fetch();
        self.a = self.a & self.fetched;
        self.set_flag(Flags6502::Z, self.a == 0x00);
        self.set_flag(Flags6502::N, (self.a & 0x80) != 0);
        return 1;
    }

    // Instruction: Arithmetic Shift Left
    // Function:    A = C <- (A << 1) <- 0
    // Flags Out:   N, Z, C
    fn asl(&mut self) -> u8 {
        self.fetch();
        self.temp = (self.fetched as u16) << 1;
        self.set_flag(Flags6502::C, (self.temp & 0xFF00) > 0);
        self.set_flag(Flags6502::Z, (self.temp & 0x00FF) == 0x00);
        self.set_flag(Flags6502::N, (self.temp & 0x80) != 0);
        if self.lookup[self.opcode as usize].addresmode as usize == Cpu6502::imp as usize {
            self.a = (self.temp & 0x00FF) as u8;
        } else {
            let addr = &self.addr_abs.clone();
            let v: u8 = (self.temp & 0x00FF) as u8;
            self.write(addr, &v);
        }
        return 0;
    }

    // Instruction: Branch if Carry Clear
    // Function:    if(C == 0) pc = address
    fn bcc(&mut self) -> u8 {
        if self.get_flag(Flags6502::C) == 0 {
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }

            self.pc = self.addr_abs;
        }
        0
    }

    // Instruction: Branch if Carry Set
    // Function:    if(C == 1) pc = address
    fn bcs(&mut self) -> u8 {
        if self.get_flag(Flags6502::C) == 1 {
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }
            self.pc = self.addr_abs;
        }
        return 0;
    }

    // Instruction: Branch if Equal
    // Function:    if(Z == 1) pc = address
    fn beq(&mut self) -> u8 {
        if self.get_flag(Flags6502::Z) == 1 {
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }
            self.pc = self.addr_abs;
        }
        0
    }

    fn bit(&mut self) -> u8 {
        self.fetch();
        self.temp = (self.a & self.fetched) as u16;
        self.set_flag(Flags6502::Z, (self.temp & 0x00FF) == 0x00);
        self.set_flag(Flags6502::N, (self.fetched & (1 << 7)) != 0);
        self.set_flag(Flags6502::V, (self.fetched & (1 << 6)) != 0);
        0
    }

    // Instruction: Branch if Negative
    // Function:    if(N == 1) pc = address
    fn bmi(&mut self) -> u8 {
        if self.get_flag(Flags6502::N) == 1 {
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }
            self.pc = self.addr_abs;
        }
        return 0;
    }

    // Instruction: Branch if Not Equal
    // Function:    if(Z == 0) pc = address
    fn bne(&mut self) -> u8 {
        if self.get_flag(Flags6502::Z) == 0 {
            self.cycles += 1;

            self.addr_abs = self.pc.overflowing_add(self.addr_rel).0;

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }
            self.pc = self.addr_abs;
        }
        return 0;
    }

    // Instruction: Branch if Positive
    // Function:    if(N == 0) pc = address
    fn bpl(&mut self) -> u8 {
        if self.get_flag(Flags6502::N) == 0 {
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }

            self.pc = self.addr_abs;
        }
        return 0;
    }

    // Instruction: Break
    // Function:    Program Sourced Interrupt

    fn brk(&mut self) -> u8 {
        self.pc += 1;

        self.set_flag(Flags6502::I, true);
        self.write(
            &(0x0100 + self.sp as u16),
            &(((self.pc >> 8) & 0x00FF) as u8),
        );
        self.sp -= 1;
        self.write(&(0x0100 + self.sp as u16), &((self.pc & 0x00FF) as u8));
        self.sp -= 1;

        self.set_flag(Flags6502::B, true);

        self.write(&(0x0100 + self.sp as u16), &self.sr.clone());
        self.sp -= 1;
        self.set_flag(Flags6502::B, false);

        self.pc = self.read(0xFFFE) as u16 | ((self.read(0xFFFF) as u16) << 8);
        return 0;
    }

    // Instruction: Branch if Overflow Clear
    // Function:    if(V == 0) pc = address
    fn bvc(&mut self) -> u8 {
        if self.get_flag(Flags6502::V) == 0 {
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }

            self.pc = self.addr_abs;
        }
        return 0;
    }

    // Instruction: Branch if Overflow Set
    // Function:    if(V == 1) pc = address
    fn bvs(&mut self) -> u8 {
        if self.get_flag(Flags6502::V) == 1 {
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel;

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }
            self.pc = self.addr_abs;
        }
        return 0;
    }

    // Instruction: Clear Carry Flag
    // Function:    C = 0
    fn clc(&mut self) -> u8 {
        self.set_flag(Flags6502::C, false);
        return 0;
    }

    // Instruction: Clear Decimal Flag
    // Function:    D = 0
    fn cld(&mut self) -> u8 {
        self.set_flag(Flags6502::D, false);
        return 0;
    }

    // Instruction: Disable Interrupts / Clear Interrupt Flag
    // Function:    I = 0
    fn cli(&mut self) -> u8 {
        self.set_flag(Flags6502::I, false);
        return 0;
    }

    // Instruction: Clear Overflow Flag
    // Function:    V = 0
    fn clv(&mut self) -> u8 {
        self.set_flag(Flags6502::V, false);
        return 0;
    }

    // Instruction: Compare Accumulator
    // Function:    C <- A >= M      Z <- (A - M) == 0
    // Flags Out:   N, C, Z
    fn cmp(&mut self) -> u8 {
        self.fetch();
        self.temp = self.a as u16 - self.fetched as u16;
        self.set_flag(Flags6502::C, self.a >= self.fetched);
        self.set_flag(Flags6502::Z, (self.temp & 0x00FF) == 0x0000);
        self.set_flag(Flags6502::N, (self.temp & 0x0080) != 0);
        return 1;
    }

    // Instruction: Compare X Register
    // Function:    C <- X >= M      Z <- (X - M) == 0
    // Flags Out:   N, C, Z
    fn cpx(&mut self) -> u8 {
        self.fetch();
        self.temp = self.x as u16 - self.fetched as u16;
        self.set_flag(Flags6502::C, self.x >= self.fetched);
        self.set_flag(Flags6502::Z, (self.temp & 0x00FF) == 0x0000);
        self.set_flag(Flags6502::N, (self.temp & 0x0080) != 0);
        return 0;
    }

    // Instruction: Compare Y Register
    // Function:    C <- Y >= M      Z <- (Y - M) == 0
    // Flags Out:   N, C, Z
    fn cpy(&mut self) -> u8 {
        self.fetch();
        self.temp = self.y as u16 - self.fetched as u16;
        self.set_flag(Flags6502::C, self.y >= self.fetched);
        self.set_flag(Flags6502::Z, (self.temp & 0x00FF) == 0x0000);
        self.set_flag(Flags6502::N, (self.temp & 0x0080) != 0);
        return 0;
    }

    // Instruction: Decrement Value at Memory Location
    // Function:    M = M - 1
    // Flags Out:   N, Z
    fn dec(&mut self) -> u8 {
        self.fetch();
        self.temp = (self.fetched - 1) as u16;
        self.write(&self.addr_abs.clone(), &((self.temp & 0x00FF) as u8));
        self.set_flag(Flags6502::Z, (self.temp & 0x00FF) == 0x0000);
        self.set_flag(Flags6502::N, (self.temp & 0x0080) != 0);
        return 0;
    }

    // Instruction: Decrement X Register
    // Function:    X = X - 1
    // Flags Out:   N, Z
    fn dex(&mut self) -> u8 {
        self.x -= 1;
        self.set_flag(Flags6502::Z, self.x == 0x00);
        self.set_flag(Flags6502::N, (self.x & 0x80) != 0);
        return 0;
    }

    // Instruction: Decrement Y Register
    // Function:    Y = Y - 1
    // Flags Out:   N, Z
    fn dey(&mut self) -> u8 {
        self.y -= 1;
        self.set_flag(Flags6502::Z, self.y == 0x00);
        self.set_flag(Flags6502::N, (self.y & 0x80) != 0);
        return 0;
    }

    // Instruction: Bitwise Logic XOR
    // Function:    A = A xor M
    // Flags Out:   N, Z
    fn eor(&mut self) -> u8 {
        self.fetch();
        self.a = self.a ^ self.fetched;
        self.set_flag(Flags6502::Z, self.a == 0x00);
        self.set_flag(Flags6502::N, (self.a & 0x80) != 0);
        return 1;
    }

    // Instruction: Increment Value at Memory Location
    // Function:    M = M + 1
    // Flags Out:   N, Z
    fn inc(&mut self) -> u8 {
        self.fetch();
        self.temp = (self.fetched + 1) as u16;
        self.write(&self.addr_abs.clone(), &((self.temp & 0x00FF) as u8));
        self.set_flag(Flags6502::Z, (self.temp & 0x00FF) == 0x0000);
        self.set_flag(Flags6502::N, (self.temp & 0x0080) != 0);
        return 0;
    }

    // Instruction: Increment X Register
    // Function:    X = X + 1
    // Flags Out:   N, Z
    fn inx(&mut self) -> u8 {
        self.x += 1;
        self.set_flag(Flags6502::Z, self.x == 0x00);
        self.set_flag(Flags6502::N, (self.x & 0x80) != 0x00);
        return 0;
    }

    // Instruction: Increment Y Register
    // Function:    Y = Y + 1
    // Flags Out:   N, Z
    fn iny(&mut self) -> u8 {
        self.y += 1;
        self.set_flag(Flags6502::Z, self.y == 0x00);
        self.set_flag(Flags6502::N, (self.y & 0x80) != 0x00);
        return 0;
    }

    // Instruction: Jump To Location
    // Function:    pc = address
    fn jmp(&mut self) -> u8 {
        self.pc = self.addr_abs;
        return 0;
    }

    // Instruction: Jump To Sub-Routine
    // Function:    Push current pc to stack, pc = address

    fn jsr(&mut self) -> u8 {
        self.pc -= 1;

        self.write(
            &(0x0100 + self.sp as u16),
            &(((self.pc >> 8) & 0x00FF) as u8),
        );
        self.sp -= 1;
        self.write(&(0x0100 + self.sp as u16), &((self.pc & 0x00FF) as u8));
        self.sp -= 1;

        self.pc = self.addr_abs;
        return 0;
    }

    // Instruction: Load The Accumulator
    // Function:    A = M
    // Flags Out:   N, Z
    fn lda(&mut self) -> u8 {
        self.fetch();
        self.a = self.fetched;
        self.set_flag(Flags6502::Z, self.a == 0x00);
        self.set_flag(Flags6502::N, (self.a & 0x80) != 0x00);
        return 1;
    }

    // Instruction: Load The X Register
    // Function:    X = M
    // Flags Out:   N, Z
    fn ldx(&mut self) -> u8 {
        self.fetch();
        self.x = self.fetched;
        self.set_flag(Flags6502::Z, self.x == 0x00);
        self.set_flag(Flags6502::N, (self.x & 0x80) != 0x00);
        return 1;
    }

    // Instruction: Load The Y Register
    // Function:    Y = M
    // Flags Out:   N, Z
    fn ldy(&mut self) -> u8 {
        self.fetch();
        self.y = self.fetched;
        self.set_flag(Flags6502::Z, self.y == 0x00);
        self.set_flag(Flags6502::N, (self.y & 0x80) != 0);
        return 1;
    }

    fn lsr(&mut self) -> u8 {
        self.fetch();
        self.set_flag(Flags6502::C, (self.fetched & 0x0001) != 0);
        self.temp = (self.fetched >> 1) as u16;
        self.set_flag(Flags6502::Z, (self.temp & 0x00FF) == 0x0000);
        self.set_flag(Flags6502::N, (self.temp & 0x0080) != 0);
        if self.lookup[self.opcode as usize].addresmode as usize == Cpu6502::imp as usize {
            self.a = (self.temp & 0x00FF) as u8;
        } else {
            self.write(&self.addr_abs.clone(), &((self.temp & 0x00FF) as u8));
        }
        return 0;
    }

    fn nop(&mut self) -> u8 {
        // Sadly not all NOPs are equal, Ive added a few here
        // based on https://wiki.nesdev.com/w/index.php/CPU_unofficial_opcodes
        // and will add more based on game compatibility, and ultimately
        // I'd like to cover all illegal opcodes too
        match self.opcode {
            0x1C => 1,
            0x3C => 1,
            0x5C => 1,
            0x7C => 1,
            0xDC => 1,
            0xFC => 1,
            _ => 0,
        }
    }

    // Instruction: Bitwise Logic OR
    // Function:    A = A | M
    // Flags Out:   N, Z
    fn ora(&mut self) -> u8 {
        self.fetch();
        self.a = self.a | self.fetched;
        self.set_flag(Flags6502::Z, self.a == 0x00);
        self.set_flag(Flags6502::N, (self.a & 0x80) != 0);
        return 1;
    }

    // Instruction: Push Accumulator to Stack
    // Function:    A -> stack

    fn pha(&mut self) -> u8 {
        self.write(&(0x0100 + self.sp as u16), &self.a.clone());
        self.sp -= 1;
        return 0;
    }

    // Instruction: Push Status Register to Stack
    // Function:    status -> stack
    // Note:        Break flag is set to 1 before push

    fn php(&mut self) -> u8 {
        self.write(
            &(0x0100 + self.sp as u16),
            &(self.sr | Flags6502::B | Flags6502::U),
        );
        self.set_flag(Flags6502::B, false);
        self.set_flag(Flags6502::U, false);
        self.sp -= 1;
        return 0;
    }

    // Instruction: Pop Accumulator off Stack
    // Function:    A <- stack
    // Flags Out:   N, Z

    fn pla(&mut self) -> u8 {
        self.sp += 1;
        self.a = self.read(0x0100 + self.sp as u16);
        self.set_flag(Flags6502::Z, self.a == 0x00);
        self.set_flag(Flags6502::N, (self.a & 0x80) != 0x00);
        return 0;
    }

    // Instruction: Pop Status Register off Stack
    // Function:    Status <- stack

    fn plp(&mut self) -> u8 {
        self.sp += 1;
        self.sr = self.read(0x0100 + self.sp as u16);
        self.set_flag(Flags6502::U, true);
        return 0;
    }

    fn rol(&mut self) -> u8 {
        self.fetch();
        self.temp = (self.fetched << 1) as u16 | (self.get_flag(Flags6502::C) as u16);
        self.set_flag(Flags6502::C, (self.temp & 0xFF00) != 0);
        self.set_flag(Flags6502::Z, (self.temp & 0x00FF) == 0x0000);
        self.set_flag(Flags6502::N, (self.temp & 0x0080) != 0);
        if self.lookup[self.opcode as usize].addresmode as usize == Cpu6502::imp as usize {
            self.a = (self.temp & 0x00FF) as u8;
        } else {
            self.write(&self.addr_abs.clone(), &((self.temp & 0x00FF) as u8));
        }
        return 0;
    }

    fn ror(&mut self) -> u8 {
        self.fetch();
        self.temp = ((self.get_flag(Flags6502::C) << 7) | (self.fetched >> 1)) as u16;
        self.set_flag(Flags6502::C, (self.fetched & 0x01) != 0);
        self.set_flag(Flags6502::Z, (self.temp & 0x00FF) == 0x00);
        self.set_flag(Flags6502::N, (self.temp & 0x0080) != 0);
        if self.lookup[self.opcode as usize].addresmode as usize == Cpu6502::imp as usize {
            self.a = (self.temp & 0x00FF) as u8;
        } else {
            self.write(&self.addr_abs.clone(), &((self.temp & 0x00FF) as u8));
        }
        return 0;
    }

    fn rti(&mut self) -> u8 {
        self.sp += 1;
        self.sr = self.read(0x0100 + self.sp as u16);
        self.sr &= !Flags6502::B;
        self.sr &= !Flags6502::U;

        self.sp += 1;
        self.pc = self.read(0x0100 + self.sp as u16) as u16;
        self.sp += 1;
        self.pc |= (self.read(0x0100 + self.sp as u16) as u16) << 8;
        return 0;
    }

    fn rts(&mut self) -> u8 {
        self.sp += 1;
        self.pc = self.read(0x0100 + self.sp as u16) as u16;
        self.sp += 1;
        self.pc |= (self.read(0x0100 + self.sp as u16) as u16) << 8;

        self.pc += 1;
        return 0;
    }

    // Instruction: Set Carry Flag
    // Function:    C = 1
    fn sec(&mut self) -> u8 {
        self.set_flag(Flags6502::C, true);
        return 0;
    }

    // Instruction: Set Decimal Flag
    // Function:    D = 1
    fn sed(&mut self) -> u8 {
        self.set_flag(Flags6502::D, true);
        return 0;
    }

    // Instruction: Set Interrupt Flag / Enable Interrupts
    // Function:    I = 1
    fn sei(&mut self) -> u8 {
        self.set_flag(Flags6502::I, true);
        return 0;
    }

    // Instruction: Store Accumulator at Address
    // Function:    M = A
    fn sta(&mut self) -> u8 {
        self.write(&self.addr_abs.clone(), &self.a.clone());
        return 0;
    }

    // Instruction: Store X Register at Address
    // Function:    M = X
    fn stx(&mut self) -> u8 {
        self.write(&self.addr_abs.clone(), &self.x.clone());
        return 0;
    }

    // Instruction: Store Y Register at Address
    // Function:    M = Y
    fn sty(&mut self) -> u8 {
        self.write(&self.addr_abs.clone(), &self.y.clone());
        return 0;
    }

    // Instruction: Transfer Accumulator to X Register
    // Function:    X = A
    // Flags Out:   N, Z
    fn tax(&mut self) -> u8 {
        self.x = self.a;
        self.set_flag(Flags6502::Z, self.x == 0x00);
        self.set_flag(Flags6502::N, (self.x & 0x80) != 0);
        return 0;
    }

    // Instruction: Transfer Accumulator to Y Register
    // Function:    Y = A
    // Flags Out:   N, Z
    fn tay(&mut self) -> u8 {
        self.y = self.a;
        self.set_flag(Flags6502::Z, self.y == 0x00);
        self.set_flag(Flags6502::N, (self.y & 0x80) != 0);
        return 0;
    }

    // Instruction: Transfer Stack Pointer to X Register
    // Function:    X = stack pointer
    // Flags Out:   N, Z
    fn tsx(&mut self) -> u8 {
        self.x = self.sp;
        self.set_flag(Flags6502::Z, self.x == 0x00);
        self.set_flag(Flags6502::N, (self.x & 0x80) != 0);
        return 0;
    }

    // Instruction: Transfer X Register to Accumulator
    // Function:    A = X
    // Flags Out:   N, Z
    fn txa(&mut self) -> u8 {
        self.a = self.x;
        self.set_flag(Flags6502::Z, self.a == 0x00);
        self.set_flag(Flags6502::N, (self.a & 0x80) != 0);
        return 0;
    }

    // Instruction: Transfer X Register to Stack Pointer
    // Function:    stack pointer = X
    fn txs(&mut self) -> u8 {
        self.sp = self.x;
        return 0;
    }

    // Instruction: Transfer Y Register to Accumulator
    // Function:    A = Y
    // Flags Out:   N, Z
    fn tya(&mut self) -> u8 {
        self.a = self.y;
        self.set_flag(Flags6502::Z, self.a == 0x00);
        self.set_flag(Flags6502::N, (self.a & 0x80) != 0);
        return 0;
    }

    // This function captures illegal opcodes
    fn xxx(&mut self) -> u8 {
        return 0;
    }

    ///////////////////////////////////////////////////////////////////////////////
    // HELPER FUNCTIONS

    fn hex(n: u32, d: u8) -> String {
        let char_arra: [char; 16] = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
        ];
        let mut nc = n;
        let mut sf: String = "".to_string();
        for _ in (0..d).rev() {
            sf = char_arra[(&nc & 0xF) as usize].to_string().add(&sf);
            nc >>= 4;
        }
        return sf;
    }

    pub(crate) fn complete(&self) -> bool {
        return self.cycles == 0;
    }

    pub(crate) fn disassemble(&self, n_start: u16, n_stop: u16) -> HashMap<u16, String> {
        let mut addr: u32 = n_start.clone() as u32;

        let mut value: u8;
        let mut hi: u8;
        let mut lo: u8;
        let mut map_lines: HashMap<u16, String> = HashMap::new();
        let mut v_s: String;

        while addr <= n_stop as u32 {
            let line_addr = addr.clone() as u16;
            let opcode = self.bus.read(&(addr as u16), true) as usize;
            let mut s_inst: String = format!(
                "${}: {} ",
                Cpu6502::hex(addr as u32, 4),
                self.lookup[opcode.clone()].name
            );
            let addremode = self.lookup[opcode.clone()].addresmode as usize;
            addr += 1;

            if addremode == Cpu6502::imp as usize {
                s_inst = s_inst.add(" {IMP}");
            } else if addremode == Cpu6502::imm as usize {
                value = self.read(addr as u16);
                addr += 1;
                v_s = format!("#${} {{IMM}}", Cpu6502::hex(value as u32, 2));
                s_inst = s_inst.add(&v_s);
            } else if addremode == Cpu6502::zp0 as usize {
                lo = self.read(addr as u16);
                addr += 1;
                // hi = 0x00;
                v_s = format!("${} {{ZP0}}", Cpu6502::hex(lo as u32, 2));
                s_inst = s_inst.add(&v_s);
            } else if addremode == Cpu6502::zpx as usize {
                lo = self.read(addr as u16);
                addr += 1;
                // hi = 0x00;
                v_s = format!("${}, X {{ZPX}}", Cpu6502::hex(lo as u32, 2));
                s_inst = s_inst.add(&v_s);
            } else if addremode == Cpu6502::zpy as usize {
                lo = self.read(addr as u16);
                addr += 1;
                // hi = 0x00;
                v_s = format!("${}, Y {{ZPY}}", Cpu6502::hex(lo as u32, 2));
                s_inst = s_inst.add(&v_s);
            } else if addremode == Cpu6502::izx as usize {
                lo = self.read(addr as u16);
                addr += 1;
                // hi = 0x00;
                v_s = format!("(${}, X) {{IZX}}", Cpu6502::hex(lo as u32, 2));
                s_inst = s_inst.add(&v_s);
            } else if addremode == Cpu6502::izy as usize {
                lo = self.read(addr as u16);
                addr += 1;
                // hi = 0x00;
                v_s = format!("(${}), Y {{IZY}}", Cpu6502::hex(lo as u32, 2));
                s_inst = s_inst.add(&v_s);
            } else if addremode == Cpu6502::abs as usize {
                lo = self.read(addr as u16);
                addr += 1;
                hi = self.read(addr as u16);
                addr += 1;
                v_s = format!("${} {{ABS}}", Cpu6502::hex((hi as u32) << 8 | lo as u32, 4));
                s_inst = s_inst.add(&v_s);
            } else if addremode == Cpu6502::abx as usize {
                lo = self.read(addr as u16);
                addr += 1;
                hi = self.read(addr as u16);
                addr += 1;
                v_s = format!(
                    "${}, X {{ABX}}",
                    Cpu6502::hex((hi as u32) << 8 | lo as u32, 4)
                );
                s_inst = s_inst.add(&v_s);
            } else if addremode == Cpu6502::aby as usize {
                lo = self.read(addr as u16);
                addr += 1;
                hi = self.read(addr as u16);
                addr += 1;
                v_s = format!(
                    "${}, Y {{ABY}}",
                    Cpu6502::hex((hi as u32) << 8 | lo as u32, 4)
                );
                s_inst = s_inst.add(&v_s);
            } else if addremode == Cpu6502::ind as usize {
                lo = self.read(addr as u16);
                addr += 1;
                hi = self.read(addr as u16);
                addr += 1;
                v_s = format!(
                    "(${}) {{IND}}",
                    Cpu6502::hex((hi as u32) << 8 | lo as u32, 4)
                );
                s_inst = s_inst.add(&v_s);
            } else if addremode == Cpu6502::rel as usize {
                value = self.read(addr as u16);
                addr += 1;
                v_s = format!(
                    "${} [${}] {{REL}}",
                    Cpu6502::hex(value as u32, 2),
                    Cpu6502::hex(addr + value as u32, 4)
                );
                s_inst = s_inst.add(&v_s);
            } else {
                panic!("not enter ifs")
            }

            map_lines.insert(line_addr, s_inst);
        }

        map_lines
    }
}
