mod cpu_6502;
mod bus;
mod olc_nes_video1_6502;

extern crate olc_pixel_game_engine;


fn main() {
    println!("Hello, world!");
    olc_nes_video1_6502::main();
}
