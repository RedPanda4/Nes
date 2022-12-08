use std::borrow::Borrow;
use std::ops::Deref;
use crate::bus::Bus;

enum Flags6502
{
    C = (1 << 0),
    // Carry Bit
    Z = (1 << 1),
    // Zero
    I = (1 << 2),
    // Disable Interrupts
    D = (1 << 3),
    // Decimal Mode (unused in this implementation)
    B = (1 << 4),
    // Break
    U = (1 << 5),
    // Unused
    V = (1 << 6),
    // Overflow
    N = (1 << 7),    // Negative
}

struct Instruction {
    name: String,
    operate: fn() -> u8,
    addresmode: fn(&mut Cpu6502) -> u8,
    cyles: u8
}

pub(crate) struct Cpu6502 {
    // accumulator
    a: u8,
    // register X
    x: u8,
    // register y
    y: u8,
    // program counter
    pc: u16,
    // stack pointer
    sp: u8,
    // status register
    sr: u8,

    // Bus
    bus: Box<Bus>,

    // // Flags
    // flags: Flags6502,

    // Represents the working input value to the ALU
    fetch: u8,
    // All used memory addresses end up in here
    addr_abs: u16,
    // Represents absolute address following a branch
    addr_rel: u16,
    // Is the instruction byte
    opcode: u8,
    // Counts how many cycles the instruction has remaining
    cycles: u8,

    lookup: Vec<Instruction>


}

impl Cpu6502 {

    pub fn new(bus: Box<Bus>) -> Self {

        let loukup_table: Vec<Instruction> = vec![
            Instruction { name: "BRK".to_string(), operate: Cpu6502::BRK, addresmode:Cpu6502::IMM, cyles:7 }, Instruction { name: "ORA".to_string(), operate: Cpu6502::ORA, addresmode:Cpu6502::IZX, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:8 }, Instruction { name: "???".to_string(), operate: Cpu6502::NOP, addresmode:Cpu6502::IMP, cyles:3 }, Instruction { name: "ORA".to_string(), operate: Cpu6502::ORA, addresmode:Cpu6502::ZP0, cyles:3 }, Instruction { name: "ASL".to_string(), operate: Cpu6502::ASL, addresmode:Cpu6502::ZP0, cyles:5 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:5 }, Instruction { name: "PHP".to_string(), operate: Cpu6502::PHP, addresmode:Cpu6502::IMP, cyles:3 }, Instruction { name: "ORA".to_string(), operate: Cpu6502::ORA, addresmode:Cpu6502::IMM, cyles:2 }, Instruction { name: "ASL".to_string(), operate: Cpu6502::ASL, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::NOP, addresmode:Cpu6502::IMP, cyles:4 }, Instruction { name: "ORA".to_string(), operate: Cpu6502::ORA, addresmode:Cpu6502::ABS, cyles:4 }, Instruction { name: "ASL".to_string(), operate: Cpu6502::ASL, addresmode:Cpu6502::ABS, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:6 },
            Instruction { name: "BPL".to_string(), operate: Cpu6502::BPL, addresmode:Cpu6502::REL, cyles:2 }, Instruction { name: "ORA".to_string(), operate: Cpu6502::ORA, addresmode:Cpu6502::IZY, cyles:5 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:8 }, Instruction { name: "???".to_string(), operate: Cpu6502::NOP, addresmode:Cpu6502::IMP, cyles:4 }, Instruction { name: "ORA".to_string(), operate: Cpu6502::ORA, addresmode:Cpu6502::ZPX, cyles:4 }, Instruction { name: "ASL".to_string(), operate: Cpu6502::ASL, addresmode:Cpu6502::ZPX, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:6 }, Instruction { name: "CLC".to_string(), operate: Cpu6502::CLC, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "ORA".to_string(), operate: Cpu6502::ORA, addresmode:Cpu6502::ABY, cyles:4 }, Instruction { name: "???".to_string(), operate: Cpu6502::NOP, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:7 }, Instruction { name: "???".to_string(), operate: Cpu6502::NOP, addresmode:Cpu6502::IMP, cyles:4 }, Instruction { name: "ORA".to_string(), operate: Cpu6502::ORA, addresmode:Cpu6502::ABX, cyles:4 }, Instruction { name: "ASL".to_string(), operate: Cpu6502::ASL, addresmode:Cpu6502::ABX, cyles:7 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:7 },
            Instruction { name: "JSR".to_string(), operate: Cpu6502::JSR, addresmode:Cpu6502::ABS, cyles:6 }, Instruction { name: "AND".to_string(), operate: Cpu6502::AND, addresmode:Cpu6502::IZX, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:8 }, Instruction { name: "BIT".to_string(), operate: Cpu6502::BIT, addresmode:Cpu6502::ZP0, cyles:3 }, Instruction { name: "AND".to_string(), operate: Cpu6502::AND, addresmode:Cpu6502::ZP0, cyles:3 }, Instruction { name: "ROL".to_string(), operate: Cpu6502::ROL, addresmode:Cpu6502::ZP0, cyles:5 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:5 }, Instruction { name: "PLP".to_string(), operate: Cpu6502::PLP, addresmode:Cpu6502::IMP, cyles:4 }, Instruction { name: "AND".to_string(), operate: Cpu6502::AND, addresmode:Cpu6502::IMM, cyles:2 }, Instruction { name: "ROL".to_string(), operate: Cpu6502::ROL, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "BIT".to_string(), operate: Cpu6502::BIT, addresmode:Cpu6502::ABS, cyles:4 }, Instruction { name: "AND".to_string(), operate: Cpu6502::AND, addresmode:Cpu6502::ABS, cyles:4 }, Instruction { name: "ROL".to_string(), operate: Cpu6502::ROL, addresmode:Cpu6502::ABS, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:6 },
            Instruction { name: "BMI".to_string(), operate: Cpu6502::BMI, addresmode:Cpu6502::REL, cyles:2 }, Instruction { name: "AND".to_string(), operate: Cpu6502::AND, addresmode:Cpu6502::IZY, cyles:5 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:8 }, Instruction { name: "???".to_string(), operate: Cpu6502::NOP, addresmode:Cpu6502::IMP, cyles:4 }, Instruction { name: "AND".to_string(), operate: Cpu6502::AND, addresmode:Cpu6502::ZPX, cyles:4 }, Instruction { name: "ROL".to_string(), operate: Cpu6502::ROL, addresmode:Cpu6502::ZPX, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:6 }, Instruction { name: "SEC".to_string(), operate: Cpu6502::SEC, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "AND".to_string(), operate: Cpu6502::AND, addresmode:Cpu6502::ABY, cyles:4 }, Instruction { name: "???".to_string(), operate: Cpu6502::NOP, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:7 }, Instruction { name: "???".to_string(), operate: Cpu6502::NOP, addresmode:Cpu6502::IMP, cyles:4 }, Instruction { name: "AND".to_string(), operate: Cpu6502::AND, addresmode:Cpu6502::ABX, cyles:4 }, Instruction { name: "ROL".to_string(), operate: Cpu6502::ROL, addresmode:Cpu6502::ABX, cyles:7 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:7 },
            Instruction { name: "RTI".to_string(), operate: Cpu6502::RTI, addresmode:Cpu6502::IMP, cyles:6 }, Instruction { name: "EOR".to_string(), operate: Cpu6502::EOR, addresmode:Cpu6502::IZX, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:8 }, Instruction { name: "???".to_string(), operate: Cpu6502::NOP, addresmode:Cpu6502::IMP, cyles:3 }, Instruction { name: "EOR".to_string(), operate: Cpu6502::EOR, addresmode:Cpu6502::ZP0, cyles:3 }, Instruction { name: "LSR".to_string(), operate: Cpu6502::LSR, addresmode:Cpu6502::ZP0, cyles:5 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:5 }, Instruction { name: "PHA".to_string(), operate: Cpu6502::PHA, addresmode:Cpu6502::IMP, cyles:3 }, Instruction { name: "EOR".to_string(), operate: Cpu6502::EOR, addresmode:Cpu6502::IMM, cyles:2 }, Instruction { name: "LSR".to_string(), operate: Cpu6502::LSR, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "JMP".to_string(), operate: Cpu6502::JMP, addresmode:Cpu6502::ABS, cyles:3 }, Instruction { name: "EOR".to_string(), operate: Cpu6502::EOR, addresmode:Cpu6502::ABS, cyles:4 }, Instruction { name: "LSR".to_string(), operate: Cpu6502::LSR, addresmode:Cpu6502::ABS, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:6 },
            Instruction { name: "BVC".to_string(), operate: Cpu6502::BVC, addresmode:Cpu6502::REL, cyles:2 }, Instruction { name: "EOR".to_string(), operate: Cpu6502::EOR, addresmode:Cpu6502::IZY, cyles:5 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:8 }, Instruction { name: "???".to_string(), operate: Cpu6502::NOP, addresmode:Cpu6502::IMP, cyles:4 }, Instruction { name: "EOR".to_string(), operate: Cpu6502::EOR, addresmode:Cpu6502::ZPX, cyles:4 }, Instruction { name: "LSR".to_string(), operate: Cpu6502::LSR, addresmode:Cpu6502::ZPX, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:6 }, Instruction { name: "CLI".to_string(), operate: Cpu6502::CLI, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "EOR".to_string(), operate: Cpu6502::EOR, addresmode:Cpu6502::ABY, cyles:4 }, Instruction { name: "???".to_string(), operate: Cpu6502::NOP, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:7 }, Instruction { name: "???".to_string(), operate: Cpu6502::NOP, addresmode:Cpu6502::IMP, cyles:4 }, Instruction { name: "EOR".to_string(), operate: Cpu6502::EOR, addresmode:Cpu6502::ABX, cyles:4 }, Instruction { name: "LSR".to_string(), operate: Cpu6502::LSR, addresmode:Cpu6502::ABX, cyles:7 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:7 },
            Instruction { name: "RTS".to_string(), operate: Cpu6502::RTS, addresmode:Cpu6502::IMP, cyles:6 }, Instruction { name: "ADC".to_string(), operate: Cpu6502::ADC, addresmode:Cpu6502::IZX, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:8 }, Instruction { name: "???".to_string(), operate: Cpu6502::NOP, addresmode:Cpu6502::IMP, cyles:3 }, Instruction { name: "ADC".to_string(), operate: Cpu6502::ADC, addresmode:Cpu6502::ZP0, cyles:3 }, Instruction { name: "ROR".to_string(), operate: Cpu6502::ROR, addresmode:Cpu6502::ZP0, cyles:5 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:5 }, Instruction { name: "PLA".to_string(), operate: Cpu6502::PLA, addresmode:Cpu6502::IMP, cyles:4 }, Instruction { name: "ADC".to_string(), operate: Cpu6502::ADC, addresmode:Cpu6502::IMM, cyles:2 }, Instruction { name: "ROR".to_string(), operate: Cpu6502::ROR, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "JMP".to_string(), operate: Cpu6502::JMP, addresmode:Cpu6502::IND, cyles:5 }, Instruction { name: "ADC".to_string(), operate: Cpu6502::ADC, addresmode:Cpu6502::ABS, cyles:4 }, Instruction { name: "ROR".to_string(), operate: Cpu6502::ROR, addresmode:Cpu6502::ABS, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:6 },
            Instruction { name: "BVS".to_string(), operate: Cpu6502::BVS, addresmode:Cpu6502::REL, cyles:2 }, Instruction { name: "ADC".to_string(), operate: Cpu6502::ADC, addresmode:Cpu6502::IZY, cyles:5 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:8 }, Instruction { name: "???".to_string(), operate: Cpu6502::NOP, addresmode:Cpu6502::IMP, cyles:4 }, Instruction { name: "ADC".to_string(), operate: Cpu6502::ADC, addresmode:Cpu6502::ZPX, cyles:4 }, Instruction { name: "ROR".to_string(), operate: Cpu6502::ROR, addresmode:Cpu6502::ZPX, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:6 }, Instruction { name: "SEI".to_string(), operate: Cpu6502::SEI, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "ADC".to_string(), operate: Cpu6502::ADC, addresmode:Cpu6502::ABY, cyles:4 }, Instruction { name: "???".to_string(), operate: Cpu6502::NOP, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:7 }, Instruction { name: "???".to_string(), operate: Cpu6502::NOP, addresmode:Cpu6502::IMP, cyles:4 }, Instruction { name: "ADC".to_string(), operate: Cpu6502::ADC, addresmode:Cpu6502::ABX, cyles:4 }, Instruction { name: "ROR".to_string(), operate: Cpu6502::ROR, addresmode:Cpu6502::ABX, cyles:7 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:7 },
            Instruction { name: "???".to_string(), operate: Cpu6502::NOP, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "STA".to_string(), operate: Cpu6502::STA, addresmode:Cpu6502::IZX, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::NOP, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:6 }, Instruction { name: "STY".to_string(), operate: Cpu6502::STY, addresmode:Cpu6502::ZP0, cyles:3 }, Instruction { name: "STA".to_string(), operate: Cpu6502::STA, addresmode:Cpu6502::ZP0, cyles:3 }, Instruction { name: "STX".to_string(), operate: Cpu6502::STX, addresmode:Cpu6502::ZP0, cyles:3 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:3 }, Instruction { name: "DEY".to_string(), operate: Cpu6502::DEY, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::NOP, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "TXA".to_string(), operate: Cpu6502::TXA, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "STY".to_string(), operate: Cpu6502::STY, addresmode:Cpu6502::ABS, cyles:4 }, Instruction { name: "STA".to_string(), operate: Cpu6502::STA, addresmode:Cpu6502::ABS, cyles:4 }, Instruction { name: "STX".to_string(), operate: Cpu6502::STX, addresmode:Cpu6502::ABS, cyles:4 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:4 },
            Instruction { name: "BCC".to_string(), operate: Cpu6502::BCC, addresmode:Cpu6502::REL, cyles:2 }, Instruction { name: "STA".to_string(), operate: Cpu6502::STA, addresmode:Cpu6502::IZY, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:6 }, Instruction { name: "STY".to_string(), operate: Cpu6502::STY, addresmode:Cpu6502::ZPX, cyles:4 }, Instruction { name: "STA".to_string(), operate: Cpu6502::STA, addresmode:Cpu6502::ZPX, cyles:4 }, Instruction { name: "STX".to_string(), operate: Cpu6502::STX, addresmode:Cpu6502::ZPY, cyles:4 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:4 }, Instruction { name: "TYA".to_string(), operate: Cpu6502::TYA, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "STA".to_string(), operate: Cpu6502::STA, addresmode:Cpu6502::ABY, cyles:5 }, Instruction { name: "TXS".to_string(), operate: Cpu6502::TXS, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:5 }, Instruction { name: "???".to_string(), operate: Cpu6502::NOP, addresmode:Cpu6502::IMP, cyles:5 }, Instruction { name: "STA".to_string(), operate: Cpu6502::STA, addresmode:Cpu6502::ABX, cyles:5 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:5 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:5 },
            Instruction { name: "LDY".to_string(), operate: Cpu6502::LDY, addresmode:Cpu6502::IMM, cyles:2 }, Instruction { name: "LDA".to_string(), operate: Cpu6502::LDA, addresmode:Cpu6502::IZX, cyles:6 }, Instruction { name: "LDX".to_string(), operate: Cpu6502::LDX, addresmode:Cpu6502::IMM, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:6 }, Instruction { name: "LDY".to_string(), operate: Cpu6502::LDY, addresmode:Cpu6502::ZP0, cyles:3 }, Instruction { name: "LDA".to_string(), operate: Cpu6502::LDA, addresmode:Cpu6502::ZP0, cyles:3 }, Instruction { name: "LDX".to_string(), operate: Cpu6502::LDX, addresmode:Cpu6502::ZP0, cyles:3 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:3 }, Instruction { name: "TAY".to_string(), operate: Cpu6502::TAY, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "LDA".to_string(), operate: Cpu6502::LDA, addresmode:Cpu6502::IMM, cyles:2 }, Instruction { name: "TAX".to_string(), operate: Cpu6502::TAX, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "LDY".to_string(), operate: Cpu6502::LDY, addresmode:Cpu6502::ABS, cyles:4 }, Instruction { name: "LDA".to_string(), operate: Cpu6502::LDA, addresmode:Cpu6502::ABS, cyles:4 }, Instruction { name: "LDX".to_string(), operate: Cpu6502::LDX, addresmode:Cpu6502::ABS, cyles:4 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:4 },
            Instruction { name: "BCS".to_string(), operate: Cpu6502::BCS, addresmode:Cpu6502::REL, cyles:2 }, Instruction { name: "LDA".to_string(), operate: Cpu6502::LDA, addresmode:Cpu6502::IZY, cyles:5 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:5 }, Instruction { name: "LDY".to_string(), operate: Cpu6502::LDY, addresmode:Cpu6502::ZPX, cyles:4 }, Instruction { name: "LDA".to_string(), operate: Cpu6502::LDA, addresmode:Cpu6502::ZPX, cyles:4 }, Instruction { name: "LDX".to_string(), operate: Cpu6502::LDX, addresmode:Cpu6502::ZPY, cyles:4 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:4 }, Instruction { name: "CLV".to_string(), operate: Cpu6502::CLV, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "LDA".to_string(), operate: Cpu6502::LDA, addresmode:Cpu6502::ABY, cyles:4 }, Instruction { name: "TSX".to_string(), operate: Cpu6502::TSX, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:4 }, Instruction { name: "LDY".to_string(), operate: Cpu6502::LDY, addresmode:Cpu6502::ABX, cyles:4 }, Instruction { name: "LDA".to_string(), operate: Cpu6502::LDA, addresmode:Cpu6502::ABX, cyles:4 }, Instruction { name: "LDX".to_string(), operate: Cpu6502::LDX, addresmode:Cpu6502::ABY, cyles:4 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:4 },
            Instruction { name: "CPY".to_string(), operate: Cpu6502::CPY, addresmode:Cpu6502::IMM, cyles:2 }, Instruction { name: "CMP".to_string(), operate: Cpu6502::CMP, addresmode:Cpu6502::IZX, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::NOP, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:8 }, Instruction { name: "CPY".to_string(), operate: Cpu6502::CPY, addresmode:Cpu6502::ZP0, cyles:3 }, Instruction { name: "CMP".to_string(), operate: Cpu6502::CMP, addresmode:Cpu6502::ZP0, cyles:3 }, Instruction { name: "DEC".to_string(), operate: Cpu6502::DEC, addresmode:Cpu6502::ZP0, cyles:5 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:5 }, Instruction { name: "INY".to_string(), operate: Cpu6502::INY, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "CMP".to_string(), operate: Cpu6502::CMP, addresmode:Cpu6502::IMM, cyles:2 }, Instruction { name: "DEX".to_string(), operate: Cpu6502::DEX, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "CPY".to_string(), operate: Cpu6502::CPY, addresmode:Cpu6502::ABS, cyles:4 }, Instruction { name: "CMP".to_string(), operate: Cpu6502::CMP, addresmode:Cpu6502::ABS, cyles:4 }, Instruction { name: "DEC".to_string(), operate: Cpu6502::DEC, addresmode:Cpu6502::ABS, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:6 },
            Instruction { name: "BNE".to_string(), operate: Cpu6502::BNE, addresmode:Cpu6502::REL, cyles:2 }, Instruction { name: "CMP".to_string(), operate: Cpu6502::CMP, addresmode:Cpu6502::IZY, cyles:5 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:8 }, Instruction { name: "???".to_string(), operate: Cpu6502::NOP, addresmode:Cpu6502::IMP, cyles:4 }, Instruction { name: "CMP".to_string(), operate: Cpu6502::CMP, addresmode:Cpu6502::ZPX, cyles:4 }, Instruction { name: "DEC".to_string(), operate: Cpu6502::DEC, addresmode:Cpu6502::ZPX, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:6 }, Instruction { name: "CLD".to_string(), operate: Cpu6502::CLD, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "CMP".to_string(), operate: Cpu6502::CMP, addresmode:Cpu6502::ABY, cyles:4 }, Instruction { name: "NOP".to_string(), operate: Cpu6502::NOP, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:7 }, Instruction { name: "???".to_string(), operate: Cpu6502::NOP, addresmode:Cpu6502::IMP, cyles:4 }, Instruction { name: "CMP".to_string(), operate: Cpu6502::CMP, addresmode:Cpu6502::ABX, cyles:4 }, Instruction { name: "DEC".to_string(), operate: Cpu6502::DEC, addresmode:Cpu6502::ABX, cyles:7 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:7 },
            Instruction { name: "CPX".to_string(), operate: Cpu6502::CPX, addresmode:Cpu6502::IMM, cyles:2 }, Instruction { name: "SBC".to_string(), operate: Cpu6502::SBC, addresmode:Cpu6502::IZX, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::NOP, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:8 }, Instruction { name: "CPX".to_string(), operate: Cpu6502::CPX, addresmode:Cpu6502::ZP0, cyles:3 }, Instruction { name: "SBC".to_string(), operate: Cpu6502::SBC, addresmode:Cpu6502::ZP0, cyles:3 }, Instruction { name: "INC".to_string(), operate: Cpu6502::INC, addresmode:Cpu6502::ZP0, cyles:5 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:5 }, Instruction { name: "INX".to_string(), operate: Cpu6502::INX, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "SBC".to_string(), operate: Cpu6502::SBC, addresmode:Cpu6502::IMM, cyles:2 }, Instruction { name: "NOP".to_string(), operate: Cpu6502::NOP, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::SBC, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "CPX".to_string(), operate: Cpu6502::CPX, addresmode:Cpu6502::ABS, cyles:4 }, Instruction { name: "SBC".to_string(), operate: Cpu6502::SBC, addresmode:Cpu6502::ABS, cyles:4 }, Instruction { name: "INC".to_string(), operate: Cpu6502::INC, addresmode:Cpu6502::ABS, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:6 },
            Instruction { name: "BEQ".to_string(), operate: Cpu6502::BEQ, addresmode:Cpu6502::REL, cyles:2 }, Instruction { name: "SBC".to_string(), operate: Cpu6502::SBC, addresmode:Cpu6502::IZY, cyles:5 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:8 }, Instruction { name: "???".to_string(), operate: Cpu6502::NOP, addresmode:Cpu6502::IMP, cyles:4 }, Instruction { name: "SBC".to_string(), operate: Cpu6502::SBC, addresmode:Cpu6502::ZPX, cyles:4 }, Instruction { name: "INC".to_string(), operate: Cpu6502::INC, addresmode:Cpu6502::ZPX, cyles:6 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:6 }, Instruction { name: "SED".to_string(), operate: Cpu6502::SED, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "SBC".to_string(), operate: Cpu6502::SBC, addresmode:Cpu6502::ABY, cyles:4 }, Instruction { name: "NOP".to_string(), operate: Cpu6502::NOP, addresmode:Cpu6502::IMP, cyles:2 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:7 }, Instruction { name: "???".to_string(), operate: Cpu6502::NOP, addresmode:Cpu6502::IMP, cyles:4 }, Instruction { name: "SBC".to_string(), operate: Cpu6502::SBC, addresmode:Cpu6502::ABX, cyles:4 }, Instruction { name: "INC".to_string(), operate: Cpu6502::INC, addresmode:Cpu6502::ABX, cyles:7 }, Instruction { name: "???".to_string(), operate: Cpu6502::XXX, addresmode:Cpu6502::IMP, cyles:7 },
        ];

        Self {
            a: 0x00,
            x: 0x00,
            y: 0x00,
            pc: 0x00,
            sp: 0x00,
            sr: 0x00,
            bus,
            // flags: (),
            fetch: 0x00,
            addr_abs: 0x00,
            addr_rel: 0x00,
            opcode: 0x00,
            cycles: 0x00,
            lookup: loukup_table
        }
    }

    // Link this CPU to a communications bus
    fn connect_bus(&mut self, n: Box<Bus>) {
        self.bus = n;
    }

    // Perform one clock cycle's worth of update
    fn clock(&mut self){
        if self.cycles == 0 {
            self.opcode = self.read(self.pc);
            self.pc += 1;

            let instru: Instruction = self.lookup[self.opcode];
            self.cycles = instru.cyles;
            let addtion_cycle_1 = instru.addresmode();
            let addtion_cycle_2 = instru.operate();

            self.opcode += (addtion_cycle_1 & addtion_cycle_2)
        }

        self.cycles -= 1;
    }

    // Reset Interrupt - Forces CPU into known state
    fn reset(){}
    // Interrupt Request - Executes an instruction at a specific location
    fn irq(){}
    // Non-Maskable Interrupt Request - As above, but cannot be disabled
    fn nmi(){}

    ///////////////////////////////////////////////////////////////////////////////
    // BUS CONNECTIVITY
    fn read(&self, addre: u16) -> u8 {
        self.bus.read(&addre, false)
    }

    fn write(&mut self, addre: &u16, data: &u8) {
        self.bus.write(addre, data)
    }

    ///////////////////////////////////////////////////////////////////////////////
    // FLAG FUNCTIONS

    // Returns the value of a specific bit of the status register
    fn get_flag(&self, flag: Flags6502) -> bool {
        false
    }
    // Sets or clears a specific bit of the status register
    fn set_flag(&self, flag: Flags6502, b:bool) {

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
    fn imp() -> u8 {
        fetch = a;
        0
    }

    // Address Mode: Immediate
    // The instruction expects the next byte to be used as a value, so we'll prep
    // the read address to point to the next byte
    fn imm() -> u8 {
        pc += 1;
        addr_abs = pc;
        return 0;
    }

    // Address Mode: Zero Page
    // To save program bytes, zero page addressing allows you to absolutely address
    // a location in first 0xFF bytes of address range. Clearly this only requires
    // one byte instead of the usual two.
    fn zp0() -> u8 {
    addr_abs = self::read(pc);
    pc += 1;
    addr_abs &= 0x00FF;
    return 0;
    }

    // Address Mode: Zero Page with X Offset
    // Fundamentally the same as Zero Page addressing, but the contents of the X Register
    // is added to the supplied single byte address. This is useful for iterating through
    // ranges within the first page.
    fn zpx() -> u8 {
    addr_abs = (self::read(pc) + x);
    pc += 1;
    addr_abs &= 0x00FF;
    return 0;
    }

    // Address Mode: Zero Page with Y Offset
    // Same as above but uses Y Register for offset
    fn zpy(&mut self) -> u8 {
        addr_abs = (self::read(pc) + y);
        pc += 1;
        addr_abs &= 0x00FF;
        return 0;
    }

    // Address Mode: Absolute
    // A full 16-bit address is loaded and used
    fn abs() -> u8
    {
        let lo: u16 = self::read(pc) as u16;
        pc += 1;
        let hi: u16 = self::read(pc) as u16;
        pc += 1;

        addr_abs = (hi << 8) | lo;

        return 0;
    }

    // Address Mode: Absolute with X Offset
    // Fundamentally the same as absolute addressing, but the contents of the X Register
    // is added to the supplied two byte address. If the resulting address changes
    // the page, an additional clock cycle is required
    fn abx() -> u8 {
        let lo: u16 = self::read(pc) as u16;
        pc += 1;
        let hi: u16 = self::read(pc) as u16;
        pc += 1;

        addr_abs = (hi << 8) | lo;
        addr_abs += x;

        if (addr_abs & 0xFF00) != (hi << 8) {
            1
        } else {
            0
        }
    }


    // Address Mode: Absolute with Y Offset
    // Fundamentally the same as absolute addressing, but the contents of the Y Register
    // is added to the supplied two byte address. If the resulting address changes
    // the page, an additional clock cycle is required
    fn aby() -> u8 {
        let lo: u16 = self::read(pc) as u16;
        pc += 1;
        let hi: u16 = self::read(pc) as u16;
        pc += 1;

        addr_abs = (hi << 8) | lo;
        addr_abs += y;

        if (addr_abs & 0xFF00) != (hi << 8) {
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
    fn ind() -> u8 {
        let ptr_lo: u16 = self::read(pc) as u16;
        pc += 1;
        let ptr_hi: u16 = self::read(pc) as u16;
        pc += 1;

        let ptr: u16 = (ptr_hi << 8) | ptr_lo;

        if ptr_lo == 0x00FF // Simulate page boundary hardware bug
        {
            addr_abs = (self::read(ptr & 0xFF00) << 8) | self::read(ptr + 0);
        } else // Behave normally
        {
            addr_abs = (self::read(ptr + 1) << 8) | self::read(ptr + 0);
        }

        return 0;
    }


    // Address Mode: Indirect X
    // The supplied 8-bit address is offset by X Register to index
    // a location in page 0x00. The actual 16-bit address is read
    // from this location
    fn izx() -> u8 {
        let t: u16 = self::read(pc) as u16;
        pc += 1;

        let lo: u16 = self::read(t + (x as u16) & 0x00FF);
        let hi: u16 = self::read((t + (x as u16) + 1) & 0x00FF);

        addr_abs = (hi << 8) | lo;

        0
    }


    // Address Mode: Indirect Y
    // The supplied 8-bit address indexes a location in page 0x00. From
    // here the actual 16-bit address is read, and the contents of
    // Y Register is added to it to offset it. If the offset causes a
    // change in page then an additional clock cycle is required.
    fn izy() -> u8 {
        let t: u16 = self::read(pc);
        pc += 1;

        let lo: u16 = self::read(t & 0x00FF);
        let hi: u16 = self::read((t + 1) & 0x00FF);

        addr_abs = (hi << 8) | lo;
        addr_abs += y;

        if (addr_abs & 0xFF00) != (hi << 8) {
            1
        } else {
            0
        }
    }

    // Address Mode: Relative
    // This address mode is exclusive to branch instructions. The address
    // must reside within -128 to +127 of the branch instruction, i.e.
    // you cant directly branch to any address in the addressable range.
    fn red() -> u8 {
        addr_rel = self::read(pc);
        pc += 1;
        if addr_rel & 0x80 {
            addr_rel |= 0xFF00;
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
    // set addr_abs = pc + 1, so it fetches the data from the
    // next byte for example "LDA $FF" just loads the accumulator with
    // 256, i.e. no far reaching memory fetch is required. "fetched"
    // is a variable global to the CPU, and is set by calling this
    // function. It also returns it for convenience.
    fn fetch() -> u8 {
        if !(lookup[opcode].addrmode == &Cpu6502::imp){
            fetched = self::read(addr_abs);
        }
        fetched
    }


    ///////////////////////////////////////////////////////////////////////////////
    // INSTRUCTION IMPLEMENTATIONS
    fn xxx() -> u8 {
        0
    }

}
