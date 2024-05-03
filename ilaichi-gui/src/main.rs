use ilaichi_chip8::Emulator;
use ilaichi_chip8::{SCREEN_WIDTH,SCREEN_HEIGHT};

use std::env;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

fn main() {
    let emu: Emulator = Emulator::new();

    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("usage: cargo run path/to/game");
        return;
    }
    
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("ilaichi -- chip8", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();
   
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    'gameloop: loop {
        for evt in event_pump.poll_iter() {
                match evt {
                    Event::Quit{..} => {
                    break 'gameloop;
                },
                    _ => ()
                }
        }
    }
}
