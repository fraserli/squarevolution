#![feature(let_chains)]

mod camera;
mod grid;

use notan::draw::*;
use notan::math::*;
use notan::prelude::*;

use crate::camera::Camera;
use crate::grid::{Coord, Grid};

const CELL_SIZE: f32 = 32.0;
const STEPS_PER_SECOND: f64 = 50.0;
const STEP_TIME: u128 = (1_000_000_000.0 / STEPS_PER_SECOND) as u128;

#[derive(AppState)]
struct State {
    grid: Grid,
    camera: Camera,
    accumulator: u128,
}

#[notan_main]
fn main() -> Result<(), String> {
    let window_config = WindowConfig::new()
        .set_title("Squarevolution")
        .set_resizable(true)
        .set_multisampling(16)
        .set_vsync(true);

    notan::init_with(setup)
        .update(update)
        .event(event)
        .draw(draw)
        .add_config(DrawConfig)
        .add_config(window_config)
        .build()
}

fn setup(app: &mut App) -> State {
    State {
        grid: Grid::default(),
        camera: Camera::new(app.window().size()),
        accumulator: 0,
    }
}

fn event(app: &mut App, state: &mut State, event: Event) {
    match event {
        Event::WindowResize { width, height } => {
            state.camera.resize(width, height);
        }
        Event::MouseDown { button, x, y } => match button {
            MouseButton::Left => {
                let coord = state.camera.get_coord(x, y);
                state.grid.cycle(coord);
            }
            MouseButton::Right => state.camera.begin_pan(x, y),
            _ => {}
        },
        Event::MouseUp {
            button: MouseButton::Right,
            ..
        } => state.camera.end_pan(),
        Event::MouseWheel { delta_y, .. } => {
            if app.keyboard.is_down(KeyCode::LShift) {
                state.camera.update_zoom(delta_y);
            } else {
                let pos = app.mouse.position();
                state.camera.update_zoom_point(delta_y, pos);
            }
        }
        Event::MouseMove { x, y } => state.camera.update_pan(x, y),
        Event::KeyDown {
            key: KeyCode::Space,
        } => state.grid.step(),
        _ => {}
    }
}

fn update(app: &mut App, state: &mut State) {
    if app.keyboard.is_down(KeyCode::Return) {
        if app.keyboard.was_pressed(KeyCode::Return) {
            state.accumulator = STEP_TIME;
        } else {
            state.accumulator += app.timer.delta().as_nanos();
        }

        let n = state.accumulator / STEP_TIME;
        state.grid.multistep(n);
        state.accumulator %= STEP_TIME;
    }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.set_projection(Some(state.camera.projection()));

    let render_grid = state.camera.zoom() > 0.5;
    let c = (0.1 * state.camera.zoom()).min(0.1);

    draw.clear(if render_grid {
        Color::from_rgb(c, c, c)
    } else {
        Color::BLACK
    });

    for coord in state.camera.visible_coords() {
        let (x, y) = coord.to_f32();
        if state.grid.get(coord).is_some() {
            draw.rect((x + 0.05, y + 0.05), (0.9, 0.9))
                .color(Color::from_rgb(0.9, 0.9, 0.9));
        } else if render_grid {
            draw.rect((x + 0.05, y + 0.05), (0.9, 0.9))
                .color(Color::BLACK);
        };
    }

    gfx.render(&draw);
}
