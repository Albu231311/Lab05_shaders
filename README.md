#  Sistema Solar - Renderer con Shaders Procedurales

##  Descripción del Proyecto

Este proyecto implementa un **sistema de renderizado de planetas usando únicamente shaders procedurales**, sin el uso de texturas o materiales pregenerados. Cada planeta está creado mediante funciones de ruido procedural (Perlin/Simplex noise) y técnicas de mezclado de colores en tiempo real.

El objetivo principal fue crear cuerpos celestes realistas y visualmente atractivos utilizando solo código y matemáticas, demostrando el poder de los shaders procedurales para generar contenido complejo.

---

## Controles

### Movimiento de Cámara
- **Flechas**: Mover el planeta visible
- **A/S**: Zoom in/out (cambiar escala)
- **Q/W**: Rotar en eje X
- **E/R**: Rotar en eje Z

### Selección de Planetas
- **1**: Tierra (con Luna)
- **2**: Sol
- **3**: Júpiter (Gigante Gaseoso)
- **4**: Marte
- **5**: Mercurio
- **6**: Neptuno

### Controles de la Luna (solo en modo Tierra)
- **Y**: Aumentar velocidad orbital
- **I**: Disminuir velocidad orbital

### General
- **ESC**: Salir

---


## Técnicas de Shader Utilizadas

### Ruido Procedural
```rust
// Ruido base 3D
pub fn noise_3d(p: Vec3) -> f32

// Ruido fractal (múltiples octavas)
pub fn fractal_noise(p: Vec3, octaves: i32) -> f32

// Ruido específico para la Tierra
pub fn fractal_noise_earth(p: Vec3, octaves: i32) -> f32
```



## Estructura del Código

```
src/
├── main.rs              # Loop principal y manejo de input
├── planet_shaders.rs    # Shaders de todos los planetas
├── noise.rs             # Funciones de ruido procedural
├── color.rs             # Estructura de color RGB
├── framebuffer.rs       # Buffer de renderizado
├── triangle.rs          # Rasterización de triángulos
├── vertex.rs            # Estructura de vértices
├── fragment.rs          # Estructura de fragmentos
└── shaders.rs           # Vertex shader
```



## Screenshots


### Tierra con Luna
<img width="799" height="597" alt="image" src="https://github.com/user-attachments/assets/b9ac3e7c-efba-4504-8570-79040b76ec1c" />


### Sol
<img width="799" height="596" alt="image" src="https://github.com/user-attachments/assets/042b77cd-ac29-4f08-9cf2-85a0ff2e29bc" />


### Júpiter
<img width="799" height="601" alt="image" src="https://github.com/user-attachments/assets/c878920e-a21f-4e78-882e-501a6c72a7bf" />


### Marte
<img width="797" height="597" alt="image" src="https://github.com/user-attachments/assets/3725796c-89d3-43a4-84f4-d3be5f76cec5" />


### Mercurio
<img width="802" height="599" alt="image" src="https://github.com/user-attachments/assets/78641723-c261-4fa0-96ec-3928b41422f9" />


### Neptuno
<img width="798" height="597" alt="image" src="https://github.com/user-attachments/assets/460b2496-a338-4e89-a666-2cd63beccdcd" />


---

## Compilación y Ejecución

### Clonar el Repositorio
```bash
git clone https://github.com/Albu231311/Lab05_shaders.git
cd Lab05_shaders
```

### Ejecutar el Proyecto
```bash

cargo run --release
```

### Dependencias
- `nalgebra-glm`: Álgebra lineal y vectores
- `minifb`: Ventana y manejo de eventos
- Rust 1.70+

---

