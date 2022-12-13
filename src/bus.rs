pub(crate) struct Bus {
    // pub(crate) cpu: Cpu6502,
    pub ram: [u8; 64 * 1024],
}

impl Bus {
    pub fn new() -> Self {
        // Self { cpu: Cpu6502::new(), ram: [0; 64 * 1024] }
        Self {
            ram: [0; 64 * 1024],
        }
    }

    pub(crate) fn write(&mut self, addr: &u16, data: &u8) {
        if addr >= &0x000 && addr <= &0xFFFF {
            self.ram[*addr as usize] = data.clone()
        }
    }

    pub(crate) fn read(&self, addr: &u16, mut b_read_only: bool) -> u8 {
        b_read_only = false;

        if addr >= &0x000 && addr <= &0xFFFF {
            return self.ram[*addr as usize];
        }

        return 0x0000;
    }
}
