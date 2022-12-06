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

    // Flags
    flags: Flags6502
}

impl Cpu6502 {
    fn connect_bus(&mut self, n: Bus) {
        self.bus = Box::new(n);
    }

}