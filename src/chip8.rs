// TODO: remove
#![allow(dead_code)]

use std::time::Instant;

use arbitrary_int::u4;

use crate::instructions::Inst;
use crate::graphics::Drawable;

pub struct Chip8<T: Drawable> {
    memory: [u8; 4096],
    stack: [u16; 16],
    reg: Registers,
    gfx: T,
    vram: Vec<Vec<u8>>,
    clock_timeout_nanos: u128,
}

struct Registers {
    // General purpose registers
    v: [u8; 16],

    // Special purpose registers
    pc: u16,
    i: u16,
    sp: u8,
    dt: u8,
    st: u8,
}

impl Registers {
    fn new() -> Registers {
        Registers {
            v: [0; 16],
            i: 0,
            dt: 0,
            st: 0,
            pc: 0,
            sp: 0,
        }
    }
}


impl<T: Drawable> Chip8<T> {
    fn init(&mut self) {
        self.memory[0..HEX_SPRITES.len()].copy_from_slice(&HEX_SPRITES);
        self.reg.pc = 0x200;
    }

    pub fn new(freq: usize, graphics: T) -> Chip8<T> {
        let cols = graphics.width();
        let rows = graphics.height();

        let mut c8 = Chip8 {
            memory: [0; 4096],
            stack: [0; 16],
            reg: Registers::new(),
            gfx: graphics,
            vram: vec![vec![0; cols]; rows],
            clock_timeout_nanos: 1_000_000_000_u128.checked_div(freq as u128).unwrap_or(0),
        };

        c8.init();
        c8
    }

    pub fn with_rom(freq: usize, graphics: T, rom: &[u8]) -> Chip8<T> {
        let mut c8 = Chip8::new(freq, graphics);
        c8.load_rom(rom);
        c8
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.memory[0x200..0x200 + rom.len()].copy_from_slice(rom);
    }

    fn fetch(&mut self) -> u16 {
        let pc = self.reg.pc as usize;
        self.reg.pc += 2;
        (self.memory[pc] as u16) << 8 | self.memory[pc + 1] as u16
    }

    fn step(&mut self) {
        let opcode = self.fetch();
        let inst = Inst::decode(opcode).expect("Invalid opcode");
        self.execute(inst);
    }

    pub fn run(&mut self) {
        self.gfx.init();
        let mut timers = Instant::now();
        let mut clock = Instant::now();
        
        while !self.gfx.should_close() {
            loop {
                let remaining_time = self.clock_timeout_nanos.saturating_sub(clock.elapsed().as_nanos()) / 1000000;
                
                self.gfx.update(remaining_time as u32);
                self.gfx.draw_screen(&self.vram);

                // Check for timers updates
                if timers.elapsed().as_millis() >= (1000 / 60) {
                    self.reg.dt = self.reg.dt.saturating_sub(1);
                    self.reg.st = self.reg.st.saturating_sub(1);
                    timers = Instant::now();

                    if self.reg.st == 0 {
                        self.gfx.sound_pause();
                    } else {
                        self.gfx.sound_resume();
                    }
                }

                if clock.elapsed().as_nanos() >= self.clock_timeout_nanos {
                    break;
                }
            }
            clock = Instant::now();

            self.step();
        }
        self.gfx.finalize();

    }

    fn drw(&mut self, reg1: u4, reg2: u4, n: u4) {
        let x = self.reg.v[reg1.value() as usize];
        let y = self.reg.v[reg2.value() as usize];
        let sprite = self.memory[self.reg.i as usize..self.reg.i as usize + n.value() as usize].to_vec();

        self.reg.v[0xF] = 0;
        for (row, &byte) in sprite.iter().enumerate() {
            let cy = (y as usize + row) % self.vram.len();
            for col in 0..8 {
                let cx = (x as usize + col) % self.vram[0].len();
                let color = ((byte & (0b1000_0000 >> col)) > 0) as u8;
                self.reg.v[0xF] |= (self.vram[cy][cx] != 0 && color != 0) as u8;
                self.vram[cy][cx] ^= color;
            }
        }
    }

    fn execute(&mut self, inst: Inst) {
        match inst {
            Inst::SYS(_addr) => {
                // Ignore
            }
            Inst::CLS => {
                for row in self.vram.iter_mut() {
                    for pixel in row {
                        *pixel = 0;
                    }
                }
            }
            Inst::RET => {
                // TODO: bounds check?
                self.reg.sp -= 1;
                self.reg.pc = self.stack[self.reg.sp as usize];
            }
            Inst::JP(addr) => {
                self.reg.pc = addr.value();
            }
            Inst::CALL(addr) => {
                self.stack[self.reg.sp as usize] = self.reg.pc;
                self.reg.sp += 1;
                self.reg.pc = addr.value();
            }
            Inst::SE(reg, val) => {
                if self.reg.v[reg.value() as usize] == val {
                    self.reg.pc += 2;
                }
            }
            Inst::SNE(reg, val) => {
                if self.reg.v[reg.value() as usize] != val {
                    self.reg.pc += 2;
                }
            }
            Inst::SEV(reg1, reg2) => {
                if self.reg.v[reg1.value() as usize] == self.reg.v[reg2.value() as usize] {
                    self.reg.pc += 2;
                }
            }
            Inst::LD(reg, val) => {
                self.reg.v[reg.value() as usize] = val;
            }
            Inst::ADD(reg, val) => {
                self.reg.v[reg.value() as usize] = self.reg.v[reg.value() as usize].wrapping_add(val);
            }
            Inst::LDV(reg1, reg2) => {
                self.reg.v[reg1.value() as usize] = self.reg.v[reg2.value() as usize];
            }
            Inst::OR(reg1, reg2) => {
                self.reg.v[reg1.value() as usize] |= self.reg.v[reg2.value() as usize];
            }
            Inst::AND(reg1, reg2) => {
                self.reg.v[reg1.value() as usize] &= self.reg.v[reg2.value() as usize];
            }
            Inst::XOR(reg1, reg2) => {
                self.reg.v[reg1.value() as usize] ^= self.reg.v[reg2.value() as usize];
            }
            Inst::ADDV(reg1, reg2) => {
                let (val, overflow) = self.reg.v[reg1.value() as usize].overflowing_add(self.reg.v[reg2.value() as usize]);
                self.reg.v[reg1.value() as usize] = val;
                self.reg.v[0xF] = overflow as u8;
            }
            Inst::SUB(reg1, reg2) => {
                let val1 = self.reg.v[reg1.value() as usize];
                let val2 = self.reg.v[reg2.value() as usize];
                self.reg.v[0xF] = (val1 > val2) as u8;
                self.reg.v[reg1.value() as usize] = val1.wrapping_sub(val2);
            }
            Inst::SHR(reg1, _reg2) => {
                self.reg.v[0xF] = self.reg.v[reg1.value() as usize] & 0x1;
                self.reg.v[reg1.value() as usize] >>= 1;
            }
            Inst::SUBN(reg1, reg2) => {
                let val1 = self.reg.v[reg1.value() as usize];
                let val2 = self.reg.v[reg2.value() as usize];
                self.reg.v[0xF] = (val2 > val1) as u8;
                self.reg.v[reg1.value() as usize] = val2.wrapping_sub(val1);
            }
            Inst::SHL(reg1, _reg2) => {
                self.reg.v[0xF] = self.reg.v[reg1.value() as usize] & 0b1000_0000;
                self.reg.v[reg1.value() as usize] <<= 1;
            }
            Inst::SNEV(reg1, reg2) => {
                if self.reg.v[reg1.value() as usize] != self.reg.v[reg2.value() as usize] {
                    self.reg.pc += 2;
                }
            }
            Inst::LDI(addr) => {
                self.reg.i = addr.value();
            }
            Inst::JPV(addr) => {
                self.reg.pc = addr.value() + self.reg.v[0] as u16;
            }
            Inst::RND(reg, val) => {
                self.reg.v[reg.value() as usize] = rand::random::<u8>() & val;
            },
            Inst::DRW(reg1, reg2, val) => {
                self.drw(reg1, reg2, val);
            },
            Inst::SKP(reg) => {
                if self.gfx.is_key_pressed(self.reg.v[reg.value() as usize]) {
                    self.reg.pc += 2;
                }
            },
            Inst::SKNP(reg) => {
                if !self.gfx.is_key_pressed(self.reg.v[reg.value() as usize]) {
                    self.reg.pc += 2;
                }
            },
            Inst::LDVDT(reg) => {
                self.reg.v[reg.value() as usize] = self.reg.dt;
            },
            Inst::LDVKEY(reg) => {
                self.reg.v[reg.value() as usize] = self.gfx.wait_for_key();
            },
            Inst::LDDTV(reg) => {
                self.reg.dt = self.reg.v[reg.value() as usize];
            },
            Inst::LDSTV(reg) => {
                self.reg.st = self.reg.v[reg.value() as usize];
            },
            Inst::ADDIV(reg) => {
                self.reg.i += self.reg.v[reg.value() as usize] as u16;
            },
            Inst::LDFV(reg) => {
                self.reg.i = self.reg.v[reg.value() as usize] as u16 * SPRITE_SIZE as u16;
            },
            Inst::LDBV(reg) => {
                let val = self.reg.v[reg.value() as usize];
                self.memory[self.reg.i as usize] = val / 100;
                self.memory[self.reg.i as usize + 1] = (val / 10) % 10;
                self.memory[self.reg.i as usize + 2] = val % 10;
            },
            Inst::LDIV(reg) => {
                let dst = &mut self.memory[(self.reg.i as usize)..=(self.reg.i as usize + reg.value() as usize)];
                let src = &self.reg.v[0..=reg.value() as usize];
                dst.copy_from_slice(src);
            },
            Inst::LDVI(reg) => {
                let src = &self.memory[(self.reg.i as usize)..=(self.reg.i as usize + reg.value() as usize)];
                let dst = &mut self.reg.v[0..=reg.value() as usize];
                dst.copy_from_slice(src);
            },
        }
    }
}

const SPRITE_SIZE: usize = 5;
const HEX_SPRITES: [u8; 5 * 16] = [
    // 0
    0xF0, 0x90, 0x90, 0x90, 0xF0,

    // 1
    0x20, 0x60, 0x20, 0x20, 0x70,

    // 2
    0xF0, 0x10, 0xF0, 0x80, 0xF0,

    // 3
    0xF0, 0x10, 0xF0, 0x10, 0xF0,

    // 4
    0x90, 0x90, 0xF0, 0x10, 0x10,

    // 5
    0xF0, 0x80, 0xF0, 0x10, 0xF0,

    // 6
    0xF0, 0x80, 0xF0, 0x90, 0xF0,

    // 7
    0xF0, 0x10, 0x20, 0x40, 0x40,

    // 8
    0xF0, 0x90, 0xF0, 0x90, 0xF0,

    // 9
    0xF0, 0x90, 0xF0, 0x10, 0xF0,

    // A
    0xF0, 0x90, 0xF0, 0x90, 0x90,

    // B
    0xE0, 0x90, 0xE0, 0x90, 0xE0,

    // C
    0xF0, 0x80, 0x80, 0x80, 0xF0,

    // D
    0xE0, 0x90, 0x90, 0x90, 0xE0,

    // E
    0xF0, 0x80, 0xF0, 0x80, 0xF0,

    // F
    0xF0, 0x80, 0xF0, 0x80, 0x80,
];
