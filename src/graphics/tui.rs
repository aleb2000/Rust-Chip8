use crossterm::{
    event::{self, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    layout::Rect,
    prelude::{CrosstermBackend, Stylize, Terminal},
    style::{Color, Style},
    symbols,
    widgets::{
        canvas::{Canvas, Painter, Points, Rectangle, Shape},
        Block, Paragraph,
    },
};
use sdl2::keyboard::Keycode;
use std::{
    collections::HashMap,
    io::{stdout, Result, Stdout},
    process::exit,
    time::Duration,
};

use super::Drawable;

pub struct TUIGraphics {
    terminal: Option<Terminal<CrosstermBackend<Stdout>>>,
    width_cells: usize,
    height_cells: usize,
    cell_size: (usize, usize),
    keymap: HashMap<u8, char>,
    close_requested: bool,
}

impl TUIGraphics {
    pub fn new(
        width_cells: usize,
        height_cells: usize,
        cell_size: (usize, usize),
        keymap: HashMap<u8, char>,
    ) -> TUIGraphics {
        TUIGraphics {
            terminal: None,
            width_cells,
            height_cells,
            cell_size,
            keymap,
            close_requested: false,
        }
    }

    fn clear_event_queue(&mut self) {
        while event::poll(Duration::ZERO).unwrap() {
            event::read().unwrap();
        }
    }
}

impl Drawable for TUIGraphics {
    fn init(&mut self) {
        stdout().execute(EnterAlternateScreen).unwrap();
        enable_raw_mode().unwrap();
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
        terminal.clear().unwrap();

        self.terminal = Some(terminal);
    }

    fn finalize(&mut self) {
        disable_raw_mode().unwrap();
        stdout().execute(LeaveAlternateScreen).unwrap();
    }

    fn width(&self) -> usize {
        self.width_cells
    }

    fn height(&self) -> usize {
        self.height_cells
    }

    fn is_key_pressed(&mut self, key: u8) -> bool {
        if event::poll(Duration::ZERO).unwrap() {
            if let event::Event::Key(event_key) = event::read().unwrap() {
                let &keychar = self.keymap.get(&key).unwrap();
                self.clear_event_queue();
                return event_key.code == keychar_to_keycode(keychar);
            }
        }
        false
    }

    fn should_close(&self) -> bool {
        self.close_requested
    }

    fn wait_for_key(&mut self) -> u8 {
        loop {
            let event = event::read().unwrap();

            if let event::Event::Key(key_event) = event {
                if key_event.kind != KeyEventKind::Release {
                    if let KeyCode::Char(name) = key_event.code {
                        for (&key, &keychar) in self.keymap.iter() {
                            if name.eq_ignore_ascii_case(&keychar) {
                                return key;
                            }
                        }
                    }
                } else if key_event.code == KeyCode::Char('c')
                    && key_event.modifiers == KeyModifiers::CONTROL
                {
                    self.close_requested = true;
                    return 42;
                }
            }
        }
    }

    fn draw_screen(&mut self, vram: &Vec<Vec<u8>>) {
        self.terminal
            .as_mut()
            .unwrap()
            .draw(|frame| {
                let area = frame.size();
                let buf = frame.buffer_mut();
                buf.set_style(area, (Color::Black, Color::Black));

                for (y, row) in vram.iter().enumerate() {
                    for (x, &pixel) in row.iter().enumerate() {
                        let color = if pixel == 0 {
                            Color::Black
                        } else {
                            Color::White
                        };

                        let rect = Rect::new(
                            (x * self.cell_size.0) as u16,
                            (y * self.cell_size.1) as u16,
                            self.cell_size.0 as u16,
                            self.cell_size.1 as u16,
                        );

                        buf.set_style(rect, (color, color));
                    }
                }
            })
            .unwrap();
    }

    fn update(&mut self, timeout_millis: u32) {
        if event::poll(Duration::from_millis(timeout_millis as u64)).unwrap() {
            if let event::Event::Key(key_event) = event::read().unwrap() {
                if key_event.code == KeyCode::Char('c')
                    && key_event.modifiers == KeyModifiers::CONTROL
                {
                    self.close_requested = true;
                }
            }
        }
    }

    fn sound_resume(&self) {
        todo!()
    }

    fn sound_pause(&self) {
        //todo!()
    }
}

fn keychar_to_keycode(keychar: char) -> KeyCode {
    KeyCode::Char(keychar.to_ascii_lowercase())
}
