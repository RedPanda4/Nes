extern crate olc_pixel_game_engine;

use crate::bus::Bus;
use crate::olc_pixel_game_engine as olc;
use std::collections::HashMap;
use std::ops::Add;
use crate::cpu_6502::{Cpu6502, Flags6502};
use olc_pixel_game_engine::{draw_string, Error, Pixel};

pub(crate) struct DemoOlc6502 {
    nes: Cpu6502,
    mapAsm: HashMap<u16, String>,
}

impl DemoOlc6502 {

    pub fn new() -> Self {
        Self { nes: Cpu6502::new(), mapAsm: HashMap::new() }
    }

    fn hex(n: u32, d: u8) -> String {
        // TODO implement
        let char_arra: [char; 16] = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
        ];
        let mut nc = n;
        let mut sf: String = "".to_string();
        for i in (0..d).rev() {
            sf = char_arra[(&nc & 0xF) as usize].to_string().add(&sf);
            nc >>= 4;
        }
        return sf
    }

    fn DrawRam(&self, x: i32, y: i32, mut nAddr: u16, nRows: i32, nColumns: i32) {
        let mut nRawX = x;
        let mut nRawY = y;
        let color: Pixel = Pixel::rgb(0xFF, 0xFF, 0xFF);
        let mut sOffset:String;
        for row in 0..nRows {
            let s = DemoOlc6502::hex(nAddr.clone() as u32, 4);
            sOffset = format!("${s}:");
            for col in 0..nColumns {
                sOffset = sOffset.add(" ");
                sOffset = sOffset.add(&DemoOlc6502::hex(self.nes.read(nAddr.clone()) as u32, 2));
                nAddr += 1;
            }
            olc::draw_string(nRawX, nRawY, &sOffset, color).expect("fail to draw");
            nRawY += 10
        }

    }

    fn color_status(&self, f: u8) -> Pixel {
        if self.nes.sr & f != 0 {
            olc::GREEN
        } else {
            olc::RED
        }
    }

    fn DrawCpu(&self, x: i32, y: i32) {
        olc::draw_string(x, y, "STATUS: ", olc::WHITE).expect("");
        olc::draw_string(x + 64, y, "N", self.color_status(Flags6502::N)).expect("");
        olc::draw_string(x + 80, y, "V", self.color_status(Flags6502::V)).expect("");
        olc::draw_string(x + 96, y, "-", self.color_status(Flags6502::U)).expect("");
        olc::draw_string(x + 112, y, "B", self.color_status(Flags6502::B)).expect("");
        olc::draw_string(x + 128, y, "D", self.color_status(Flags6502::D)).expect("");
        olc::draw_string(x + 144, y, "I", self.color_status(Flags6502::I)).expect("");
        olc::draw_string(x + 160, y, "Z", self.color_status(Flags6502::Z)).expect("");
        olc::draw_string(x + 178, y, "C", self.color_status(Flags6502::C)).expect("");

        let s_pc:String = format!("PC: ${}", DemoOlc6502::hex(self.nes.pc as u32, 4));
        let s_a:String = format!("A: ${} [{}]", DemoOlc6502::hex(self.nes.a as u32, 4), self.nes.a);
        let s_x:String = format!("X: ${} [{}]", DemoOlc6502::hex(self.nes.x as u32, 4), self.nes.x);
        let s_y:String = format!("Y: ${} [{}]", DemoOlc6502::hex(self.nes.y as u32, 4), self.nes.y);
        let s_stack:String = format!("Stack P: ${}", DemoOlc6502::hex(self.nes.sp as u32, 4));
        olc::draw_string(x, y + 10, &s_pc, olc::WHITE).expect("");
        olc::draw_string(x, y + 20, &s_a, olc::WHITE).expect("");
        olc::draw_string(x, y + 30, &s_x, olc::WHITE).expect("");
        olc::draw_string(x, y + 40, &s_y, olc::WHITE).expect("");
        olc::draw_string(x, y + 50, &s_stack, olc::WHITE).expect("");


    }

    fn DrawCode(&mut self, x: i32, y:i32, nLines:i32) {

        let mut pc = self.nes.pc.clone();
        let mut nLineY = (nLines >> 1) * 10 + y;
        println!("here");
        match  self.mapAsm.get(&pc) {
            None => (),
            Some(s) => {
                draw_string(x, nLineY, s, olc::CYAN).expect("")
            }
        }



        // // 448 72 26
        // let it_a = self.mapAsm.get(&self.nes.pc).expect("");
        // let mut nLineY = (nLines >> 1) * 10 + y;
        // if it_a != self.mapAsm.end() {
        //     olc::draw_string(x, nLines, it_a.second, olc::CYAN).expect("");
        //     while nLineY < (nLines * 10) + 10 {
        //         nLineY += 10;
        //         it_a += 1;
        //         if it_a != self.mapAsm.end() {
        //             olc::draw_string(x, nLineY, it_a.second);
        //         }
        //
        //     }
        // }
        //
        // it_a = self.mapAsm.find(nes.pc);
        // nLineY = (nLines >> 1) * 10 + y;
        // if (it_a != mapAsm.end()){
        //     while nLineY > y {
        //         nLineY -= 10;
        //         it_a -= 1;
        //         if it_a != self.mapAsm.end() {
        //             draw_string(x, nLineY, it_a.second, olc::WHITE).expect("");
        //         }
        //
        //     }
        // }


    }

}

impl olc::Application for DemoOlc6502  {


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
        let ss: String = "A2 0A 8E 00 00 A2 03 8E 01 00 AC 00 00 A9 00 18 6D 01 00 88 D0 FA 8D 02 00 EA EA EA".to_string();
        let mut nOffset: u16 = 0x8000;
        let vc = ss.split(" ");
        for c in vc{
            self.nes.bus.ram[nOffset as usize] = u8::from_str_radix(c, 16).expect("");
            nOffset += 1;
        }
        self.nes.bus.ram[0xFFFC] = 0x00;
        self.nes.bus.ram[0xFFFD] = 0x80;

        // self.mapAsm = self.nes.dissemble(0x0000, 0xFFFF);

        self.nes.reset();
        return Result::Ok(());
    }

    fn on_user_update(&mut self, fElapsedTime:f32) -> Result<(), Error> {

        olc::clear(olc::DARK_BLUE);

        if olc::get_key(olc::Key::SPACE).pressed {
            let mut run:bool = true;
            while run && !(self.nes.complete()) {
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
        self.DrawRam(2, 2, 0x0000, 16, 16);
        self.DrawRam(2, 182, 0x8000, 16, 16);
        self.DrawCpu(448, 2);
        self.DrawCode(448, 72, 26);

        olc::draw_string(10, 370, "SPACE = Step Instruction    R = RESET    I = IRQ    N = NMI", olc::WHITE).expect("");
        return Result::Ok(());
    }


    fn on_user_destroy(&mut self) -> Result<(), Error> {
        Result::Ok(())
    }
}


pub fn main() {
    let mut demo:DemoOlc6502 = DemoOlc6502::new();
    olc::start("olc6502 Demonstration", &mut (demo), 680, 480, 2, 2);

}
