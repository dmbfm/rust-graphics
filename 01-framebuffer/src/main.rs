#![recursion_limit = "128"]

extern crate rand;
extern crate sdl2;

#[cfg(target_os = "emscripten")]
#[macro_use]
extern crate stdweb;

#[allow(dead_code)]
mod video;

#[allow(dead_code)]
mod color;

#[cfg(target_os = "emscripten")]
mod emscripten;

#[cfg(target_os = "emscripten")]
mod canvas_video;

mod sdl_video;

use rand::rngs::SmallRng;
use rand::FromEntropy;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::SystemTime;

use sdl_video::SDLVideo;
use video::Video;

fn main() {
    let sdl_context = sdl2::init().unwrap();

    let video_subsystem = sdl_context.video().unwrap();

    let resolution = (320, 200);
    let output = (960, 600);

    let mut video = SDLVideo::new(
        &video_subsystem,
        "rust-framebuffer: 01-basic",
        resolution,
        output,
    );

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut current_time = SystemTime::now();

    let mut frames = 0;

    let mut small_rng = SmallRng::from_entropy();

    let mut main_loop = move || {
        frames += 1;
        let ellapsed = current_time.elapsed().unwrap();
        let secs = ellapsed.as_secs();

        if secs > 0 {
            println!("FPS: {}", frames);
            frames = 0;
            current_time = SystemTime::now();
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    std::process::exit(1);
                }
                _ => {}
            }
        }

        video.randomize(&mut small_rng);
        video.blit();
    };

    #[cfg(target_os = "emscripten")]
    println!("EMSCRIPTEN!");

    #[cfg(target_os = "emscripten")]
    emscripten::emscripten::set_main_loop_callback(main_loop);

    #[cfg(not(target_os = "emscripten"))]
    loop {
        main_loop();
    }
}
