extern crate stdweb;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::CanvasRenderingContext2d;
use stdweb::UnsafeTypedArray;

use color::Color;
use video::Video;

pub struct CanvasVideo {
    framebuffer_size: (u32, u32),
    output_size: (u32, u32),
    canvas: CanvasElement,
    context: CanvasRenderingContext2d,
    framebuffer: Vec<u32>,
}

impl CanvasVideo {
    pub fn new(framebuffer_size: (u32, u32), output_size: (u32, u32)) -> Self {
        let canvas: CanvasElement = stdweb::web::document()
            .create_element("canvas")
            .unwrap()
            .try_into()
            .unwrap();
        canvas.set_width(framebuffer_size.0);
        canvas.set_height(framebuffer_size.1);

        js! {
          var canvas = @{&canvas};
          canvas.style.width = @{output_size.0} + "px";
          canvas.style.height = @{output_size.1} + "px";
        };

        let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
        let framebuffer: Vec<u32> = vec![0xff; (framebuffer_size.0 * framebuffer_size.1) as usize];

        stdweb::web::document()
            .body()
            .unwrap()
            .append_child(&canvas);

        CanvasVideo {
            framebuffer_size,
            output_size,
            canvas,
            context,
            framebuffer,
        }
    }
}

impl Video for CanvasVideo {
    fn set_pixel(&mut self, x: u32, y: u32, color: &Color) {
        self.framebuffer[(y * self.framebuffer_size.0 + x) as usize] = color.pack();
    }

    fn get_pixel(&mut self, x: u32, y: u32) -> Color {
        Color::unpack(self.framebuffer[(y * self.framebuffer_size.0 + x) as usize])
    }

    fn framebuffer_size(&self) -> (u32, u32) {
        self.framebuffer_size
    }

    fn blit(&mut self) {
        js! {
          var canvas = @{&self.canvas};
          var framebuffer = @{unsafe { UnsafeTypedArray::new(&self.framebuffer) }};
          var ctx = canvas.getContext("2d");
          var imageData = ctx.createImageData(@{self.framebuffer_size.0}, @{self.framebuffer_size.1});
          var videoData = new Uint32Array(imageData.data.buffer);
          videoData.set(framebuffer);
          ctx.putImageData(imageData, 0, 0);
          return null;
        };
    }
}
