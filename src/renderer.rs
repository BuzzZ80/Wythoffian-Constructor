use core::f32;

use crate::{DIM, Vector};
use macroquad::prelude::*;

const KEYS: [KeyCode; 10] = [
    KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4, KeyCode::Key5, 
    KeyCode::Key6, KeyCode::Key7, KeyCode::Key8, KeyCode::Key9, KeyCode::Key0
];
const ROTATION_SPEED: f32 = 2.;

pub struct Renderer {
    vertices: Vec<Vector>, 
    edges: Vec<(usize, usize)>,
    z_dist: f32, 
    zoom: f32, 
    line_width: f32,
    rotations: [f32; DIM-1],
}

impl Renderer {
    pub fn new(vertices: Vec<Vector>, edges: Vec<(usize, usize)>) -> Self {
        Self {
            vertices,
            edges,
            z_dist: 2.,
            zoom: 800.,
            line_width: 2.,
            rotations: [0.; DIM-1],
        }
    }

    pub fn draw(&self) {
        // draw
        clear_background(BLACK);

        for (a, b) in self.edges.iter() {
            let subdivisions = 12;
            for i in 0..subdivisions {
                let a_perspective: f32 = f32::lerp(self.vertices[*a].z, self.vertices[*b].z, (i as f32) / (subdivisions as f32)) + self.z_dist; 
                let b_perspective: f32 = f32::lerp(self.vertices[*a].z, self.vertices[*b].z, ((i + 1) as f32) / (subdivisions as f32)) + self.z_dist;
                
                let average_z = f32::lerp(self.vertices[*a].z, self.vertices[*b].z, ((i as f32) + 0.5) / (subdivisions as f32));
                
                let center_of_edge = self.vertices[*a].lerp(&self.vertices[*b], ((i as f32) + 0.5) / (subdivisions as f32));
                
                let average_off_axis = center_of_edge.iter()
                    .skip(3).map(|h| h*h).sum::<f32>().sqrt();
                
                draw_line(
                    self.zoom * f32::lerp(self.vertices[*a].x, self.vertices[*b].x, (i as f32) / (subdivisions as f32)) / a_perspective + screen_width()/2., 
                    self.zoom * f32::lerp(self.vertices[*a].y, self.vertices[*b].y, (i as f32) / (subdivisions as f32)) / a_perspective + screen_height()/2., 
                    self.zoom * f32::lerp(self.vertices[*a].x, self.vertices[*b].x, ((i + 1) as f32) / (subdivisions as f32)) / b_perspective + screen_width()/2., 
                    self.zoom * f32::lerp(self.vertices[*a].y, self.vertices[*b].y, ((i + 1) as f32) / (subdivisions as f32)) / b_perspective + screen_height()/2., 
                    self.line_width,
                    Color { 
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: (1.0 - clamp(average_z * 2.0 + 0.5, 0.0, 1.0)) * (1.0 - clamp(f32::abs(average_off_axis) * 2.0, 0.0, 1.0))
                    }
                );
            }
        }
    }

    pub fn handle_controls(&mut self) {
        // controls
        if is_mouse_button_down(MouseButton::Left) {
            let (dx, dy) = mouse_delta_position().into();
            let r2 = if let Some(axis) = (0..DIM-3).into_iter().map(|i| is_key_down(KEYS[i])).position(|b| b) {
                Vector::ith_axis(axis+3)
            } else {
                Vector::z_axis()
            }.normalize();

            for v in self.vertices.iter_mut() {
                *v = reflect(*v, Vector::x());
                *v = reflect(*v, Vector::x() * f32::cos(ROTATION_SPEED * dx) - r2 * f32::sin(ROTATION_SPEED * dx));
                *v = reflect(*v, Vector::y());
                *v = reflect(*v, Vector::y() * f32::cos(ROTATION_SPEED * dy) - r2 * f32::sin(ROTATION_SPEED * dy));
            }
        }

        if is_key_down(KeyCode::W) {self.zoom *= 12./11.}
        if is_key_down(KeyCode::S) {self.zoom *= 11./12.}
        if is_key_down(KeyCode::A) {self.z_dist = f32::max(1.01, self.z_dist * 35./36.) }
        if is_key_down(KeyCode::D) {self.z_dist = f32::max(1.01, self.z_dist * 36./35.) }
        if is_key_down(KeyCode::Left) {self.line_width += 0.1}
        if is_key_down(KeyCode::Right) {self.line_width -= 0.1}

        if is_key_pressed(KeyCode::Up) {
            if let Some(axis) = (0..DIM-1).into_iter().map(|i| is_key_down(KEYS[i])).position(|b| b) {
                self.rotations[axis] += 1.;
            }
        }
        if is_key_pressed(KeyCode::Down) {
            if let Some(axis) = (0..DIM-1).into_iter().map(|i| is_key_down(KEYS[i])).position(|b| b) {
                self.rotations[axis] -= 1.;
            }
        }
        for v in self.vertices.iter_mut() {
            for i in 0..DIM-1 {
                *v = reflect(*v, Vector::ith(i, 1.));
                *v = reflect(*v, Vector::ith(i, 1.) * f32::cos(ROTATION_SPEED / (8.*240.) * self.rotations[i]) - Vector::ith(i+1, 1.) * f32::sin(ROTATION_SPEED / (8.*240.) * self.rotations[i]));
            }
        }
    }
}

fn reflect(v: Vector, m: Vector) -> Vector {
    v - 2.* v.dot(&m) / m.magnitude_squared() * m
}
