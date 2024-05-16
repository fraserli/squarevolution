use notan::math::*;

use crate::{Coord, CELL_SIZE};

const ZOOM_MAXIMUM: f32 = 100.0;
const ZOOM_MINIMUM: f32 = 0.01;
const ZOOM_SENSITIVITY: f32 = 0.1;

pub struct Camera {
    position: Vec2,
    zoom: f32,
    window: Vec2,
    pan: Option<Vec2>,
}

impl Camera {
    pub fn new((width, height): (u32, u32)) -> Self {
        Self {
            position: Vec2::ZERO,
            zoom: 1.0,
            window: vec2(width as f32, height as f32),
            pan: None,
        }
    }

    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    /// Sets the window dimensions.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.window = vec2(width as f32, height as f32);
    }

    pub fn update_zoom(&mut self, dir: f32) {
        let r = 1.0 + ZOOM_SENSITIVITY * dir.signum();
        self.zoom = (self.zoom * r).clamp(ZOOM_MINIMUM, ZOOM_MAXIMUM);
    }

    /// Zooms relative to a fixed point.
    pub fn update_zoom_point(&mut self, dir: f32, p: (f32, f32)) {
        let pa = self.gridspace().transform_point2(p.into());
        self.update_zoom(dir);
        let pb = self.gridspace().transform_point2(p.into());
        self.position += pb - pa;
    }

    pub fn begin_pan(&mut self, x: i32, y: i32) {
        self.pan = Some(self.gridspace().transform_point2(vec2(x as f32, y as f32)));
    }

    pub fn update_pan(&mut self, x: i32, y: i32) {
        if let Some(begin) = self.pan {
            let end = self.gridspace().transform_point2(vec2(x as f32, y as f32));
            self.position += end - begin;
        }
    }

    pub fn end_pan(&mut self) {
        self.pan = None;
    }

    /// Calculates a transformation from window coordinates to grid coordinates.
    pub fn gridspace(&self) -> Affine2 {
        let scale = vec2(1.0, -1.0) / (CELL_SIZE * self.zoom);
        Affine2::from_scale_angle_translation(
            scale,
            0.0,
            -self.position - scale * 0.5 * self.window,
        )
    }

    /// Converts window coordinates to a grid cell coordinate.
    pub fn get_coord(&self, x: i32, y: i32) -> Coord {
        let p = self.gridspace().transform_point2(vec2(x as f32, y as f32));
        Coord {
            x: p.x.floor() as i32,
            y: p.y.floor() as i32,
        }
    }

    /// Calculates the projection matrix.
    pub fn projection(&self) -> Mat4 {
        let scale = self.zoom
            * vec3(
                CELL_SIZE * 2.0 / self.window.x,
                CELL_SIZE * 2.0 / self.window.y,
                1.0,
            );
        let rotation = Quat::IDENTITY;
        let translation = Vec3::from((self.position * scale.xy(), 0.0));
        Mat4::from_scale_rotation_translation(scale, rotation, translation)
    }

    /// Gets the coordinates of all the grid cells which are visible.
    pub fn visible_coords(&self) -> impl Iterator<Item = Coord> {
        let min = self.get_coord(0, self.window.y as i32);
        let max = self.get_coord(self.window.x as i32, 0);

        (min.y..=max.y).flat_map(move |y| (min.x..=max.x).map(move |x| Coord { x, y }))
    }
}
