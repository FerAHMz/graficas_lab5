use nalgebra_glm::{Vec3, dot};
use crate::vertex::Vertex;
use crate::color::Color;
use crate::sphere::Sphere;
use std::time::Instant;

#[derive(Clone)]
pub enum PlanetType {
    Star,           // Sol con efectos de fuego
    RockyPlanet,    // Planeta rocoso tipo Tierra
    GasGiant,       // Gigante gaseoso con anillos
    IcePlanet,      // Planeta helado
    VolcanicPlanet, // Planeta volcánico
    RingedPlanet,   // Planeta con anillos prominentes
}

pub struct Planet {
    pub sphere: Sphere,
    pub planet_type: PlanetType,
    pub position: Vec3,
    pub scale: f32,
    pub rotation_speed: f32,
    pub orbital_speed: f32,
    pub orbital_radius: f32,
    pub current_rotation: f32,
    pub current_orbital_angle: f32,
}

pub struct Moon {
    pub sphere: Sphere,
    pub orbit_center: Vec3,
    pub orbital_radius: f32,
    pub orbital_speed: f32,
    pub current_angle: f32,
    pub scale: f32,
}

pub struct Ring {
    pub vertices: Vec<Vertex>,
}

impl Planet {
    pub fn new(
        planet_type: PlanetType, 
        position: Vec3, 
        scale: f32, 
        rotation_speed: f32,
        orbital_speed: f32,
        orbital_radius: f32
    ) -> Self {
        let sphere = Sphere::new(1.0, 32, 32); // Base unit sphere
        
        Planet {
            sphere,
            planet_type,
            position,
            scale,
            rotation_speed,
            orbital_speed,
            orbital_radius,
            current_rotation: 0.0,
            current_orbital_angle: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.current_rotation += self.rotation_speed * delta_time;
        self.current_orbital_angle += self.orbital_speed * delta_time;
        
        // Update position based on orbital motion
        if self.orbital_radius > 0.0 {
            self.position.x = self.orbital_radius * self.current_orbital_angle.cos();
            self.position.z = self.orbital_radius * self.current_orbital_angle.sin();
        }
    }

    pub fn get_current_position(&self) -> Vec3 {
        self.position
    }
}

impl Moon {
    pub fn new(orbit_center: Vec3, orbital_radius: f32, orbital_speed: f32, scale: f32) -> Self {
        let sphere = Sphere::new(1.0, 16, 16); // Smaller detail for moon
        
        Moon {
            sphere,
            orbit_center,
            orbital_radius,
            orbital_speed,
            current_angle: 0.0,
            scale,
        }
    }

    pub fn update(&mut self, delta_time: f32, planet_position: Vec3) {
        self.orbit_center = planet_position;
        self.current_angle += self.orbital_speed * delta_time;
    }

    pub fn get_current_position(&self) -> Vec3 {
        Vec3::new(
            self.orbit_center.x + self.orbital_radius * self.current_angle.cos(),
            self.orbit_center.y,
            self.orbit_center.z + self.orbital_radius * self.current_angle.sin(),
        )
    }
}

impl Ring {
    pub fn new(inner_radius: f32, outer_radius: f32, segments: u32) -> Self {
        let mut vertices = Vec::new();
        
        for i in 0..segments {
            let angle1 = (i as f32 / segments as f32) * 2.0 * std::f32::consts::PI;
            let angle2 = ((i + 1) as f32 / segments as f32) * 2.0 * std::f32::consts::PI;
            
            let cos1 = angle1.cos();
            let sin1 = angle1.sin();
            let cos2 = angle2.cos();
            let sin2 = angle2.sin();
            
            // Inner vertices
            let inner1 = Vec3::new(inner_radius * cos1, 0.0, inner_radius * sin1);
            let inner2 = Vec3::new(inner_radius * cos2, 0.0, inner_radius * sin2);
            
            // Outer vertices
            let outer1 = Vec3::new(outer_radius * cos1, 0.0, outer_radius * sin1);
            let outer2 = Vec3::new(outer_radius * cos2, 0.0, outer_radius * sin2);
            
            let normal = Vec3::new(0.0, 1.0, 0.0);
            
            // First triangle
            vertices.push(Vertex::new(inner1, normal, nalgebra_glm::Vec2::new(0.0, 0.0)));
            vertices.push(Vertex::new(outer1, normal, nalgebra_glm::Vec2::new(1.0, 0.0)));
            vertices.push(Vertex::new(inner2, normal, nalgebra_glm::Vec2::new(0.0, 1.0)));
            
            // Second triangle
            vertices.push(Vertex::new(inner2, normal, nalgebra_glm::Vec2::new(0.0, 1.0)));
            vertices.push(Vertex::new(outer1, normal, nalgebra_glm::Vec2::new(1.0, 0.0)));
            vertices.push(Vertex::new(outer2, normal, nalgebra_glm::Vec2::new(1.0, 1.0)));
        }
        
        Ring { vertices }
    }
}

// Shader functions for different planet types
pub fn star_shader(
    position: Vec3, 
    normal: Vec3, 
    uv: nalgebra_glm::Vec2, 
    time: f32
) -> Color {
    // 4-layer star shader for maximum points
    
    // Layer 1: Core temperature gradient
    let distance_from_center = (uv.x - 0.5).powi(2) + (uv.y - 0.5).powi(2);
    let core_intensity = 1.0 - (distance_from_center * 4.0).min(1.0);
    
    // Layer 2: Plasma turbulence
    let turbulence = ((uv.x * 10.0 + time * 2.0).sin() * 
                     (uv.y * 8.0 + time * 1.5).cos() * 
                     (time * 3.0).sin()).abs();
    
    // Layer 3: Solar flares
    let flare_noise = ((uv.x * 15.0 + time * 4.0).sin() + 
                      (uv.y * 12.0 + time * 3.0).cos()).abs();
    
    // Layer 4: Corona effect
    let corona = (1.0 - distance_from_center).powf(0.5) * 
                (time * 2.0 + uv.x * 20.0).sin().abs();
    
    // Combine layers
    let red = (255.0 * (0.9 + 0.1 * core_intensity + 0.2 * flare_noise)).min(255.0) as u8;
    let green = (255.0 * (0.6 * core_intensity + 0.3 * turbulence + 0.1 * corona)).min(255.0) as u8;
    let blue = (100.0 * (0.2 * core_intensity + 0.5 * turbulence)).min(255.0) as u8;
    
    Color::new(red, green, blue)
}

pub fn rocky_planet_shader(
    position: Vec3, 
    normal: Vec3, 
    uv: nalgebra_glm::Vec2, 
    time: f32
) -> Color {
    // 4-layer rocky planet shader
    
    // Layer 1: Continental masses
    let continent_noise = ((uv.x * 8.0).sin() * (uv.y * 6.0).cos()).abs();
    let is_land = continent_noise > 0.3;
    
    // Layer 2: Ocean depth
    let ocean_depth = ((uv.x * 15.0).sin() + (uv.y * 12.0).cos()) * 0.5 + 0.5;
    
    // Layer 3: Cloud cover
    let cloud_cover = ((uv.x * 20.0 + time * 0.5).sin() * 
                      (uv.y * 18.0 + time * 0.7).cos()).abs();
    
    // Layer 4: Polar ice caps
    let polar_factor = (uv.y - 0.5).abs() * 2.0;
    let ice_caps = polar_factor > 0.8;
    
    if ice_caps {
        Color::new(240, 248, 255) // Ice white
    } else if is_land {
        if cloud_cover > 0.6 {
            Color::new(220, 220, 220) // Clouds
        } else {
            // Land colors
            let red = (120.0 + 60.0 * continent_noise) as u8;
            let green = (80.0 + 40.0 * continent_noise) as u8;
            let blue = 40;
            Color::new(red, green, blue)
        }
    } else {
        // Ocean colors
        let blue_intensity = (100.0 + 155.0 * ocean_depth) as u8;
        Color::new(30, 60, blue_intensity)
    }
}

pub fn gas_giant_shader(
    position: Vec3, 
    normal: Vec3, 
    uv: nalgebra_glm::Vec2, 
    time: f32
) -> Color {
    // 4-layer gas giant shader
    
    // Layer 1: Atmospheric bands
    let band_position = (uv.y * 12.0).sin();
    let band_intensity = band_position.abs();
    
    // Layer 2: Storm systems
    let storm_noise = ((uv.x * 25.0 + time * 1.0).sin() * 
                      (uv.y * 15.0).cos()).abs();
    
    // Layer 3: Great Red Spot equivalent
    let spot_x = uv.x - 0.7;
    let spot_y = uv.y - 0.4;
    let spot_distance = (spot_x * spot_x + spot_y * spot_y).sqrt();
    let great_spot = if spot_distance < 0.15 { 1.0 } else { 0.0 };
    
    // Layer 4: Atmospheric turbulence
    let turbulence = ((uv.x * 30.0 + time * 2.0).sin() + 
                     (uv.y * 20.0 + time * 1.5).cos()) * 0.5 + 0.5;
    
    if great_spot > 0.0 {
        Color::new(200, 100, 50) // Great Red Spot
    } else {
        let base_orange = 200.0 * band_intensity;
        let base_brown = 150.0 * band_intensity;
        let white_storms = 100.0 * storm_noise;
        
        let red = (base_orange + white_storms + 50.0 * turbulence).min(255.0) as u8;
        let green = (base_brown + white_storms * 0.7 + 30.0 * turbulence).min(255.0) as u8;
        let blue = (80.0 + white_storms * 0.3 + 20.0 * turbulence).min(255.0) as u8;
        
        Color::new(red, green, blue)
    }
}

pub fn ice_planet_shader(
    position: Vec3, 
    normal: Vec3, 
    uv: nalgebra_glm::Vec2, 
    time: f32
) -> Color {
    // 4-layer ice planet shader
    
    // Layer 1: Ice crystal formations
    let crystal_pattern = ((uv.x * 20.0).sin() * (uv.y * 15.0).cos()).abs();
    
    // Layer 2: Frozen ocean cracks
    let crack_noise = ((uv.x * 40.0).sin() + (uv.y * 35.0).cos()).abs();
    let has_cracks = crack_noise > 0.8;
    
    // Layer 3: Aurora-like subsurface glow
    let aurora = ((uv.y * 8.0 + time * 2.0).sin() * 
                 (uv.x * 6.0 + time * 1.5).cos()).abs();
    
    // Layer 4: Surface frost variation
    let frost_variation = ((uv.x * 12.0).sin() + (uv.y * 10.0).cos()) * 0.5 + 0.5;
    
    if has_cracks {
        let blue_glow = (150.0 + 50.0 * aurora) as u8;
        Color::new(100, 150, blue_glow) // Deep ice cracks with subsurface ocean
    } else {
        let ice_brightness = (200.0 + 55.0 * crystal_pattern * frost_variation) as u8;
        let blue_tint = (220.0 + 35.0 * aurora) as u8;
        Color::new(ice_brightness, ice_brightness, blue_tint)
    }
}

pub fn volcanic_planet_shader(
    position: Vec3, 
    normal: Vec3, 
    uv: nalgebra_glm::Vec2, 
    time: f32
) -> Color {
    // 4-layer volcanic planet shader
    
    // Layer 1: Lava flows
    let lava_flow = ((uv.x * 12.0 + time * 3.0).sin() * 
                    (uv.y * 8.0 + time * 2.0).cos()).abs();
    
    // Layer 2: Volcanic rock formations
    let rock_texture = ((uv.x * 25.0).sin() * (uv.y * 20.0).cos()).abs();
    
    // Layer 3: Active volcanic eruptions
    let eruption_noise = ((uv.x * 15.0 + time * 5.0).sin() + 
                         (uv.y * 10.0 + time * 4.0).cos()).abs();
    let active_volcano = eruption_noise > 0.7;
    
    // Layer 4: Ash and smoke
    let ash_clouds = ((uv.x * 30.0 + time * 1.0).sin() * 
                     (uv.y * 25.0 + time * 0.8).cos()).abs();
    
    if active_volcano {
        let bright_red = (255.0 * eruption_noise) as u8;
        let bright_yellow = (200.0 * eruption_noise) as u8;
        Color::new(bright_red, bright_yellow, 50) // Bright lava
    } else if lava_flow > 0.6 {
        Color::new(200, 80, 20) // Lava flows
    } else if ash_clouds > 0.5 {
        let ash_level = (80.0 + 40.0 * ash_clouds) as u8;
        Color::new(ash_level, ash_level, ash_level) // Ash clouds
    } else {
        // Volcanic rock
        let rock_red = (60.0 + 40.0 * rock_texture) as u8;
        let rock_brown = (40.0 + 30.0 * rock_texture) as u8;
        Color::new(rock_red, rock_brown, 20)
    }
}

pub fn ringed_planet_shader(
    position: Vec3, 
    normal: Vec3, 
    uv: nalgebra_glm::Vec2, 
    time: f32
) -> Color {
    // 4-layer ringed planet shader (like Saturn)
    
    // Layer 1: Atmospheric bands
    let band_pattern = (uv.y * 15.0).sin().abs();
    
    // Layer 2: Hexagonal polar storm (like Saturn's north pole)
    let hex_x = uv.x - 0.5;
    let hex_y = uv.y - 0.8; // Near north pole
    let hex_distance = (hex_x * hex_x + hex_y * hex_y).sqrt();
    let hexagon_storm = if hex_distance < 0.1 { 1.0 } else { 0.0 };
    
    // Layer 3: Wind patterns
    let wind_streams = ((uv.x * 20.0 + time * 0.5).sin() * 
                       (uv.y * 8.0).cos()).abs();
    
    // Layer 4: Atmospheric composition colors
    let methane_haze = ((uv.y * 10.0).cos() + 1.0) * 0.5;
    
    if hexagon_storm > 0.0 {
        Color::new(100, 150, 200) // Hexagonal storm
    } else {
        let yellow_base = (200.0 * band_pattern + 30.0 * methane_haze) as u8;
        let brown_bands = (150.0 * band_pattern + 50.0 * wind_streams) as u8;
        let blue_tint = (120.0 * methane_haze + 20.0 * wind_streams) as u8;
        
        Color::new(yellow_base, brown_bands, blue_tint)
    }
}

pub fn moon_shader(
    position: Vec3, 
    normal: Vec3, 
    uv: nalgebra_glm::Vec2, 
    time: f32
) -> Color {
    // Simple moon shader with craters
    let crater_noise = ((uv.x * 30.0).sin() * (uv.y * 25.0).cos()).abs();
    let has_crater = crater_noise > 0.7;
    
    if has_crater {
        Color::new(80, 80, 90) // Dark crater
    } else {
        let brightness = (150.0 + 50.0 * crater_noise) as u8;
        Color::new(brightness, brightness, brightness - 10) // Gray moon surface
    }
}

pub fn ring_shader(
    position: Vec3, 
    normal: Vec3, 
    uv: nalgebra_glm::Vec2, 
    time: f32
) -> Color {
    // Ring particles shader
    let distance_from_center = (position.x * position.x + position.z * position.z).sqrt();
    let ring_density = ((distance_from_center * 50.0).sin()).abs();
    
    let particle_noise = ((position.x * 100.0 + time * 0.1).sin() * 
                         (position.z * 80.0).cos()).abs();
    
    let alpha_equivalent = (100.0 * ring_density * particle_noise) as u8;
    let ice_color = (150.0 + 50.0 * particle_noise) as u8;
    
    Color::new(ice_color, ice_color - 20, ice_color - 10)
}