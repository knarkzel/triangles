use pixels::{Error, Pixels, SurfaceTexture};
use std::sync::atomic::{AtomicU32, Ordering};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Triangles")
            .with_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    let mut world = Triangles::new();

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.get_frame());
            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
            let speed = WIDTH / world.zoom / 2;

            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(VirtualKeyCode::Left) {
                world.x -= speed;
            }

            if input.key_pressed(VirtualKeyCode::Right) {
                world.x += speed;
            }

            if input.key_pressed(VirtualKeyCode::Down) {
                world.y += speed;
            }

            if input.key_pressed(VirtualKeyCode::Up) {
                world.y -= speed;
            }

            if input.key_pressed(VirtualKeyCode::Plus) {
                world.zoom += 1;
            }

            if input.key_pressed(VirtualKeyCode::Minus) {
                if world.zoom > 1 {
                    world.zoom -= 1;
                }
            }

            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            window.request_redraw();
        }
    });
}

static FRAME: AtomicU32 = AtomicU32::new(0);

struct Triangles {
    x: u32,
    y: u32,
    zoom: u32,
}

impl Triangles {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            zoom: 1,
        }
    }

    fn draw(&self, frame: &mut [u8]) {
        let f = FRAME.fetch_add(1, Ordering::Relaxed);
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = ((i % WIDTH as usize) as u32 + self.x) / self.zoom;
            let y = ((i / WIDTH as usize) as u32 + self.y) / self.zoom;
            let mut output = f.wrapping_add((x ^ y) as u32) | 0xFF_00_00_00;
            output ^= output.wrapping_shl(((x | y) as f32).tan() as u32) * 2;
            pixel.copy_from_slice(&output.to_be_bytes());
        }
    }
}
