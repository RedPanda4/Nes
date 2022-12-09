extern crate olc_pixel_game_engine;

use std::collections::HashMap;
use std::fmt::format;
use std::ops::Add;
use crate::bus::Bus;
use crate::olc_pixel_game_engine as olc;


struct DemoOlc6502 {
    nes: Bus,
    mapAsm: HashMap<u16, String>
}

impl DemoOlc6502 {

    fn hex(n:u32, d:u8) -> String {
        // TODO implement
        let f = format!("{{:0>{d}X}}");
        return
    }

    fn DrawRam(x:i32, y:i32, nAddr:u16, nRows: i32, nColumns:i32){

    }

}



fn main()
{
    // let demo: DemoOlc6502
    // demo;
    // demo.Construct(680, 480, 2, 2);
    // demo.Start();
    // return 0;
}