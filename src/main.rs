use nalgebra_glm::{Vec3, Mat4};
use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};
use std::f32::consts::PI;

mod framebuffer;
mod triangle;
mod vertex;
mod color;
mod fragment;
mod shaders;
mod noise;
mod planet_shaders;

use framebuffer::Framebuffer;
use vertex::Vertex;
use triangle::triangle;
use shaders::vertex_shader;

pub struct Uniforms {
    model_matrix: Mat4,
    time: f32,
    current_shader: u32,
    is_moon: bool,
}

fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = Mat4::new(
        1.0,  0.0,    0.0,   0.0,
        0.0,  cos_x, -sin_x, 0.0,
        0.0,  sin_x,  cos_x, 0.0,
        0.0,  0.0,    0.0,   1.0,
    );

    let rotation_matrix_y = Mat4::new(
        cos_y,  0.0,  sin_y, 0.0,
        0.0,    1.0,  0.0,   0.0,
        -sin_y, 0.0,  cos_y, 0.0,
        0.0,    0.0,  0.0,   1.0,
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z,  cos_z, 0.0, 0.0,
        0.0,    0.0,  1.0, 0.0,
        0.0,    0.0,  0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale, 0.0,   0.0,   translation.x,
        0.0,   scale, 0.0,   translation.y,
        0.0,   0.0,   scale, translation.z,
        0.0,   0.0,   0.0,   1.0,
    );

    transform_matrix * rotation_matrix
}

// Generar esfera proceduralmente
fn create_sphere(radius: f32, segments: u32) -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Generar vértices
    for lat in 0..=segments {
        let theta = lat as f32 * PI / segments as f32;
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        for lon in 0..=segments {
            let phi = lon as f32 * 2.0 * PI / segments as f32;
            let sin_phi = phi.sin();
            let cos_phi = phi.cos();

            let x = radius * sin_theta * cos_phi;
            let y = radius * cos_theta;
            let z = radius * sin_theta * sin_phi;

            let position = Vec3::new(x, y, z);
            let normal = position.normalize();

            vertices.push(Vertex::new(position, normal, nalgebra_glm::Vec2::new(0.0, 0.0)));
        }
    }

    // Generar índices para triángulos
    for lat in 0..segments {
        for lon in 0..segments {
            let first = lat * (segments + 1) + lon;
            let second = first + segments + 1;

            // Primer triángulo
            indices.push(first);
            indices.push(second);
            indices.push(first + 1);

            // Segundo triángulo
            indices.push(second);
            indices.push(second + 1);
            indices.push(first + 1);
        }
    }

    (vertices, indices)
}

fn render_planet(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertices: &[Vertex], indices: &[u32]) {
    // Vertex Shader Stage
    let mut transformed_vertices = Vec::with_capacity(vertices.len());
    for vertex in vertices {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Rasterization Stage
    for triangle_idx in (0..indices.len()).step_by(3) {
        if triangle_idx + 2 < indices.len() {
            let i1 = indices[triangle_idx] as usize;
            let i2 = indices[triangle_idx + 1] as usize;
            let i3 = indices[triangle_idx + 2] as usize;

            if i1 < transformed_vertices.len() && i2 < transformed_vertices.len() && i3 < transformed_vertices.len() {
                let v1 = &transformed_vertices[i1];
                let v2 = &transformed_vertices[i2];
                let v3 = &transformed_vertices[i3];

                let fragments = triangle(v1, v2, v3, uniforms);
                
                for fragment in fragments {
                    let x = fragment.position.x as usize;
                    let y = fragment.position.y as usize;
                    if x < framebuffer.width && y < framebuffer.height {
                        let color = fragment.color.to_hex();
                        framebuffer.set_current_color(color);
                        framebuffer.point(x, y, fragment.depth);
                    }
                }
            }
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let start_time = Instant::now();

    let mut framebuffer = Framebuffer::new(window_width, window_height);
    let mut window = Window::new(
        "Lab Planetas - Tierra con Luna",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    framebuffer.set_background_color(0x000008);

    // Parámetros de transformación para la Tierra
    let mut translation = Vec3::new(400.0, 300.0, 0.0);
    let mut rotation = Vec3::new(0.0, 0.0, 0.0);
    let mut scale = 200.0f32;
    let mut current_shader = 0u32;

    // Parámetros para la Luna (relativos al tamaño de la Tierra)
    let moon_orbit_radius_base = 1.8; // Radio orbital relativo al tamaño de la Tierra
    let moon_scale_ratio = 0.27; // La luna es ~27% del tamaño de la Tierra
    let mut moon_orbit_speed = 0.5;

    // Generar esfera proceduralmente
    let (vertices, indices) = create_sphere(1.0, 30);
    println!("Esfera generada: {} vértices, {} triángulos", vertices.len(), indices.len() / 3);

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        handle_input(&window, &mut translation, &mut rotation, &mut scale, &mut current_shader, 
                    &mut moon_orbit_speed);

        // Rotación automática de la Tierra
        rotation.y += 0.01;

        let elapsed = start_time.elapsed().as_secs_f32();

        // Ajustar FPS dinámicamente
        let frame_delay = if scale > 300.0 { 
            Duration::from_millis(33)
        } else { 
            Duration::from_millis(16)
        };

        framebuffer.clear();

        // === RENDERIZAR TIERRA ===
        let model_matrix = create_model_matrix(translation, scale, rotation);
        let uniforms = Uniforms { 
            model_matrix,
            time: elapsed,
            current_shader,
            is_moon: false,
        };

        render_planet(&mut framebuffer, &uniforms, &vertices, &indices);

        
        if current_shader == 0 {
            // Calcular tamaño y posición de la luna RELATIVOS a la Tierra
            let moon_scale = scale * moon_scale_ratio;
            let moon_orbit_radius = scale * moon_orbit_radius_base;
            
            // Distancia mínima para evitar que la luna toque la Tierra
            let min_distance = scale * 1.3; // 30% más que el radio de la Tierra
            
            // Usar la distancia mayor entre la calculada y la mínima
            let final_orbit_radius = moon_orbit_radius.max(min_distance);
            
            // Calcular posición orbital de la luna
            let moon_angle = elapsed * moon_orbit_speed;
            let moon_x = translation.x + final_orbit_radius * moon_angle.cos();
            let moon_y = translation.y + final_orbit_radius * moon_angle.sin() * 0.7;
            let moon_translation = Vec3::new(moon_x, moon_y, 0.0);
            
            // La luna siempre mira hacia la Tierra (rotación sincrónica)
            let moon_rotation = Vec3::new(0.0, elapsed * 0.5, 0.0);
            
            let moon_model_matrix = create_model_matrix(moon_translation, moon_scale, moon_rotation);
            let moon_uniforms = Uniforms {
                model_matrix: moon_model_matrix,
                time: elapsed,
                current_shader,
                is_moon: true,
            };

            render_planet(&mut framebuffer, &moon_uniforms, &vertices, &indices);
        }

        window
            .update_with_buffer(&framebuffer.buffer, window_width, window_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}



fn handle_input(window: &Window, translation: &mut Vec3, rotation: &mut Vec3, scale: &mut f32, 
                current_shader: &mut u32, moon_orbit_speed: &mut f32) {
    let move_speed = 5.0;
    let rotation_speed = PI / 30.0;
    let scale_speed = 5.0;

    // Movimiento
    if window.is_key_down(Key::Right) {
        translation.x += move_speed;
    }
    if window.is_key_down(Key::Left) {
        translation.x -= move_speed;
    }
    if window.is_key_down(Key::Up) {
        translation.y -= move_speed;
    }
    if window.is_key_down(Key::Down) {
        translation.y += move_speed;
    }

    
    if window.is_key_down(Key::S) {
        *scale += scale_speed;
    }
    if window.is_key_down(Key::A) {
        *scale -= scale_speed;
        if *scale < 10.0 {
            *scale = 10.0;
        }
    }

    // Rotación manual
    if window.is_key_down(Key::Q) {
        rotation.x -= rotation_speed;
    }
    if window.is_key_down(Key::W) {
        rotation.x += rotation_speed;
    }
    if window.is_key_down(Key::E) {
        rotation.z -= rotation_speed;
    }
    if window.is_key_down(Key::R) {
        rotation.z += rotation_speed;
    }

    
    if *current_shader == 0 {
        if window.is_key_down(Key::Y) {
            *moon_orbit_speed += 0.05;
            println!("Velocidad orbital de la luna: {:.2}", moon_orbit_speed);
        }
        if window.is_key_down(Key::I) {
            *moon_orbit_speed -= 0.05;
            if *moon_orbit_speed < 0.1 {
                *moon_orbit_speed = 0.1;
            }
            println!("Velocidad orbital de la luna: {:.2}", moon_orbit_speed);
        }
    }

    // Cambiar shaders
    if window.is_key_down(Key::Key1) {
        *current_shader = 0;
        println!("Shader: TIERRA (con Luna)");
        
    }
    if window.is_key_down(Key::Key2) {
        *current_shader = 1;
        println!("Shader: SOL");
    }
    if window.is_key_down(Key::Key3) {
        *current_shader = 2;
        println!("Shader: GIGANTE GASEOSO");
    }
    if window.is_key_down(Key::Key4) {
        *current_shader = 3;
        println!("Shader: MARTE");
    }
    if window.is_key_down(Key::Key5) {
        *current_shader = 4;
        println!("Shader: MERCURIO");
    }

    if window.is_key_down(Key::Key6) {
    *current_shader = 5; // Neptuno
    println!("Shader: NEPTUNO");
}
}