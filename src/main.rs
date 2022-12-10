extern crate olc_pixel_game_engine;

use std::thread::sleep;
use std::time::Duration;
use crate::olc_pixel_game_engine as olc;

mod cpu_6502;
mod bus;
mod olc_nes_video1_6502;

use olc_nes_video1_6502 as cl;


fn main() {
    println!("Hello, world!");

    for n in 0..0xFFFFFFF {

        println!("{}", &cl::DemoOlc6502::hex(0xF3A298EB, 2));
        sleep(Duration::new(1,0));
        break
    }
}
