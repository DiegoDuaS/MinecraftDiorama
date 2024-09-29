use once_cell::sync::Lazy;
use std::sync::Arc;

use crate::color::Color;
use crate::texture::Texture;

// Static texture initialization
static TEXTURES: Lazy<Vec<Arc<Texture>>> = Lazy::new(|| {
    vec![
        Arc::new(Texture::new("./assets/netherrack.png")), // Puedes agregar más texturas aquí
    ]
});

#[derive(Debug, Clone)]
pub enum TextureType {
    Netherrack,
}

#[derive(Debug, Clone)]
pub struct Material {
    pub diffuse: Color,
    pub spec: f32,
    pub albedo: [f32; 2],
    pub reflectivity: f32,
    pub transparency: f32,
    pub refraction_index: f32,
    pub has_texture: bool,
    pub texture_index: usize, // Almacena el índice de la textura
}

impl Material {
    // Crear un nuevo material sin textura
    pub fn new(
        diffuse: Color,
        spec: f32,
        albedo: [f32; 2],
        reflectivity: f32,
        transparency: f32,
        refraction_index: f32,
    ) -> Self {
        Material {
            diffuse,
            spec,
            albedo,
            reflectivity,
            transparency,
            refraction_index,
            has_texture: false,
            texture_index: 0, // Default a 0, sin textura inicialmente
        }
    }

    // Crear un nuevo material con una textura específica
    pub fn new_with_texture(
        spec: f32,
        albedo: [f32; 2],
        reflectivity: f32,
        transparency: f32,
        refraction_index: f32,
        texture_type: TextureType, // Solo un tipo de textura
    ) -> Self {
        let texture_index = match texture_type {
            TextureType::Netherrack => 0, // Suponiendo que Netherrack es el primer índice
        };

        Material {
            diffuse: Color::new(0, 0, 0), 
            spec,
            albedo,
            reflectivity,
            transparency,
            refraction_index,
            has_texture: true,
            texture_index, // Guarda el índice de la textura
        }
    }

    // Obtener el color de la textura activa
    pub fn get_diffuse_color(&self, u: f32, v: f32) -> Color {
        if self.has_texture {
            let texture = &TEXTURES[self.texture_index]; // Usar la textura por índice
            let x = (u * (texture.width as f32 - 1.0)) as usize;
            let y = ((1.0 - v) * (texture.height as f32 - 1.0)) as usize;
            return texture.get_color(x, y);
        }
        self.diffuse // Fallback a color difuso si no hay textura
    }

    pub fn black() -> Self {
        Material {
            diffuse: Color::new(0, 0, 0),
            spec: 0.0,
            albedo: [0.0, 0.0],
            reflectivity: 0.0,
            transparency: 0.0,
            refraction_index: 0.0,
            has_texture: false,
            texture_index: 0, // Default a 0, sin textura
        }
    }
}
