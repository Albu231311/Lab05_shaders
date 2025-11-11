use nalgebra_glm::{Vec3, dot};
use crate::color::Color;
use crate::noise::{fractal_noise, cloud_noise, sun_noise, gas_bands};


pub fn earth_shader(world_pos: Vec3, normal: Vec3, light_dir: Vec3, time: f32) -> Color {
    let sphere_pos = world_pos.normalize();

    // === CAPA 1: BASE AZUL OSCURA (MAR) ===
    let base_color = Color::new(0, 50, 150); // Azul océano intenso

    // === CAPA 2: CASQUETES POLARES ===
    let latitude = sphere_pos.y.abs();
    let ice_factor = if latitude > 0.8 {
        ((latitude - 0.8) / 0.2).clamp(0.0, 1.0) * 0.9
    } else {
        0.0
    };

    let ice_color = Color::new(255, 255, 255);
    let terrain_with_ice = if ice_factor > 0.1 {
        let r = (base_color.r() as f32 * (1.0 - ice_factor) + ice_color.r() as f32 * ice_factor).clamp(0.0, 255.0);
        let g = (base_color.g() as f32 * (1.0 - ice_factor) + ice_color.g() as f32 * ice_factor).clamp(0.0, 255.0);
        let b = (base_color.b() as f32 * (1.0 - ice_factor) + ice_color.b() as f32 * ice_factor).clamp(0.0, 255.0);
        Color::new(r as u8, g as u8, b as u8)
    } else {
        base_color
    };

    // === CAPA 3: BLOQUES DE CONTINENTES CON SENOS/COSENOS ===
    
    // Máscara para evitar continentes en los polos
    let pole_mask = if latitude < 0.7 { 1.0 } else { 0.0 };
    
    // Patrón de bloques usando senos y cosenos
    let x = sphere_pos.x;
    let y = sphere_pos.y;
    let z = sphere_pos.z;
    
    // ===== CONTINENTE 1 - CAFÉ (ÁFRICA) =====
    let continent_pattern1 = (x * 2.0).sin() * (z * 1.5).cos();
    let continent_pattern2 = (y * 1.0 + z * 2.5).cos() * (x * 1.8).sin();
    let continent_pattern3 = (z * 2.2).sin() * (y * 1.5 + x * 1.3).cos();
    let coast_detail1 = (x * 8.0).sin() * (z * 10.0).cos() * 0.2;
    let coast_detail2 = (x * 12.0 + z * 8.0).sin() * (y * 15.0).cos() * 0.15;
    let fine_detail1 = ((x * 25.0).sin() * (z * 30.0).cos()) * 0.08;
    let base_continent1 = continent_pattern1 * 0.5 + continent_pattern2 * 0.35 + continent_pattern3 * 0.15;
    let continent_density1 = base_continent1 + coast_detail1 + coast_detail2 + fine_detail1;
    
    // ===== CONTINENTE 2 - VERDE (SUDAMÉRICA) =====
    let continent_pattern4 = ((x - 1.5) * 2.1).sin() * ((z + 0.8) * 1.6).cos();
    let continent_pattern5 = ((y * 0.9 + (z + 0.8) * 2.3).cos()) * ((x - 1.5) * 1.9).sin();
    let continent_pattern6 = ((z + 0.8) * 2.4).sin() * ((y * 1.4 + (x - 1.5) * 1.6).cos());
    let coast_detail4 = ((x - 1.5) * 9.0).sin() * ((z + 0.8) * 11.0).cos() * 0.18;
    let coast_detail5 = ((x - 1.5) * 13.0).sin() * (y * 16.0).cos() * 0.14;
    let fine_detail2 = (((x - 1.5) * 26.0).sin() * ((z + 0.8) * 31.0).cos()) * 0.07;
    let base_continent2 = continent_pattern4 * 0.5 + continent_pattern5 * 0.35 + continent_pattern6 * 0.15;
    let continent_density2 = base_continent2 + coast_detail4 + coast_detail5 + fine_detail2;
    
    // ===== CONTINENTE 3 - VERDE (AMÉRICA DEL NORTE) =====
    let continent_pattern7 = ((x + 0.8) * 1.9).sin() * ((z - 1.2) * 1.7).cos();
    let continent_pattern8 = ((y * 1.1 + (z - 1.2) * 2.1).cos()) * ((x + 0.8) * 2.0).sin();
    let continent_pattern9 = ((z - 1.2) * 2.3).sin() * ((y * 1.6 + (x + 0.8) * 1.5).cos());
    let coast_detail7 = ((x + 0.8) * 8.5).sin() * ((z - 1.2) * 10.5).cos() * 0.19;
    let coast_detail8 = ((x + 0.8) * 12.5).sin() * (y * 15.5).cos() * 0.16;
    let fine_detail3 = (((x + 0.8) * 24.0).sin() * ((z - 1.2) * 29.0).cos()) * 0.09;
    let base_continent3 = continent_pattern7 * 0.5 + continent_pattern8 * 0.35 + continent_pattern9 * 0.15;
    let continent_density3 = base_continent3 + coast_detail7 + coast_detail8 + fine_detail3;
    
    // ===== CONTINENTE 4 - CAFÉ (AUSTRALIA) =====
    let continent_pattern10 = ((x + 1.2) * 2.2).sin() * ((z + 1.5) * 1.8).cos();
    let continent_pattern11 = ((y * 0.8 + (z + 1.5) * 2.4).cos()) * ((x + 1.2) * 1.85).sin();
    let continent_pattern12 = ((z + 1.5) * 2.6).sin() * ((y * 1.3 + (x + 1.2) * 1.7).cos());
    let coast_detail10 = ((x + 1.2) * 9.5).sin() * ((z + 1.5) * 11.5).cos() * 0.17;
    let coast_detail11 = ((x + 1.2) * 13.5).sin() * (y * 14.5).cos() * 0.14;
    let fine_detail4 = (((x + 1.2) * 27.0).sin() * ((z + 1.5) * 32.0).cos()) * 0.08;
    let base_continent4 = continent_pattern10 * 0.5 + continent_pattern11 * 0.35 + continent_pattern12 * 0.15;
    let continent_density4 = base_continent4 + coast_detail10 + coast_detail11 + fine_detail4;
    
    // ===== CONTINENTE 5 - VERDE (EURASIA) =====
    let continent_pattern13 = ((x - 0.5) * 1.95).sin() * ((z - 0.6) * 1.65).cos();
    let continent_pattern14 = ((y * 1.05 + (z - 0.6) * 2.15).cos()) * ((x - 0.5) * 2.05).sin();
    let continent_pattern15 = ((z - 0.6) * 2.35).sin() * ((y * 1.55 + (x - 0.5) * 1.45).cos());
    let coast_detail13 = ((x - 0.5) * 8.8).sin() * ((z - 0.6) * 10.8).cos() * 0.185;
    let coast_detail14 = ((x - 0.5) * 12.8).sin() * (y * 15.8).cos() * 0.155;
    let fine_detail5 = (((x - 0.5) * 25.5).sin() * ((z - 0.6) * 30.5).cos()) * 0.075;
    let base_continent5 = continent_pattern13 * 0.5 + continent_pattern14 * 0.35 + continent_pattern15 * 0.15;
    let continent_density5 = base_continent5 + coast_detail13 + coast_detail14 + fine_detail5;
    
    // ===== CONTINENTE 6 - CAFÉ (ZONA VACÍA) =====
    let continent_pattern16 = ((x + 0.3) * 2.15).sin() * ((z + 0.5) * 1.75).cos();
    let continent_pattern17 = ((y * 0.95 + (z + 0.5) * 2.35).cos()) * ((x + 0.3) * 1.88).sin();
    let continent_pattern18 = ((z + 0.5) * 2.55).sin() * ((y * 1.35 + (x + 0.3) * 1.65).cos());
    let coast_detail16 = ((x + 0.3) * 9.2).sin() * ((z + 0.5) * 11.2).cos() * 0.175;
    let coast_detail17 = ((x + 0.3) * 13.2).sin() * (y * 14.8).cos() * 0.145;
    let fine_detail6 = (((x + 0.3) * 26.5).sin() * ((z + 0.5) * 31.5).cos()) * 0.08;
    let base_continent6 = continent_pattern16 * 0.5 + continent_pattern17 * 0.35 + continent_pattern18 * 0.15;
    let continent_density6 = base_continent6 + coast_detail16 + coast_detail17 + fine_detail6;
    
    // Combinar continentes verdes
    let continent_density_green = continent_density2.max(continent_density3).max(continent_density5);
    
    // Combinar continentes cafés (ahora incluye el nuevo continente 6)
    let continent_density_brown = continent_density1.max(continent_density4).max(continent_density6);
    
    // Calcular factor para continentes VERDES
    let continent_factor_green = if continent_density_green > 0.10 && pole_mask > 0.5 {
        let factor = ((continent_density_green - 0.10) / 0.90).clamp(0.0, 1.0);
        if factor > 0.2 {
            factor * 0.95
        } else {
            factor * 0.65
        }
    } else {
        0.0
    };
    
    // Calcular factor para continentes CAFÉS
    let continent_factor_brown = if continent_density_brown > 0.10 && pole_mask > 0.5 {
        let factor = ((continent_density_brown - 0.10) / 0.90).clamp(0.0, 1.0);
        if factor > 0.2 {
            factor * 0.95
        } else {
            factor * 0.65
        }
    } else {
        0.0
    };

    // Colores más saturados y vivos
    let continent_color_green = Color::new(20, 160, 40);   // Verde más saturado
    let continent_color_brown = Color::new(220, 150, 60);  // Café más brillante
    
    // Aplicar continentes verdes primero
    let terrain_with_green = if continent_factor_green > 0.15 {
        let r = (terrain_with_ice.r() as f32 * (1.0 - continent_factor_green) + continent_color_green.r() as f32 * continent_factor_green).clamp(0.0, 255.0);
        let g = (terrain_with_ice.g() as f32 * (1.0 - continent_factor_green) + continent_color_green.g() as f32 * continent_factor_green).clamp(0.0, 255.0);
        let b = (terrain_with_ice.b() as f32 * (1.0 - continent_factor_green) + continent_color_green.b() as f32 * continent_factor_green).clamp(0.0, 255.0);
        Color::new(r as u8, g as u8, b as u8)
    } else {
        terrain_with_ice
    };
    
    // Aplicar continentes cafés sobre el resultado anterior
    let terrain_color = if continent_factor_brown > 0.15 {
        let r = (terrain_with_green.r() as f32 * (1.0 - continent_factor_brown) + continent_color_brown.r() as f32 * continent_factor_brown).clamp(0.0, 255.0);
        let g = (terrain_with_green.g() as f32 * (1.0 - continent_factor_brown) + continent_color_brown.g() as f32 * continent_factor_brown).clamp(0.0, 255.0);
        let b = (terrain_with_green.b() as f32 * (1.0 - continent_factor_brown) + continent_color_brown.b() as f32 * continent_factor_brown).clamp(0.0, 255.0);
        Color::new(r as u8, g as u8, b as u8)
    } else {
        terrain_with_green
    };

    // === CAPA 4: NUBES ===
    let cloud_density = cloud_noise(sphere_pos, time);
    
    let cloud_factor = if cloud_density > 0.6 {
        ((cloud_density - 0.6) / 0.4).clamp(0.0, 1.0) * 0.5
    } else {
        0.0
    };

    let cloud_color = Color::new(255, 255, 255);
    let final_color = if cloud_factor > 0.15 {
        let blend = cloud_factor;
        let r = (terrain_color.r() as f32 * (1.0 - blend) + cloud_color.r() as f32 * blend).clamp(0.0, 255.0);
        let g = (terrain_color.g() as f32 * (1.0 - blend) + cloud_color.g() as f32 * blend).clamp(0.0, 255.0);
        let b = (terrain_color.b() as f32 * (1.0 - blend) + cloud_color.b() as f32 * blend).clamp(0.0, 255.0);
        Color::new(r as u8, g as u8, b as u8)
    } else {
        terrain_color
    };

    // === ILUMINACIÓN ===
    let light_dir_normalized = light_dir.normalize();
    let diffuse = dot(&normal, &light_dir_normalized).max(0.0) * 0.7;
    let ambient = 0.5; // Más luz ambiente para que el océano se vea azul
    let intensity = (diffuse + ambient).min(1.0);

    let r = (final_color.r() as f32 * intensity).clamp(0.0, 255.0);
    let g = (final_color.g() as f32 * intensity).clamp(0.0, 255.0);
    let b = (final_color.b() as f32 * intensity).clamp(0.0, 255.0);

    Color::new(r as u8, g as u8, b as u8)
}

pub fn sun_shader(world_pos: Vec3, normal: Vec3, _light_dir: Vec3, time: f32) -> Color {
    // === CAPA 1: SUPERFICIE ARDIENTE ===
    let surface_turbulence = sun_noise(world_pos, time);
    
    let core_color = Color::new(255, 255, 100);  // Amarillo brillante
    let hot_color = Color::new(255, 150, 50);    // Naranja caliente
    let flare_color = Color::new(255, 50, 0);    // Rojo llamarada
    
    let base_color = if surface_turbulence < 0.3 {
        core_color
    } else if surface_turbulence < 0.7 {
        let t = (surface_turbulence - 0.3) / 0.4;
        Color::new(
            (core_color.r() as f32 * (1.0 - t) + hot_color.r() as f32 * t) as u8,
            (core_color.g() as f32 * (1.0 - t) + hot_color.g() as f32 * t) as u8,
            (core_color.b() as f32 * (1.0 - t) + hot_color.b() as f32 * t) as u8,
        )
    } else {
        flare_color
    };
    
    // === CAPA 2: MANCHAS SOLARES ===
    let spot_noise = fractal_noise(world_pos * 1.5, 3);
    let spot_factor = if spot_noise > 0.8 {
        ((spot_noise - 0.8) / 0.2) * 0.5
    } else {
        0.0
    };
    
    let spot_color = Color::new(150, 80, 20);
    let surface_with_spots = if spot_factor > 0.0 {
        Color::new(
            (base_color.r() as f32 * (1.0 - spot_factor) + spot_color.r() as f32 * spot_factor) as u8,
            (base_color.g() as f32 * (1.0 - spot_factor) + spot_color.g() as f32 * spot_factor) as u8,
            (base_color.b() as f32 * (1.0 - spot_factor) + spot_color.b() as f32 * spot_factor) as u8,
        )
    } else {
        base_color
    };
    
    // === CAPA 3: CORONA (borde brillante) ===
    let view_dir = Vec3::new(0.0, 0.0, 1.0);
    let fresnel = 1.0 - dot(&normal, &view_dir).abs();
    let corona_factor = fresnel.powf(2.0) * 0.6;
    
    let corona_color = Color::new(255, 200, 100);
    let surface_with_corona = if corona_factor > 0.0 {
        Color::new(
            (surface_with_spots.r() as f32 + corona_color.r() as f32 * corona_factor).min(255.0) as u8,
            (surface_with_spots.g() as f32 + corona_color.g() as f32 * corona_factor).min(255.0) as u8,
            (surface_with_spots.b() as f32 + corona_color.b() as f32 * corona_factor).min(255.0) as u8,
        )
    } else {
        surface_with_spots
    };
    
    // === CAPA 4: PULSACIÓN ===
    let pulse = (time * 3.0).sin() * 0.1 + 0.9;
    
    Color::new(
        (surface_with_corona.r() as f32 * pulse) as u8,
        (surface_with_corona.g() as f32 * pulse) as u8,
        (surface_with_corona.b() as f32 * pulse) as u8,
    )
}

pub fn gas_giant_shader(world_pos: Vec3, normal: Vec3, light_dir: Vec3, time: f32) -> Color {
    let band_pattern = gas_bands(world_pos, time);
    
    let band1_color = Color::new(200, 150, 100);
    let band2_color = Color::new(150, 100, 80);
    let band3_color = Color::new(100, 80, 120);
    
    let base_color = if band_pattern < 0.33 {
        band1_color
    } else if band_pattern < 0.66 {
        band2_color
    } else {
        band3_color
    };
    
    let turbulence = fractal_noise(world_pos * 2.0 + Vec3::new(time * 0.1, 0.0, 0.0), 4);
    let storm_factor = if turbulence > 0.7 {
        ((turbulence - 0.7) / 0.3) * 0.4
    } else {
        0.0
    };
    
    let storm_color = Color::new(220, 200, 180);
    let surface_with_storms = if storm_factor > 0.0 {
        Color::new(
            (base_color.r() as f32 * (1.0 - storm_factor) + storm_color.r() as f32 * storm_factor) as u8,
            (base_color.g() as f32 * (1.0 - storm_factor) + storm_color.g() as f32 * storm_factor) as u8,
            (base_color.b() as f32 * (1.0 - storm_factor) + storm_color.b() as f32 * storm_factor) as u8,
        )
    } else {
        base_color
    };
    
    let spot_center = Vec3::new(0.3, 0.0, 0.0);
    let distance_to_spot = (world_pos - spot_center).magnitude();
    let spot_factor = if distance_to_spot < 0.3 {
        (1.0 - distance_to_spot / 0.3) * 0.7
    } else {
        0.0
    };
    
    let red_spot_color = Color::new(180, 80, 60);
    let surface_with_spot = if spot_factor > 0.0 {
        Color::new(
            (surface_with_storms.r() as f32 * (1.0 - spot_factor) + red_spot_color.r() as f32 * spot_factor) as u8,
            (surface_with_storms.g() as f32 * (1.0 - spot_factor) + red_spot_color.g() as f32 * spot_factor) as u8,
            (surface_with_storms.b() as f32 * (1.0 - spot_factor) + red_spot_color.b() as f32 * spot_factor) as u8,
        )
    } else {
        surface_with_storms
    };
    
    let view_dir = Vec3::new(0.0, 0.0, 1.0);
    let fresnel = 1.0 - dot(&normal, &view_dir).abs();
    let atmosphere_factor = fresnel.powf(4.0) * 0.3;
    
    let atmosphere_color = Color::new(200, 180, 160);
    let final_color = if atmosphere_factor > 0.0 {
        Color::new(
            (surface_with_spot.r() as f32 * (1.0 - atmosphere_factor) + atmosphere_color.r() as f32 * atmosphere_factor) as u8,
            (surface_with_spot.g() as f32 * (1.0 - atmosphere_factor) + atmosphere_color.g() as f32 * atmosphere_factor) as u8,
            (surface_with_spot.b() as f32 * (1.0 - atmosphere_factor) + atmosphere_color.b() as f32 * atmosphere_factor) as u8,
        )
    } else {
        surface_with_spot
    };
    
    let diffuse = dot(&normal, &light_dir).max(0.0) * 0.8;
    let ambient = 0.3;
    let intensity = (diffuse + ambient).min(1.0);
    
    Color::new(
        (final_color.r() as f32 * intensity) as u8,
        (final_color.g() as f32 * intensity) as u8,
        (final_color.b() as f32 * intensity) as u8,
    )
}

pub fn mars_shader(world_pos: Vec3, normal: Vec3, light_dir: Vec3, time: f32) -> Color {
    let sphere_pos = world_pos.normalize();
    
    // === CAPA 1: BASE MARCIANA (ÓXIDO DE HIERRO) ===
    // Color base rojizo-anaranjado característico de Marte
    let base_red = Color::new(193, 68, 14);      // Rojo óxido principal
    let orange_tint = Color::new(220, 100, 40);  // Tonos anaranjados
    
    // Variación sutil de color base usando noise
    let color_variation = fractal_noise(sphere_pos * 1.5, 3);
    let base_color = if color_variation > 0.5 {
        let t = ((color_variation - 0.5) * 2.0).min(0.4); // Limitamos la variación
        Color::new(
            (base_red.r() as f32 * (1.0 - t) + orange_tint.r() as f32 * t) as u8,
            (base_red.g() as f32 * (1.0 - t) + orange_tint.g() as f32 * t) as u8,
            (base_red.b() as f32 * (1.0 - t) + orange_tint.b() as f32 * t) as u8,
        )
    } else {
        base_red
    };
    
    // === CAPA 2: CASQUETES POLARES ===
    let latitude = sphere_pos.y.abs();
    let ice_factor = if latitude > 0.75 {
        ((latitude - 0.75) / 0.25).clamp(0.0, 1.0) * 0.85
    } else {
        0.0
    };
    
    // Hielo de CO2 y agua con tono ligeramente rosado
    let ice_color = Color::new(245, 240, 235);
    let terrain_with_ice = if ice_factor > 0.1 {
        Color::new(
            (base_color.r() as f32 * (1.0 - ice_factor) + ice_color.r() as f32 * ice_factor).clamp(0.0, 255.0) as u8,
            (base_color.g() as f32 * (1.0 - ice_factor) + ice_color.g() as f32 * ice_factor).clamp(0.0, 255.0) as u8,
            (base_color.b() as f32 * (1.0 - ice_factor) + ice_color.b() as f32 * ice_factor).clamp(0.0, 255.0) as u8,
        )
    } else {
        base_color
    };
    
    // === CAPA 3: CRÁTERES DE IMPACTO (más sutiles) ===
    let crater_noise1 = fractal_noise(sphere_pos * 8.0, 3);
    let crater_noise2 = fractal_noise(sphere_pos * 12.0 + Vec3::new(100.0, 100.0, 100.0), 2);
    
    let crater_factor = if crater_noise1 > 0.8 && crater_noise2 > 0.75 {
        ((crater_noise1 - 0.8) / 0.2 * (crater_noise2 - 0.75) / 0.25) * 0.35
    } else {
        0.0
    };
    
    // Color más oscuro para cráteres (basalto expuesto)
    let crater_color = Color::new(120, 50, 30);
    let terrain_with_craters = if crater_factor > 0.05 {
        Color::new(
            (terrain_with_ice.r() as f32 * (1.0 - crater_factor) + crater_color.r() as f32 * crater_factor) as u8,
            (terrain_with_ice.g() as f32 * (1.0 - crater_factor) + crater_color.g() as f32 * crater_factor) as u8,
            (terrain_with_ice.b() as f32 * (1.0 - crater_factor) + crater_color.b() as f32 * crater_factor) as u8,
        )
    } else {
        terrain_with_ice
    };
    
    // === CAPA 4: VALLES Y CAÑONES (VALLES MARINERIS) ===
    // Gran sistema de cañones usando noise para hacerlo más natural
    let canyon_base = fractal_noise(sphere_pos * 3.0, 4);
    let canyon_mask = if latitude < 0.4 { // Solo cerca del ecuador
        1.0
    } else {
        0.0
    };
    
    let canyon_factor = if canyon_base > 0.65 && canyon_mask > 0.5 {
        ((canyon_base - 0.65) / 0.35) * 0.4
    } else {
        0.0
    };
    
    let canyon_color = Color::new(140, 55, 25);
    let terrain_with_canyons = if canyon_factor > 0.1 {
        Color::new(
            (terrain_with_craters.r() as f32 * (1.0 - canyon_factor) + canyon_color.r() as f32 * canyon_factor) as u8,
            (terrain_with_craters.g() as f32 * (1.0 - canyon_factor) + canyon_color.g() as f32 * canyon_factor) as u8,
            (terrain_with_craters.b() as f32 * (1.0 - canyon_factor) + canyon_color.b() as f32 * canyon_factor) as u8,
        )
    } else {
        terrain_with_craters
    };
    
    // === CAPA 5: REGIONES VOLCÁNICAS OSCURAS ===
    // Patrones de basalto volcánico usando noise
    let volcanic_noise = fractal_noise(sphere_pos * 2.5, 4);
    let volcanic_detail = fractal_noise(sphere_pos * 5.0 + Vec3::new(50.0, 50.0, 50.0), 2);
    
    let volcanic_factor = if volcanic_noise > 0.6 && volcanic_detail > 0.55 && ice_factor < 0.1 {
        ((volcanic_noise - 0.6) / 0.4) * ((volcanic_detail - 0.55) / 0.45) * 0.5
    } else {
        0.0
    };
    
    // Regiones oscuras de basalto
    let volcanic_color = Color::new(100, 45, 25);
    let terrain_with_volcanic = if volcanic_factor > 0.15 {
        Color::new(
            (terrain_with_canyons.r() as f32 * (1.0 - volcanic_factor) + volcanic_color.r() as f32 * volcanic_factor) as u8,
            (terrain_with_canyons.g() as f32 * (1.0 - volcanic_factor) + volcanic_color.g() as f32 * volcanic_factor) as u8,
            (terrain_with_canyons.b() as f32 * (1.0 - volcanic_factor) + volcanic_color.b() as f32 * volcanic_factor) as u8,
        )
    } else {
        terrain_with_canyons
    };
    
    // === CAPA 6: DUNAS Y CARACTERÍSTICAS DE ARENA ===
    let dune_noise = fractal_noise(sphere_pos * 15.0, 3);
    let dune_factor = if dune_noise > 0.7 {
        ((dune_noise - 0.7) / 0.3) * 0.2
    } else {
        0.0
    };
    
    let dune_color = Color::new(210, 95, 35);
    let terrain_with_dunes = if dune_factor > 0.05 {
        Color::new(
            (terrain_with_volcanic.r() as f32 * (1.0 - dune_factor) + dune_color.r() as f32 * dune_factor) as u8,
            (terrain_with_volcanic.g() as f32 * (1.0 - dune_factor) + dune_color.g() as f32 * dune_factor) as u8,
            (terrain_with_volcanic.b() as f32 * (1.0 - dune_factor) + dune_color.b() as f32 * dune_factor) as u8,
        )
    } else {
        terrain_with_volcanic
    };
    
    // === CAPA 7: TORMENTAS DE POLVO (SUTILES) ===
    let dust_storm = fractal_noise(sphere_pos * 1.5 + Vec3::new(time * 0.03, 0.0, time * 0.02), 4);
    let dust_factor = if dust_storm > 0.75 {
        ((dust_storm - 0.75) / 0.25) * 0.2
    } else {
        0.0
    };
    
    let dust_color = Color::new(210, 120, 70);
    let final_color = if dust_factor > 0.08 {
        Color::new(
            (terrain_with_dunes.r() as f32 * (1.0 - dust_factor) + dust_color.r() as f32 * dust_factor) as u8,
            (terrain_with_dunes.g() as f32 * (1.0 - dust_factor) + dust_color.g() as f32 * dust_factor) as u8,
            (terrain_with_dunes.b() as f32 * (1.0 - dust_factor) + dust_color.b() as f32 * dust_factor) as u8,
        )
    } else {
        terrain_with_dunes
    };
    
    // === ILUMINACIÓN ===
    let light_dir_normalized = light_dir.normalize();
    let diffuse = dot(&normal, &light_dir_normalized).max(0.0) * 0.75;
    let ambient = 0.35; // Marte tiene atmósfera tenue, menos luz ambiente
    let intensity = (diffuse + ambient).min(1.0);
    
    Color::new(
        (final_color.r() as f32 * intensity).clamp(0.0, 255.0) as u8,
        (final_color.g() as f32 * intensity).clamp(0.0, 255.0) as u8,
        (final_color.b() as f32 * intensity).clamp(0.0, 255.0) as u8,
    )
}


pub fn mercury_shader(world_pos: Vec3, normal: Vec3, light_dir: Vec3, time: f32) -> Color {
    let sphere_pos = world_pos.normalize();

    // === Ruido base para grandes manchas ===
    let base_noise = fractal_noise(sphere_pos * 1.5, 5); // frecuencia más baja → zonas grandes
    let detail_noise = fractal_noise(sphere_pos * 10.0 + Vec3::new(time * 0.05, 0.0, 0.0), 3);
    let combined_noise = (base_noise * 0.7 + detail_noise * 0.3).clamp(0.0, 1.0);

    // === Paleta de colores estilo Mercurio ===
    let dark_brown = Color::new(90, 50, 20);
    let orange_brown = Color::new(180, 90, 40);
    let bright_orange = Color::new(230, 140, 50);
    let yellow_tone = Color::new(255, 190, 80);

    // Mezcla progresiva entre tonos
    let base_color = if combined_noise < 0.3 {
        dark_brown
    } else if combined_noise < 0.55 {
        Color::new(
            (dark_brown.r() as f32 * (1.0 - combined_noise) + orange_brown.r() as f32 * combined_noise) as u8,
            (dark_brown.g() as f32 * (1.0 - combined_noise) + orange_brown.g() as f32 * combined_noise) as u8,
            (dark_brown.b() as f32 * (1.0 - combined_noise) + orange_brown.b() as f32 * combined_noise) as u8,
        )
    } else if combined_noise < 0.8 {
        Color::new(
            (orange_brown.r() as f32 * (1.0 - combined_noise) + bright_orange.r() as f32 * combined_noise) as u8,
            (orange_brown.g() as f32 * (1.0 - combined_noise) + bright_orange.g() as f32 * combined_noise) as u8,
            (orange_brown.b() as f32 * (1.0 - combined_noise) + bright_orange.b() as f32 * combined_noise) as u8,
        )
    } else {
        Color::new(
            (bright_orange.r() as f32 * (1.0 - combined_noise) + yellow_tone.r() as f32 * combined_noise) as u8,
            (bright_orange.g() as f32 * (1.0 - combined_noise) + yellow_tone.g() as f32 * combined_noise) as u8,
            (bright_orange.b() as f32 * (1.0 - combined_noise) + yellow_tone.b() as f32 * combined_noise) as u8,
        )
    };

    // === Simulación de cráteres ===
    let crater_noise = fractal_noise(sphere_pos * 20.0, 4);
    let crater_factor = if crater_noise > 0.75 { (crater_noise - 0.75) * 3.0 } else { 0.0 };
    let crater_color = Color::new(50, 30, 15);
    let with_craters = Color::new(
        (base_color.r() as f32 * (1.0 - crater_factor) + crater_color.r() as f32 * crater_factor) as u8,
        (base_color.g() as f32 * (1.0 - crater_factor) + crater_color.g() as f32 * crater_factor) as u8,
        (base_color.b() as f32 * (1.0 - crater_factor) + crater_color.b() as f32 * crater_factor) as u8,
    );

    // === Iluminación difusa + ambiente ===
    let diffuse = dot(&normal, &light_dir).max(0.0) * 0.85;
    let ambient = 0.25;
    let intensity = (diffuse + ambient).min(1.0);

    Color::new(
        (with_craters.r() as f32 * intensity) as u8,
        (with_craters.g() as f32 * intensity) as u8,
        (with_craters.b() as f32 * intensity) as u8,
    )
}

// Agrega esta función al final del archivo planet_shaders.rs

pub fn moon_shader(world_pos: Vec3, normal: Vec3, light_dir: Vec3, time: f32) -> Color {
    let sphere_pos = world_pos.normalize();

    // === COLOR BLANCO PURO DE LA LUNA ===
    let pure_white = Color::new(255, 255, 255);
    let bright_white = Color::new(255, 255, 255);
    let off_white = Color::new(250, 250, 245);

    // === MANCHAS NEGRAS CARACTERÍSTICAS (MARES LUNARES) ===
    // Mares lunares principales - patrones grandes y reconocibles
    let mare_noise1 = fractal_noise(sphere_pos * 2.0, 4);
    let mare_noise2 = fractal_noise(sphere_pos * 4.0 + Vec3::new(50.0, 0.0, 50.0), 3);
    let mare_noise3 = fractal_noise(sphere_pos * 8.0 + Vec3::new(100.0, 100.0, 0.0), 2);
    
    // Mare Tranquillitatis (Mar de la Tranquilidad) - grande y oscuro
    let mare_tranq_center = Vec3::new(0.3, 0.1, 0.2);
    let distance_to_tranq = (sphere_pos - mare_tranq_center).magnitude();
    let mare_tranq_factor = if distance_to_tranq < 0.15 {
        (1.0 - distance_to_tranq / 0.15) * 0.9
    } else {
        0.0
    };

    // Mare Imbrium (Mar de las Lluvias) - grande y circular
    let mare_imbrium_center = Vec3::new(-0.2, 0.05, 0.25);
    let distance_to_imbrium = (sphere_pos - mare_imbrium_center).magnitude();
    let mare_imbrium_factor = if distance_to_imbrium < 0.12 {
        (1.0 - distance_to_imbrium / 0.12) * 0.85
    } else {
        0.0
    };

    // Mare Serenitatis (Mar de la Serenidad)
    let mare_seren_center = Vec3::new(0.15, -0.08, 0.3);
    let distance_to_seren = (sphere_pos - mare_seren_center).magnitude();
    let mare_seren_factor = if distance_to_seren < 0.1 {
        (1.0 - distance_to_seren / 0.1) * 0.8
    } else {
        0.0
    };

    // Oceanus Procellarum (Océano de las Tormentas) - muy extenso
    let ocean_proc_center = Vec3::new(-0.35, 0.0, 0.1);
    let distance_to_ocean = (sphere_pos - ocean_proc_center).magnitude();
    let ocean_proc_factor = if distance_to_ocean < 0.2 {
        (1.0 - distance_to_ocean / 0.2) * 0.7
    } else {
        0.0
    };

    // Manchas negras adicionales por noise
    let dark_spot1 = if mare_noise1 > 0.7 {
        ((mare_noise1 - 0.7) / 0.3) * 0.6
    } else {
        0.0
    };

    let dark_spot2 = if mare_noise2 > 0.75 {
        ((mare_noise2 - 0.75) / 0.25) * 0.5
    } else {
        0.0
    };

    let dark_spot3 = if mare_noise3 > 0.8 {
        ((mare_noise3 - 0.8) / 0.2) * 0.4
    } else {
        0.0
    };

    // Combinar todas las manchas negras
    let total_dark_factor = (mare_tranq_factor + mare_imbrium_factor + mare_seren_factor + 
                            ocean_proc_factor + dark_spot1 + dark_spot2 + dark_spot3).min(1.0);

    // === CRÁTERES BLANCOS SOBRE FONDO BLANCO (sutiles variaciones) ===
    let crater_noise1 = fractal_noise(sphere_pos * 10.0, 3);
    let crater_noise2 = fractal_noise(sphere_pos * 20.0 + Vec3::new(200.0, 0.0, 200.0), 2);
    
    let crater_factor = if crater_noise1 > 0.8 && crater_noise2 > 0.7 {
        ((crater_noise1 - 0.8) / 0.2 * (crater_noise2 - 0.7) / 0.3) * 0.3
    } else {
        0.0
    };

    // === CONSTRUCCIÓN DEL COLOR FINAL ===
    
    // Base blanca pura
    let mut base_color = pure_white;

    // Aplicar variaciones sutiles de blanco para los cráteres
    if crater_factor > 0.05 {
        let crater_color = Color::new(245, 245, 240); // Blanco ligeramente más oscuro
        base_color = Color::new(
            (base_color.r() as f32 * (1.0 - crater_factor) + crater_color.r() as f32 * crater_factor) as u8,
            (base_color.g() as f32 * (1.0 - crater_factor) + crater_color.g() as f32 * crater_factor) as u8,
            (base_color.b() as f32 * (1.0 - crater_factor) + crater_color.b() as f32 * crater_factor) as u8,
        );
    }

    // Aplicar manchas negras (mares lunares)
    if total_dark_factor > 0.1 {
        let mare_color = Color::new(80, 80, 80); // Gris muy oscuro, casi negro
        base_color = Color::new(
            (base_color.r() as f32 * (1.0 - total_dark_factor) + mare_color.r() as f32 * total_dark_factor) as u8,
            (base_color.g() as f32 * (1.0 - total_dark_factor) + mare_color.g() as f32 * total_dark_factor) as u8,
            (base_color.b() as f32 * (1.0 - total_dark_factor) + mare_color.b() as f32 * total_dark_factor) as u8,
        );
    }

    // === VARIACIÓN DE TONOS SUTILES ===
    let tone_variation = fractal_noise(sphere_pos * 1.5, 2);
    let final_base_color = if tone_variation > 0.6 {
        // Áreas ligeramente más cálidas
        let warm_white = Color::new(255, 253, 248);
        let t = (tone_variation - 0.6) / 0.4;
        Color::new(
            (base_color.r() as f32 * (1.0 - t) + warm_white.r() as f32 * t) as u8,
            (base_color.g() as f32 * (1.0 - t) + warm_white.g() as f32 * t) as u8,
            (base_color.b() as f32 * (1.0 - t) + warm_white.b() as f32 * t) as u8,
        )
    } else {
        base_color
    };

    // === ILUMINACIÓN ===
    let light_dir_normalized = light_dir.normalize();
    let diffuse = dot(&normal, &light_dir_normalized).max(0.0) * 1.1; // Alto albedo lunar
    let ambient = 0.6; // Buena reflectividad incluso en áreas oscuras
    let intensity = (diffuse + ambient).min(1.2);

    let illuminated_color = Color::new(
        (final_base_color.r() as f32 * intensity).min(255.0) as u8,
        (final_base_color.g() as f32 * intensity).min(255.0) as u8,
        (final_base_color.b() as f32 * intensity).min(255.0) as u8,
    );

    // === EFECTO DE BRILLO ESPECULAR EN CRÁTERES ===
    let view_dir = Vec3::new(0.0, 0.0, 1.0);
    let half_vector = (light_dir_normalized + view_dir).normalize();
    let specular = dot(&normal, &half_vector).max(0.0).powf(32.0) * 0.3;

    if specular > 0.1 {
        // Añadir brillo especular en áreas muy iluminadas
        Color::new(
            (illuminated_color.r() as f32 + 20.0 * specular).min(255.0) as u8,
            (illuminated_color.g() as f32 + 20.0 * specular).min(255.0) as u8,
            (illuminated_color.b() as f32 + 20.0 * specular).min(255.0) as u8,
        )
    } else {
        illuminated_color
    }
}

pub fn neptune_shader(world_pos: Vec3, normal: Vec3, light_dir: Vec3, time: f32) -> Color {
    let sphere_pos = world_pos.normalize();

    // === COLOR BASE AZUL PROFUNDO DE NEPTUNO ===
    let deep_blue = Color::new(30, 60, 150);    // Azul muy oscuro
    let bright_blue = Color::new(80, 120, 220); // Azul brillante para detalles
    let methane_blue = Color::new(40, 100, 200); // Tonos de metano

    // === PATRONES DE BANDAS ATMOSFÉRICAS ===
    let latitude = sphere_pos.y;
    let band_pattern = (latitude * 12.0 + time * 0.1).sin() * 0.5 + 0.5;
    
    // Bandas más pronunciadas cerca del ecuador
    let equator_mask = (1.0 - latitude.abs() * 2.0).max(0.0);
    let band_intensity = band_pattern * equator_mask * 0.7;

    // === TORMENTAS Y REMOLINOS CARACTERÍSTICOS ===
    let storm_noise1 = fractal_noise(sphere_pos * 3.0 + Vec3::new(time * 0.05, 0.0, 0.0), 4);
    let storm_noise2 = fractal_noise(sphere_pos * 8.0 + Vec3::new(0.0, time * 0.03, 0.0), 3);
    
    // Gran Mancha Oscura (simulada)
    let dark_spot_center = Vec3::new(0.4, 0.2, 0.0);
    let distance_to_dark_spot = (sphere_pos - dark_spot_center).magnitude();
    let dark_spot_factor = if distance_to_dark_spot < 0.25 {
        (1.0 - distance_to_dark_spot / 0.25) * 0.8
    } else {
        0.0
    };

    // Manchas blancas (nubes de metano)
    let white_spot_factor = if storm_noise1 > 0.75 {
        ((storm_noise1 - 0.75) / 0.25) * 0.6
    } else {
        0.0
    };

    // Remolinos atmosféricos
    let swirl_factor = if storm_noise2 > 0.7 {
        ((storm_noise2 - 0.7) / 0.3) * 0.4
    } else {
        0.0
    };

    // === TEXTURA DE NUBES DE METANO CRISTALIZADO ===
    let cloud_detail1 = fractal_noise(sphere_pos * 6.0, 3);
    let cloud_detail2 = fractal_noise(sphere_pos * 12.0 + Vec3::new(time * 0.02, 0.0, 0.0), 2);
    let methane_clouds = (cloud_detail1 * 0.7 + cloud_detail2 * 0.3) * 0.4;

    // === CONSTRUCCIÓN DEL COLOR FINAL ===
    
    // Base azul profunda
    let mut base_color = deep_blue;

    // Añadir variación de color por bandas
    if band_intensity > 0.1 {
        base_color = Color::new(
            (base_color.r() as f32 * (1.0 - band_intensity) + bright_blue.r() as f32 * band_intensity) as u8,
            (base_color.g() as f32 * (1.0 - band_intensity) + bright_blue.g() as f32 * band_intensity) as u8,
            (base_color.b() as f32 * (1.0 - band_intensity) + bright_blue.b() as f32 * band_intensity) as u8,
        );
    }

    // Añadir nubes de metano
    if methane_clouds > 0.2 {
        let cloud_blend = (methane_clouds - 0.2) / 0.8;
        base_color = Color::new(
            (base_color.r() as f32 * (1.0 - cloud_blend) + methane_blue.r() as f32 * cloud_blend) as u8,
            (base_color.g() as f32 * (1.0 - cloud_blend) + methane_blue.g() as f32 * cloud_blend) as u8,
            (base_color.b() as f32 * (1.0 - cloud_blend) + methane_blue.b() as f32 * cloud_blend) as u8,
        );
    }

    // Añadir Gran Mancha Oscura
    if dark_spot_factor > 0.1 {
        let dark_spot_color = Color::new(20, 40, 100);
        base_color = Color::new(
            (base_color.r() as f32 * (1.0 - dark_spot_factor) + dark_spot_color.r() as f32 * dark_spot_factor) as u8,
            (base_color.g() as f32 * (1.0 - dark_spot_factor) + dark_spot_color.g() as f32 * dark_spot_factor) as u8,
            (base_color.b() as f32 * (1.0 - dark_spot_factor) + dark_spot_color.b() as f32 * dark_spot_factor) as u8,
        );
    }

    // Añadir manchas blancas (nubes brillantes)
    if white_spot_factor > 0.1 {
        let white_cloud_color = Color::new(200, 220, 255);
        base_color = Color::new(
            (base_color.r() as f32 * (1.0 - white_spot_factor) + white_cloud_color.r() as f32 * white_spot_factor) as u8,
            (base_color.g() as f32 * (1.0 - white_spot_factor) + white_cloud_color.g() as f32 * white_spot_factor) as u8,
            (base_color.b() as f32 * (1.0 - white_spot_factor) + white_cloud_color.b() as f32 * white_spot_factor) as u8,
        );
    }

    // Añadir remolinos
    if swirl_factor > 0.1 {
        let swirl_color = Color::new(60, 90, 180);
        base_color = Color::new(
            (base_color.r() as f32 * (1.0 - swirl_factor) + swirl_color.r() as f32 * swirl_factor) as u8,
            (base_color.g() as f32 * (1.0 - swirl_factor) + swirl_color.g() as f32 * swirl_factor) as u8,
            (base_color.b() as f32 * (1.0 - swirl_factor) + swirl_color.b() as f32 * swirl_factor) as u8,
        );
    }

    // === ANILLOS SUTILES (simulados) ===
    let ring_mask = (1.0 - latitude.abs() * 8.0).max(0.0).powf(2.0);
    let ring_pattern = (sphere_pos.x * 20.0 + time * 0.1).sin().abs();
    let ring_factor = ring_mask * ring_pattern * 0.15;

    if ring_factor > 0.05 {
        let ring_color = Color::new(70, 100, 180);
        base_color = Color::new(
            (base_color.r() as f32 * (1.0 - ring_factor) + ring_color.r() as f32 * ring_factor) as u8,
            (base_color.g() as f32 * (1.0 - ring_factor) + ring_color.g() as f32 * ring_factor) as u8,
            (base_color.b() as f32 * (1.0 - ring_factor) + ring_color.b() as f32 * ring_factor) as u8,
        );
    }

    // === EFECTO DE ATMÓSFERA PROFUNDA ===
    let view_dir = Vec3::new(0.0, 0.0, 1.0);
    let fresnel = 1.0 - dot(&normal, &view_dir).abs();
    let atmosphere_glow = fresnel.powf(3.0) * 0.4;
    
    let atmosphere_color = Color::new(100, 150, 255);
    let final_color = if atmosphere_glow > 0.0 {
        Color::new(
            (base_color.r() as f32 * (1.0 - atmosphere_glow) + atmosphere_color.r() as f32 * atmosphere_glow) as u8,
            (base_color.g() as f32 * (1.0 - atmosphere_glow) + atmosphere_color.g() as f32 * atmosphere_glow) as u8,
            (base_color.b() as f32 * (1.0 - atmosphere_glow) + atmosphere_color.b() as f32 * atmosphere_glow) as u8,
        )
    } else {
        base_color
    };

    
    let light_dir_normalized = light_dir.normalize();
    let diffuse = dot(&normal, &light_dir_normalized).max(0.0) * 0.6; // Menos luz directa
    let ambient = 0.4; // Más luz ambiente por la dispersión atmosférica
    let intensity = (diffuse + ambient).min(1.0);

    Color::new(
        (final_color.r() as f32 * intensity).clamp(0.0, 255.0) as u8,
        (final_color.g() as f32 * intensity).clamp(0.0, 255.0) as u8,
        (final_color.b() as f32 * intensity).clamp(0.0, 255.0) as u8,
    )
}



