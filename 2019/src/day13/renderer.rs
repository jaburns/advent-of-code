use super::state::{Game, Tile};
use glium::glutin::dpi::LogicalSize;
use glium::glutin::{ContextBuilder, Event, EventsLoop, WindowBuilder, WindowEvent};
use glium::texture::srgb_texture2d::SrgbTexture2d;
use glium::{index, texture, uniforms};
use glium::{Display, Program, Rect, Surface, VertexBuffer};
use std::sync::mpsc::{Receiver, TryRecvError};
use std::sync::{Arc, Mutex};

const WINDOW_SCALE: u32 = 20;

fn tile_to_texel(tile: Tile) -> (u8, u8, u8, u8) {
    fn from_hex(color: u32) -> (u8, u8, u8, u8) {
        (
            ((color & 0xff_00_00) >> 16) as u8,
            ((color & 0xff_00) >> 8) as u8,
            (color & 0xff) as u8,
            0xff,
        )
    }

    match tile {
        Tile::Empty => from_hex(0x35_5c_7d),
        Tile::Wall => from_hex(0x6c_5b_7b),
        Tile::Block => from_hex(0xc0_6c_84),
        Tile::Paddle => from_hex(0xff_ff_ff),
        Tile::Ball => from_hex(0xff_ff_ff),
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

fn get_quad() -> Vec<Vertex> {
    vec![
        Vertex {
            position: [-1.0, -1.0],
        },
        Vertex {
            position: [-1.0, 1.0],
        },
        Vertex {
            position: [1.0, -1.0],
        },
        Vertex {
            position: [1.0, 1.0],
        },
    ]
}

pub fn run(width: u32, height: u32, tick_rx: Receiver<()>, shared_state: Arc<Mutex<Game>>) {
    let mut event_loop = EventsLoop::new();

    let win_size = LogicalSize::new(
        (width * WINDOW_SCALE) as f64,
        (height * WINDOW_SCALE) as f64,
    );

    let wb = WindowBuilder::new()
        .with_title("Breakout")
        .with_dimensions(win_size)
        .with_min_dimensions(win_size)
        .with_max_dimensions(win_size);

    let cb = ContextBuilder::new();
    let display = Display::new(wb, cb, &event_loop).unwrap();

    let vertex_buffer = VertexBuffer::new(&display, &get_quad()).unwrap();
    let indices = index::NoIndices(index::PrimitiveType::TriangleStrip);

    let vertex_shader_src = include_str!("shader.vert");
    let fragment_shader_src = include_str!("shader.frag");
    let program =
        Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let state_texture = SrgbTexture2d::empty_with_format(
        &display,
        texture::SrgbFormat::U8U8U8U8,
        texture::MipmapsOption::NoMipmap,
        height,
        width,
    )
    .unwrap();

    let mut closed = false;

    while !closed {
        match tick_rx.try_recv() {
            Ok(()) => {
                let game = shared_state.lock().unwrap();
                state_texture.write(
                    Rect {
                        left: 0,
                        bottom: 0,
                        width: game.height() as u32,
                        height: game.width() as u32,
                    },
                    game.map_tiles(tile_to_texel),
                );
            }
            Err(TryRecvError::Disconnected) => closed = true,
            _ => (),
        };

        event_loop.poll_events(|event| {
            if let Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } = event
            {
                closed = true;
            }
        });

        let uniforms = uniform! {
            window_size: ((width * WINDOW_SCALE) as f32, (height * WINDOW_SCALE) as f32),
            tex: uniforms::Sampler::new(&state_texture)
                    .wrap_function(uniforms::SamplerWrapFunction::Clamp)
                    .magnify_filter(uniforms::MagnifySamplerFilter::Nearest)
                    .minify_filter(uniforms::MinifySamplerFilter::Nearest)
        };

        let mut target = display.draw();
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    }
}
