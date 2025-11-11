use nalgebra_glm::Vec3;
use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::color::Color;
use crate::planet_shaders::{earth_shader, sun_shader, gas_giant_shader, mars_shader, mercury_shader, moon_shader, neptune_shader};
use crate::Uniforms;

pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex, uniforms: &Uniforms) -> Vec<Fragment> {
    let mut fragments = Vec::new();
    let (a, b, c) = (v1.transformed_position, v2.transformed_position, v3.transformed_position);

    let (min_x, min_y, max_x, max_y) = calculate_bounding_box(&a, &b, &c);
    let light_dir = Vec3::new(0.0, 0.0, -1.0).normalize();
    let triangle_area = edge_function(&a, &b, &c);

    if triangle_area.abs() < 0.1 {
        return fragments;
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let point = Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0);
            let (w1, w2, w3) = barycentric_coordinates(&point, &a, &b, &c, triangle_area);

            if w1 >= 0.0 && w2 >= 0.0 && w3 >= 0.0 {
               
                let world_pos = v1.position * w1 + v2.position * w2 + v3.position * w3;
                let interpolated_normal = (v1.transformed_normal * w1 + v2.transformed_normal * w2 + v3.transformed_normal * w3).normalize();

               
                let planet_color = if uniforms.is_moon {
                    
                    moon_shader(world_pos, interpolated_normal, light_dir, uniforms.time)
                } else {
                    match uniforms.current_shader {
                        0 => earth_shader(world_pos, interpolated_normal, light_dir, uniforms.time),
                        1 => sun_shader(world_pos, interpolated_normal, light_dir, uniforms.time),
                        2 => gas_giant_shader(world_pos, interpolated_normal, light_dir, uniforms.time),
                        3 => mars_shader(world_pos, interpolated_normal, light_dir, uniforms.time),
                        4 => mercury_shader(world_pos, interpolated_normal, light_dir, uniforms.time),
                        5 => neptune_shader(world_pos, interpolated_normal, light_dir, uniforms.time),
                        _ => Color::new(255, 0, 255), 
                    }
                };
                
                let depth = a.z * w1 + b.z * w2 + c.z * w3;
                fragments.push(Fragment::new(x as f32, y as f32, planet_color, depth));
            }
        }
    }

    fragments
}

// ... (las funciones auxiliares permanecen igual)
fn calculate_bounding_box(v1: &Vec3, v2: &Vec3, v3: &Vec3) -> (i32, i32, i32, i32) {
    let min_x = v1.x.min(v2.x).min(v3.x).floor() as i32;
    let min_y = v1.y.min(v2.y).min(v3.y).floor() as i32;
    let max_x = v1.x.max(v2.x).max(v3.x).ceil() as i32;
    let max_y = v1.y.max(v2.y).max(v3.y).ceil() as i32;

    (min_x, min_y, max_x, max_y)
}

fn barycentric_coordinates(p: &Vec3, a: &Vec3, b: &Vec3, c: &Vec3, area: f32) -> (f32, f32, f32) {
    let w1 = edge_function(b, c, p) / area;
    let w2 = edge_function(c, a, p) / area;
    let w3 = edge_function(a, b, p) / area;

    (w1, w2, w3)
}

fn edge_function(a: &Vec3, b: &Vec3, c: &Vec3) -> f32 {
    (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
}