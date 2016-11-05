extern crate piston;
extern crate graphics;
extern crate gfx_graphics;
extern crate gfx;
extern crate gfx_device_gl;
extern crate glutin_window;

use glutin_window::{GlutinWindow, OpenGL};
use gfx::traits::*;
use gfx::Typed;
use gfx::format::{DepthStencil, Formatted, Srgba8};
use std::path::Path;
use piston::window::{OpenGLWindow, Window, WindowSettings, Size};
use piston::input::{AfterRenderEvent, RenderEvent};
use piston::event_loop::Events;
use gfx_graphics::{Gfx2d, GlyphCache};

fn main() {
    let opengl = OpenGL::V3_2;
    let size = Size { width: 500, height: 300 };
    let samples = 4;
    let ref mut window: GlutinWindow =
        WindowSettings::new("gfx_graphics: text_test", size)
        .exit_on_esc(true)
        .opengl(opengl)
        .samples(samples)
        .build().unwrap();

    let (mut device, mut factory) = gfx_device_gl::create(|s|
        window.get_proc_address(s) as *const std::os::raw::c_void);

    let mut glyph_cache = GlyphCache::new(
        Path::new("assets/FiraSans-Regular.ttf"),
        factory.clone()
    ).unwrap();

    // Create the main color/depth targets.
    let draw_size = window.draw_size();
    let aa = samples as gfx::tex::NumSamples;
    let dim = (draw_size.width as u16, draw_size.height as u16, 1, aa.into());
    let color_format = <Srgba8 as Formatted>::get_format();
    let depth_format = <DepthStencil as Formatted>::get_format();
    let (output_color, output_stencil) =
        gfx_device_gl::create_main_targets_raw(dim,
                                               color_format.0,
                                               depth_format.0);
    let output_color = Typed::new(output_color);
    let output_stencil = Typed::new(output_stencil);

    let mut encoder = factory.create_command_buffer().into();
    let mut g2d = Gfx2d::new(opengl, &mut factory);
    let mut events = window.events();

    while let Some(e) = events.next(window) {
        if let Some(args) = e.render_args() {
            g2d.draw(&mut encoder, &output_color, &output_stencil, args.viewport(), |c, g| {
                use graphics::*;

                clear([1.0; 4], g);
                text::Text::new_color([0.0, 0.5, 0.0, 1.0], 32).draw(
                    "Hello gfx_graphics!",
                    &mut glyph_cache,
                    &DrawState::default(),
                    c.transform.trans(10.0, 100.0),
                    g
                );
            });
            encoder.flush(&mut device);
        }

        if let Some(_) = e.after_render_args() {
            device.cleanup();
        }
    }
}
