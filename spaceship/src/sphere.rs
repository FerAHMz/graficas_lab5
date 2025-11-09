use nalgebra_glm::{Vec2, Vec3};
use std::f32::consts::PI;
use crate::vertex::Vertex;

pub struct Sphere {
    pub vertices: Vec<Vertex>,
}

impl Sphere {
    pub fn new(radius: f32, latitude_segments: u32, longitude_segments: u32) -> Self {
        let mut vertices = Vec::new();

        // Generate sphere vertices using spherical coordinates
        for lat in 0..=latitude_segments {
            let theta = lat as f32 * PI / latitude_segments as f32;
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();

            for lon in 0..=longitude_segments {
                let phi = lon as f32 * 2.0 * PI / longitude_segments as f32;
                let sin_phi = phi.sin();
                let cos_phi = phi.cos();

                // Position on unit sphere
                let x = cos_phi * sin_theta;
                let y = cos_theta;
                let z = sin_phi * sin_theta;

                let position = Vec3::new(x * radius, y * radius, z * radius);
                let normal = Vec3::new(x, y, z); // Normal is the same as position for unit sphere
                let tex_coords = Vec2::new(
                    lon as f32 / longitude_segments as f32,
                    lat as f32 / latitude_segments as f32,
                );

                vertices.push(Vertex::new(position, normal, tex_coords));
            }
        }

        // Generate triangulated faces
        let mut triangulated_vertices = Vec::new();
        
        for lat in 0..latitude_segments {
            for lon in 0..longitude_segments {
                let first = (lat * (longitude_segments + 1) + lon) as usize;
                let second = first + longitude_segments as usize + 1;

                // First triangle
                triangulated_vertices.push(vertices[first].clone());
                triangulated_vertices.push(vertices[second].clone());
                triangulated_vertices.push(vertices[first + 1].clone());

                // Second triangle
                triangulated_vertices.push(vertices[second].clone());
                triangulated_vertices.push(vertices[second + 1].clone());
                triangulated_vertices.push(vertices[first + 1].clone());
            }
        }

        Sphere {
            vertices: triangulated_vertices,
        }
    }

    pub fn get_vertex_array(&self) -> &Vec<Vertex> {
        &self.vertices
    }
}