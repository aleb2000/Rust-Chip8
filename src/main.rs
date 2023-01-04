mod chip8;
mod instructions;
mod graphics;

use chip8::Chip8;
use clap::Parser;
use std::{fs, collections::HashMap};
use graphics::{SDLGraphics};
use anyhow::anyhow;

#[derive(Parser, Debug)]
struct Args {
    /// Path to chip8 program
    #[arg(required = true)]
    file: String,

    /// Clock frequency in Hz, valid values are in the range (0, 1_000_000_000].
    /// If the value is 0 the program will run at maximum speed and will not enforce any clockspeed
    #[arg(short, long, default_value_t = 500)]
    freq: usize,

    /// Size of a game pixel (in screen pixels)
    #[arg(short, long, default_value_t = 20)]
    pixel_size: usize,
}

/// Default keybindings
const KEYBINDINGS: [(u8, char); 16] = [
    (0x1, '1'),
    (0x2, '2'),
    (0x3, '3'),
    (0xC, '4'),
    (0x4, 'Q'),
    (0x5, 'W'),
    (0x6, 'E'),
    (0xD, 'R'),
    (0x7, 'A'),
    (0x8, 'S'),
    (0x9, 'D'),
    (0xE, 'F'),
    (0xA, 'Z'),
    (0x0, 'X'),
    (0xB, 'C'),
    (0xF, 'V'),
];

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.freq > 1_000_000_000 {
        return Err(anyhow!("Frequency too high, max is 1,000,000,000"));
    }

    let keymap = HashMap::from(KEYBINDINGS);

    let gfx = SDLGraphics::new(64, 32, args.pixel_size as u32, keymap); 
    
    let rom = fs::read(&args.file)?;
    let mut chip8 = Chip8::with_rom(args.freq, gfx, &rom);
    chip8.run();

    Ok(())
}

