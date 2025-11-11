use nalgebra_glm::Vec3;

pub fn noise_3d(p: Vec3) -> f32 {
    let x = p.x;
    let y = p.y;
    let z = p.z;
    
    let hash = ((x * 127.1 + y * 311.7 + z * 74.7).sin() * 43758.5453).fract();
    (hash * 2.0 - 1.0).abs()
}

pub fn fractal_noise(mut p: Vec3, octaves: i32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 1.0;
    
    for _ in 0..octaves {
        value += noise_3d(p * frequency) * amplitude;
        frequency *= 2.0;
        amplitude *= 0.5;
        p = p * 2.0;
    }
    
    value.clamp(0.0, 1.0)
}

pub fn continent_noise(p: Vec3) -> f32 {
    // Más continentes grandes
    let large_scale = fractal_noise(p * 0.6, 2) * 0.7;
    let medium_scale = fractal_noise(p * 1.2, 3) * 0.3;
    (large_scale + medium_scale).clamp(0.0, 1.0)
}

pub fn cloud_noise(p: Vec3, time: f32) -> f32 {
    // Nubes más realistas
    let moving_pos = p + Vec3::new(time * 0.02, 0.0, time * 0.01);
    let cloud_base = fractal_noise(moving_pos * 1.5, 4);
    let cloud_detail = fractal_noise(moving_pos * 4.0, 2) * 0.3;
    (cloud_base + cloud_detail).clamp(0.0, 1.0)
}

pub fn sun_noise(p: Vec3, time: f32) -> f32 {
    let animated_pos = p + Vec3::new(time * 0.5, time * 0.3, time * 0.2);
    fractal_noise(animated_pos * 4.0, 5)
}

pub fn gas_bands(p: Vec3, time: f32) -> f32 {
    let bands = (p.y * 8.0 + time * 0.2).sin() * 0.5 + 0.5;
    let turbulence = fractal_noise(p * 2.0 + Vec3::new(time * 0.1, 0.0, 0.0), 3);
    (bands + turbulence * 0.3).clamp(0.0, 1.0)
}