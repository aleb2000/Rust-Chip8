mod sdl;

pub use self::sdl::SDLGraphics;

pub trait Drawable {
    fn init(&mut self);
    fn finalize(&mut self);
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn update(&mut self, timeout_millis: u32);


    // Graphics
    fn draw_screen(&mut self, vram: &Vec<Vec<u8>>);

    // Input
    fn is_key_pressed(&self, key: u8) -> bool;
    fn wait_for_key(&mut self) -> u8;
    fn should_close(&self) -> bool;

    // Sound
    fn sound_resume(&self);
    fn sound_pause(&self);
}
