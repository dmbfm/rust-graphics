extern crate rand;
extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::surface::Surface;

use color::Color;
use video::Video;

pub struct SDLVideo<'a> {
    framebuffer_size: (u32, u32),
    output_size: (u32, u32),
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    framebuffer: Surface<'a>,
}

impl<'a> SDLVideo<'a> {
    pub fn new(
        sdl_video_context: &sdl2::VideoSubsystem,
        title: &str,
        framebuffer_size: (u32, u32),
        output_size: (u32, u32),
    ) -> Self {
        let window = sdl_video_context
            .window(title, output_size.0, output_size.1)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let framebuffer = Surface::new(
            framebuffer_size.0,
            framebuffer_size.1,
            sdl2::pixels::PixelFormatEnum::RGBA8888,
        ).unwrap();

        SDLVideo {
            framebuffer_size,
            output_size,
            canvas,
            framebuffer,
        }
    }
}

impl<'a> Video for SDLVideo<'a> {
    fn set_pixel(&mut self, x: u32, y: u32, color: &Color) {
        let (w, _) = self.framebuffer_size;
        let offset = ((w * y + x) * 4) as usize;

        self.framebuffer.with_lock_mut(|data: &mut [u8]| {
            data[offset] = color.a;
            data[offset + 1] = color.b;
            data[offset + 2] = color.g;
            data[offset + 3] = color.r;
        });
    }

    fn framebuffer_size(&self) -> (u32, u32) {
        self.framebuffer_size
    }

    fn get_pixel(&mut self, x: u32, y: u32) -> Color {
        let (w, _) = self.framebuffer_size;
        let offset = ((w * y + x) * 4) as usize;

        let data = self.framebuffer.without_lock().unwrap();

        Color::new(
            data[offset + 3],
            data[offset + 2],
            data[offset + 1],
            data[offset],
        )
    }

    fn blit(&mut self) {
        let (fb_w, fb_h) = self.framebuffer_size;
        let (w, h) = self.output_size;

        let texture_creator = self.canvas.texture_creator();
        let surface_texture = texture_creator
            .create_texture_from_surface(&self.framebuffer)
            .unwrap();

        self.canvas
            .copy(
                &surface_texture,
                Rect::new(0, 0, fb_w, fb_h),
                Rect::new(0, 0, w, h),
            )
            .unwrap();
        self.canvas.present();
    }
}
