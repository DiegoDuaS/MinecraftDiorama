use material::{Material, TextureType};
use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::Vec3;
use std::{f32::consts::PI, time::Duration};
use rayon::prelude::*;
use once_cell::sync::Lazy;
use std::sync::Arc;


mod framebuffer;
use framebuffer::Framebuffer;

mod color;
use color::Color;

mod ray_intersect;
use ray_intersect::Intersect;

mod material;

mod camera;
use camera::Camera;

mod cube;
use cube::Cube;

mod light;
use light::Light;

mod castray;
use castray::cast_ray;

mod texture;
use texture::Texture;



pub fn render(framebuffer: &mut Framebuffer, objects: &[Cube], camera: &Camera, light: &[Light], daylight: &Light) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov = PI / 3.0;
    let perspective_scale = (fov / 2.0).tan();

    framebuffer.buffer.par_chunks_mut(framebuffer.width as usize).enumerate().for_each(|(y, row)| {
        let screen_y = -(2.0 * y as f32) / height + 1.0;
        let screen_y = screen_y * perspective_scale;

        row.iter_mut().enumerate().for_each(|(x, pixel)| {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_x = screen_x * aspect_ratio * perspective_scale;

            let ray_direction = Vec3::new(screen_x, screen_y, -1.0).normalize();
            let rotated_direction = camera.basis_change(&ray_direction);

            let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects, daylight, light, 0);
            *pixel = pixel_color.to_u32();
        });
    });
}

fn main() {
    let width = 1000;
    let height = 550;

    let mut framebuffer = Framebuffer::new(width, height);

    let frame_delay = Duration::from_millis(0);

   
    let mut window = Window::new(
        "Raytracing",
        width,
        height,
        WindowOptions::default(),
    ).unwrap();

    window.set_position(100, 100);
    window.update();

    framebuffer.set_background_color(Color::new(179, 179, 179));

    
    let netherrack = Material::new_with_texture(
        30.0,
        [0.6, 0.2],
        0.0,
        0.0,
        0.0,
        TextureType::Netherrack
    );

    let obsidian = Material::new_with_texture(
        50.0,
        [0.6, 0.5],
        0.0,
        0.0,
        0.0,
        TextureType::Obsidian
    );

    let ruinedobsidian = Material::new_with_texture(
        40.0,
        [0.6, 0.3],
        0.0,
        0.0,
        0.0,
        TextureType::RuinedObsidian
    );

    let magma = Material::new_with_texture(
        100.0,
        [0.7, 0.7],
        0.1,
        0.0,
        0.0,
        TextureType::MagmaBlock
    );

    let lava = Material::new_with_texture(
        100.0,
        [0.7, 0.9],
        0.1,
        0.2,
        0.3,
        TextureType::Lava
    );

    let mut objects = Vec::new();
    let lights = vec![
        Light::new(Vec3::new(2.75, -0.4, 0.25), Color::new(255, 185, 46), 1.0),
        Light::new(Vec3::new(2.75, -0.20, 0.1), Color::new(255, 185, 46), 1.0),
        Light::new(Vec3::new(1.75, -0.4, 1.25), Color::new(255, 185, 46), 1.0),
        Light::new(Vec3::new(1.75, -0.20, 1.50), Color::new(255, 185, 46), 1.0),
        Light::new(Vec3::new(1.25, -0.4, 1.75), Color::new(255, 185, 46), 1.0),
        Light::new(Vec3::new(1.25, -0.20, 2.1), Color::new(255, 185, 46), 1.0),
    ];
    let cube_size = 0.5;  // Tamaño del cubo

    for i in 0..6 { // Número de cubos en la dirección x 
        for j in 0..4 { // Número de cubos en la dirección z 
            let material = if (i + j) % 3 == 0 {
                magma.clone() // Algunas partes del piso serán magma
            } else if (i + j) % 5 == 0 {
                lava.clone() // Algunas partes del piso serán lava
            } else {
                netherrack.clone() // El resto del piso será netherrack
            };
            let max = if (i + j) % 3 == 0 {
                Vec3::new(i as f32 * cube_size + cube_size, -0.5, j as f32 * cube_size + cube_size) // Algunas partes del piso serán magma
            } else if (i + j) % 5 == 0 {
                Vec3::new(i as f32 * cube_size + cube_size, -0.55, j as f32 * cube_size + cube_size) // Algunas partes del piso serán lava
            } else {
                Vec3::new(i as f32 * cube_size + cube_size, -0.5, j as f32 * cube_size + cube_size) // El resto del piso será netherrack
            };
            objects.push(Cube {
                min: Vec3::new(i as f32 * cube_size, -1.0, j as f32 * cube_size), // Vértice inferior izquierdo
                max, // Vértice superior derecho
                material,
            });
        }
    }

    for i in 0..4 {  // Número de cubos en la dirección x 
        for j in 0..1 {  // Número de cubos en la dirección z 
            let material = if i == 2 {  // Si es el tercer cubo (i == 2), usa ruinedobsidian
                ruinedobsidian.clone()
            } else {
                obsidian.clone()
            };
    
            objects.push(Cube {
                min: Vec3::new((i + 1) as f32 * cube_size, -0.5, (j + 1) as f32 * cube_size),  // Vértice inferior izquierdo
                max: Vec3::new((i + 1) as f32 * cube_size + cube_size, 0.0, (j + 1) as f32 * cube_size + cube_size),  // Vértice superior derecho
                material,  // Material del cubo
            });
        }
    }

    for k in 0..4 {  // 4 bloques hacia arriba 
        let y_min = 0.0 + k as f32 * cube_size;  // Coordenada inferior en y
        let y_max = y_min + cube_size;  // Coordenada superior en y
    
        let material = if k == 2 || k == 3 {  // Si es el tercer o cuarto cubo (k == 2 o k == 3), usa ruinedobsidian
            ruinedobsidian.clone()
        } else {
            obsidian.clone()
        };
    
        objects.push(Cube {
            min: Vec3::new(0.5, y_min, 0.5),  // Vértice inferior izquierdo
            max: Vec3::new(0.5 + cube_size, y_max, 0.5 + cube_size),  // Vértice superior derecho
            material,  // Material del cubo
        });
    }

    for k in 0..3 {  // 3 bloques hacia arriba
        let y_min = 0.0 + k as f32 * cube_size;  // Coordenada inferior en y
        let y_max = y_min + cube_size;  // Coordenada superior en y
    
        let material = if k == 1 {  // Si es el segundo cubo (k == 1), usa ruinedobsidian
            ruinedobsidian.clone()
        } else {
            obsidian.clone()
        };
    
        objects.push(Cube {
            min: Vec3::new(2.0, y_min, 0.5),  // Vértice inferior izquierdo
            max: Vec3::new(2.0 + cube_size, y_max, 0.5 + cube_size),  // Vértice superior derecho
            material,  // Material del cubo
        });
    }

    let mut camera = Camera::new(
        Vec3::new(2.0,0.0,6.0),
        Vec3::new(1.5,1.0,0.0),
        Vec3::new(0.0,1.0,0.0), 
    );

    let mut daylight = Light::new(
        Vec3::new(4.0, 3.0, 7.0), 
        Color::new(229, 156, 19), 
        2.0
    );
    
    

    let rotaton_speed = PI/50.0;
    let zoom_speed = 0.1; 

    // Bucle principal
    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        if window.is_key_down(Key::W) {
            camera.orbit(0.0, -rotaton_speed);
        }
        if window.is_key_down(Key::S) {
            camera.orbit(0.0, rotaton_speed);
        }
        if window.is_key_down(Key::A) {
            camera.orbit(rotaton_speed, 0.0);
        }
        if window.is_key_down(Key::D) {
            camera.orbit(-rotaton_speed, 0.0);
        }

        if window.is_key_down(Key::Up) {
            camera.zoom(zoom_speed); 
        }
        if window.is_key_down(Key::Down) {
            camera.zoom(-zoom_speed);  
        }

        let day_speed = 0.05;  // Controla la velocidad del ciclo de día

        if window.is_key_down(Key::Right) {
            daylight.advance_day_cycle(day_speed);  // Avanza el ciclo hacia el mediodía/noche
        }
        if window.is_key_down(Key::Left) {
           daylight.advance_day_cycle(-day_speed);  // Retrocede el ciclo hacia el amanecer
        }

        daylight.update_light();

        framebuffer.clear();

        render(
            &mut framebuffer,
            &objects,
            &mut camera,
            &lights,
            &daylight
        );

        window
            .update_with_buffer(&framebuffer.buffer, width, height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

