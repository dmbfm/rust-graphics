extern crate rand;
extern crate sdl2;

use color::Color;

pub trait Video {
    fn set_pixel(&mut self, x: u32, y: u32, color: &Color);

    fn get_pixel(&mut self, x: u32, y: u32) -> Color;

    fn framebuffer_size(&self) -> (u32, u32);

    fn randomize<R: rand::RngCore>(&mut self, rng: &mut R) {
        let (w, h) = self.framebuffer_size();

        for x in 0..w {
            for y in 0..h {
                let color = Color::from_rgb(
                    (rng.next_u32() % 255) as u8,
                    (rng.next_u32() % 255) as u8,
                    (rng.next_u32() % 255) as u8,
                );

                self.set_pixel(x, y, &color);
            }
        }
    }

    fn blit(&mut self);
}
