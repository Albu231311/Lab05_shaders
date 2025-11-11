use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn black() -> Self {
        Color { r: 0, g: 0, b: 0 }
    }

    pub fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    pub fn r(&self) -> u8 { self.r }
    pub fn g(&self) -> u8 { self.g }
    pub fn b(&self) -> u8 { self.b }

    /// 🔹 Interpolación lineal entre dos colores (a y b) usando t ∈ [0,1]
    pub fn lerp(a: &Color, b: &Color, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        Color::new(
            (a.r() as f32 * (1.0 - t) + b.r() as f32 * t) as u8,
            (a.g() as f32 * (1.0 - t) + b.g() as f32 * t) as u8,
            (a.b() as f32 * (1.0 - t) + b.b() as f32 * t) as u8,
        )
    }

    /// 🔹 Escala el brillo del color por un factor (por ejemplo, para iluminación)
    pub fn scale(&self, factor: f32) -> Color {
        Color::new(
            (self.r() as f32 * factor).min(255.0) as u8,
            (self.g() as f32 * factor).min(255.0) as u8,
            (self.b() as f32 * factor).min(255.0) as u8,
        )
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color(r: {}, g: {}, b: {})", self.r, self.g, self.b)
    }
}
