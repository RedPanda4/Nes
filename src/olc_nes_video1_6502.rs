extern crate olc_pixel_game_engine;

use crate::cpu_6502::{Cpu6502, Flags6502};
use crate::olc_pixel_game_engine as olc;
use olc_pixel_game_engine::{draw_string, Error, Pixel};
use std::collections::HashMap;
use std::ops::Add;

pub(crate) struct DemoOlc6502 {
    nes: Cpu6502,
    map_asm: HashMap<u16, String>,
}

impl DemoOlc6502 {
    pub fn new() -> Self {
        Self {
            nes: Cpu6502::new(),
            map_asm: HashMap::new(),
        }
    }

    fn hex(n: u32, d: u8) -> String {
        // TODO implement
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

    fn draw_ram(&self, x: i32, y: i32, mut n_addr: u16, n_rows: i32, n_columns: i32) {
        let n_raw_x = x;
        let mut n_raw_y = y;
        let color: Pixel = Pixel::rgb(0xFF, 0xFF, 0xFF);
        let mut s_offset: String;
        for _ in 0..n_rows {
            let s = DemoOlc6502::hex(n_addr.clone() as u32, 4);
            s_offset = format!("${s}:");
            for _ in 0..n_columns {
                s_offset = s_offset.add(" ");
                s_offset = s_offset.add(&DemoOlc6502::hex(self.nes.read(n_addr.clone()) as u32, 2));
                n_addr += 1;
            }
            olc::draw_string(n_raw_x, n_raw_y, &s_offset, color).expect("fail to draw");
            n_raw_y += 10
        }
    }

    fn color_status(&self, f: u8) -> Pixel {
        if self.nes.sr & f != 0 {
            olc::GREEN
        } else {
            olc::RED
        }
    }

    fn draw_cpu(&self, x: i32, y: i32) {
        olc::draw_string(x, y, "STATUS: ", olc::WHITE).expect("");
        olc::draw_string(x + 64, y, "N", self.color_status(Flags6502::N)).expect("");
        olc::draw_string(x + 80, y, "V", self.color_status(Flags6502::V)).expect("");
        olc::draw_string(x + 96, y, "-", self.color_status(Flags6502::U)).expect("");
        olc::draw_string(x + 112, y, "B", self.color_status(Flags6502::B)).expect("");
        olc::draw_string(x + 128, y, "D", self.color_status(Flags6502::D)).expect("");
        olc::draw_string(x + 144, y, "I", self.color_status(Flags6502::I)).expect("");
        olc::draw_string(x + 160, y, "Z", self.color_status(Flags6502::Z)).expect("");
        olc::draw_string(x + 178, y, "C", self.color_status(Flags6502::C)).expect("");

        let s_pc: String = format!("PC: ${}", DemoOlc6502::hex(self.nes.pc as u32, 4));
        let s_a: String = format!(
            "A: ${} [{}]",
            DemoOlc6502::hex(self.nes.a as u32, 4),
            self.nes.a
        );
        let s_x: String = format!(
            "X: ${} [{}]",
            DemoOlc6502::hex(self.nes.x as u32, 4),
            self.nes.x
        );
        let s_y: String = format!(
            "Y: ${} [{}]",
            DemoOlc6502::hex(self.nes.y as u32, 4),
            self.nes.y
        );
        let s_stack: String = format!("Stack P: ${}", DemoOlc6502::hex(self.nes.sp as u32, 4));
        olc::draw_string(x, y + 10, &s_pc, olc::WHITE).expect("");
        olc::draw_string(x, y + 20, &s_a, olc::WHITE).expect("");
        olc::draw_string(x, y + 30, &s_x, olc::WHITE).expect("");
        olc::draw_string(x, y + 40, &s_y, olc::WHITE).expect("");
        olc::draw_string(x, y + 50, &s_stack, olc::WHITE).expect("");
    }

    fn draw_code(&mut self, x: i32, y: i32, n_lines: i32) {
        let pc = self.nes.pc.clone();
        let mut n_line_y = (n_lines >> 1) * 10 + y;
        match self.map_asm.get(&pc) {
            None => (),
            Some(s) => {
                draw_string(x, n_line_y, s, olc::CYAN).expect("");
            }
        }

        let mut offset_pc: u16 = 0;
        while n_line_y < (n_lines * 10) + y {
            offset_pc += 1;
            match self.map_asm.get(&(pc + offset_pc)) {
                None => (),
                Some(s) => {
                    n_line_y += 10;
                    draw_string(x, n_line_y, s, olc::WHITE).expect("");
                }
            }
        }

        offset_pc = 0;
        n_line_y = (n_lines >> 1) * 10 + y;
        while n_line_y > y {
            offset_pc += 1;
            let acc = pc.overflowing_sub(offset_pc).0;
            if let Some(s) = self.map_asm.get(&(acc)) {
                n_line_y -= 10;
                draw_string(x, n_line_y, s, olc::WHITE).expect("");
            }
        }

        // // 448 72 26
        // let it_a = self.map_asm.get(&self.nes.pc).expect("");
        // let mut n_line_y = (n_lines >> 1) * 10 + y;
        // if it_a != self.map_asm.end() {
        //     olc::draw_string(x, n_lines, it_a.second, olc::CYAN).expect("");
        //     while n_line_y < (n_lines * 10) + 10 {
        //         n_line_y += 10;
        //         it_a += 1;
        //         if it_a != self.map_asm.end() {
        //             olc::draw_string(x, n_line_y, it_a.second);
        //         }
        //
        //     }
        // }
        //
        // it_a = self.map_asm.find(nes.pc);
        // n_line_y = (n_lines >> 1) * 10 + y;
        // if (it_a != map_asm.end()){
        //     while n_line_y > y {
        //         n_line_y -= 10;
        //         it_a -= 1;
        //         if it_a != self.map_asm.end() {
        //             draw_string(x, n_line_y, it_a.second, olc::WHITE).expect("");
        //         }
        //
        //     }
        // }
    }
}

impl olc::Application for DemoOlc6502 {
    fn on_user_create(&mut self) -> Result<(), Error> {
        // Load Program (assembled at https://www.masswerk.at/6502/assembler.html)
        /*
            *=$8000
            LDX #10
            STX $0000
            LDX #3
            STX $0001
            LDY $0000
            LDA #0
            CLC
            loop
            ADC $0001
            DEY
            BNE loop
            STA $0002
            NOP
            NOP
            NOP
        */

        // Convert hex string into bytes for RAM
        let ss: String =
            "A2 0A 8E 00 00 A2 03 8E 01 00 AC 00 00 A9 00 18 6D 01 00 88 D0 FA 8D 02 00 EA EA EA"
                .to_string();
        let mut n_offset: u16 = 0x8000;
        let vc = ss.split(" ");
        for c in vc {
            self.nes.bus.ram[n_offset as usize] = u8::from_str_radix(c, 16).expect("");
            n_offset += 1;
        }
        self.nes.bus.ram[0xFFFC] = 0x00;
        self.nes.bus.ram[0xFFFD] = 0x80;

        self.map_asm = self.nes.disassemble(0x0000, 0xFFFF);

        self.nes.reset();
        return Result::Ok(());
    }

    fn on_user_update(&mut self, _f_elapsed_time: f32) -> Result<(), Error> {
        olc::clear(olc::DARK_BLUE);

        if olc::get_key(olc::Key::SPACE).pressed {
            let mut run: bool = true;
            while run || !(self.nes.complete()) {
                self.nes.clock();
                run = false;
            }
        }

        if olc::get_key(olc::Key::R).pressed {
            self.nes.reset();
        }

        if olc::get_key(olc::Key::I).pressed {
            self.nes.irq();
        }

        if olc::get_key(olc::Key::N).pressed {
            self.nes.nmi();
        }
        self.draw_ram(2, 2, 0x0000, 16, 16);
        self.draw_ram(2, 182, 0x8000, 16, 16);
        self.draw_cpu(448, 2);
        self.draw_code(448, 72, 26);

        olc::draw_string(
            10,
            370,
            "SPACE = Step Instruction    R = RESET    I = IRQ    N = NMI",
            olc::WHITE,
        )
        .expect("");
        return Result::Ok(());
    }

    fn on_user_destroy(&mut self) -> Result<(), Error> {
        Result::Ok(())
    }
}

pub fn main() -> Result<(), Error> {
    let mut demo: DemoOlc6502 = DemoOlc6502::new();
    olc::start("olc6502 Demonstration", &mut (demo), 680, 480, 2, 2)
}
