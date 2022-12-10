extern crate olc_pixel_game_engine;

use crate::bus::Bus;
use crate::olc_pixel_game_engine as olc;
use std::collections::HashMap;
use std::ops::Add;
use crate::cpu_6502::Flags6502;
use olc_pixel_game_engine::Pixel;

pub(crate) struct DemoOlc6502 {
    nes: Bus,
    mapAsm: HashMap<u16, String>,
}

impl DemoOlc6502 {
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

    fn DrawRam(&self, x: i32, y: i32, nAddr: u16, nRows: i32, nColumns: i32) {
        let mut nRawX = x;
        let mut nRawY = y;
        let color: Pixel = Pixel::rgb(0xFF, 0xFF, 0xFF);
        for row in 0..nRows {
            let s = DemoOlc6502::hex(nAddr as u32, 4);
            let mut sOffset = format!("${s}:");
            for col in 0..nColumns {
                sOffset.add(" ");
                sOffset.add(&DemoOlc6502::hex(self.nes.read(&nAddr,true) as u32, 2));
            }
            olc::draw_string(nRawX, nRawY, &sOffset, color).expect("fail to draw");
            nRawY += 10
        }

    }

    fn color_status(&self, f: u8) -> Pixel {
        if self.nes.cpu.sr & f != 0 {
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

        let s_pc:String = format!("PC: ${}", DemoOlc6502::hex(self.nes.cpu.pc as u32, 4));
        let s_a:String = format!("A: ${} [{}]", DemoOlc6502::hex(self.nes.cpu.a as u32, 4), self.nes.cpu.a);
        let s_x:String = format!("X: ${} [{}]", DemoOlc6502::hex(self.nes.cpu.x as u32, 4), self.nes.cpu.x);
        let s_y:String = format!("Y: ${} [{}]", DemoOlc6502::hex(self.nes.cpu.y as u32, 4), self.nes.cpu.y);
        let s_stack:String = format!("Stack P: ${}", DemoOlc6502::hex(self.nes.cpu.sp as u32, 4));
        olc::draw_string(x, y + 10, &s_pc, olc::WHITE).expect("");
        olc::draw_string(x, y + 20, &s_a, olc::WHITE).expect("");
        olc::draw_string(x, y + 30, &s_x, olc::WHITE).expect("");
        olc::draw_string(x, y + 40, &s_y, olc::WHITE).expect("");
        olc::draw_string(x, y + 50, &s_stack, olc::WHITE).expect("");


    }
    fn DrawCode(&mut self, x: i32, y:i32, nLines:i32) {
        let it_a = self.mapAsm.get(&self.nes.cpu.pc).expect("");
        let mut nLineY = (nLines >> 1) * 10 + y;



    }
    fn OnUserCreate() {

    }

    fn OnUserUpdate(fElapsedTime:f32) -> bool {
        true
    }


}


fn main() {

}
