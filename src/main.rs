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



pub fn render(framebuffer: &mut Framebuffer, objects: &[Cube], camera: &Camera, light: &Light) {
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

            let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects, light, 0);
            *pixel = pixel_color.to_u32();
        });
    });
}

fn main() {
    let texture_netherrack = Texture::new("./assets/netherrack.png");
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

    let rubber = Material::new(
        Color::new(80, 0, 0),
        10.0,
        [0.9, 0.1],
        0.0,
        0.0,
        0.0,
    );
    
    let ivory = Material::new(
        Color::new(100, 100, 100),
        50.0,
        [0.6, 0.3],
        0.6,
        0.0,
        0.0,
    );
    
    let glass = Material::new(
        Color::new(255, 255, 255),
        1425.0,
        [0.0, 10.0],
        0.5,
        0.5,
        0.3,
    );
    
    let netherrack = Material::new_with_texture(
        100.0,
        [0.7, 0.3],
        0.1,
        0.0,
        0.0,
        TextureType::Netherrack
    );
    

    let mut objects = Vec::new(); // Usamos un Vec para almacenar los cubos

        for i in 0..10 {
            for j in 0..10 {
                objects.push(Cube {
                    min: Vec3::new(i as f32, -1.0, j as f32),  // Vértice inferior izquierdo
                    max: Vec3::new(i as f32 + 1.0, 1.0, j as f32 + 1.0), // Vértice superior derecho
                    material: netherrack.clone(), // Material del cubo
                });
            }
        }


    let mut camera = Camera::new(
        Vec3::new(0.0,0.0,15.0),
        Vec3::new(0.0,0.0,0.0),
        Vec3::new(0.0,1.0,0.0), 
    );

    let light = Light::new(
        Vec3::new(5.0,10.0,24.0), 
        Color::new(255,255,255), 
        2.0
    );


    let rotaton_speed = PI/50.0;

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

        framebuffer.clear();

        render(
            &mut framebuffer,
            &objects,
            &mut camera,
            &light
        );

        window
            .update_with_buffer(&framebuffer.buffer, width, height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

