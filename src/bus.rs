use crate::cpu_6502::Cpu6502;

pub(crate) struct Bus {
    cpu: Cpu6502,
    ram: [u8; 64 * 1024]
}

impl Bus {

    // pub fn new() -> Self {
    //     Self { cpu: (), ram: [0; 64 * 1024] }
    // }

    fn write(&mut self, addr: &u16, data: &u8) {
        if addr >= &0x000 && addr <= &0xFFFF {
            self.ram[*addr as usize] = data.clone()
        }
    }

    fn read(&self, addr: &u16, mut b_read_only: bool) -> u8{
        b_read_only = false;

        if addr >= &0x000 && addr <= &0xFFFF {
            return self.ram[*addr as usize];
        }

        return 0x0000
    }

}

