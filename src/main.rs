mod chip8;
mod graphics;
mod instructions;

use anyhow::anyhow;
use chip8::Chip8;
use clap::Parser;
use graphics::{Drawable, SDLGraphics, TUIGraphics};
use std::{collections::HashMap, fs};

#[derive(Parser, Debug)]
struct Args {
    /// Path to chip8 program
    #[arg(required = true)]
    file: String,

    /// Clock frequency in Hz, valid values are in the range [0, 1_000_000_000).
    /// If the value is 0 the program will run at maximum speed and will not enforce any clockspeed
    #[arg(short, long, default_value_t = 500)]
    freq: usize,

    /// Size of a game pixel (in screen pixels)
    #[arg(short, long, default_value_t = 20)]
    pixel_size: usize,

    /// Run with TUI graphics
    #[arg(long, default_value_t = false)]
    tui: bool,
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
    let rom = fs::read(&args.file)?;
    let freq = args.freq;

    if args.tui {
        let gfx = TUIGraphics::new(64, 32, (2, 1), keymap);
        run(freq, rom, gfx);
    } else {
        let gfx = SDLGraphics::new(64, 32, args.pixel_size as u32, keymap);
        run(freq, rom, gfx);
    }

    Ok(())
}

fn run<G: Drawable>(freq: usize, rom: Vec<u8>, gfx: G) {
    let mut chip8 = Chip8::with_rom(freq, gfx, &rom);
    chip8.run();
}
