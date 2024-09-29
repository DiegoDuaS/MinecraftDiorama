use nalgebra_glm::Vec3;
use std::f32::INFINITY;


use crate::ray_intersect::RayIntersect;

use crate::Intersect;
use crate::Light;
use crate::Cube;
use crate::Color;

const ORIGIN_BIAS: f32 = 1e-4;

fn offset_origin(intersect: &Intersect, direction: &Vec3) -> Vec3{
    let offset = intersect.normal * ORIGIN_BIAS;
    if direction.dot(&intersect.normal) < 0.0 {
        intersect.point - offset
    } else {
        intersect.point + offset
    }
}

fn refract(incident: &Vec3, normal: &Vec3, eta_t: f32) -> Vec3 {
    let cosi = -incident.dot(normal).max(-1.0).min(1.0);

    let (n_cosi, eta, n_normal);

    if cosi < 0.0 {
        n_cosi = -cosi;
        eta = 1.0 / eta_t;
        n_normal = -normal;
    } else {
        n_cosi = cosi;
        eta = eta_t;
        n_normal = *normal;
    }

    let k = 1.0 - eta * eta * (1.0 - n_cosi * n_cosi);

    if k < 0.0{
        reflect(incident, &n_normal)
    } else {
        eta * incident + (eta * n_cosi - k.sqrt()) * n_normal
    }
}

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3{
    incident - 2.0 * incident.dot(normal) * normal
}

fn cast_shadow(
    intersect: &Intersect,
    light: &Light,
    objects: &[Cube],
) -> f32 {
    let light_dir = (light.position - intersect.point).normalize();
    let shadow_ray_origin = offset_origin(intersect, &light_dir); // Usar offset para evitar auto-sombra
    let light_distance = (light.position - shadow_ray_origin).magnitude(); // Distancia a la luz

    for object in objects {
        let shadow_intersect = object.ray_intersect(&shadow_ray_origin, &light_dir);

        // Verificar si el objeto está entre el punto y la luz
        if shadow_intersect.is_intersecting && shadow_intersect.distance < light_distance {
            return 1.0; // Sombra completa
        }
    }

    0.0 // Sin sombra
}


pub fn cast_ray(
    ray_origin: &Vec3, 
    ray_direction: &Vec3, 
    objects: &[Cube], 
    daylight: &Light, 
    other_lights: &[Light], 
    depth: u32
) -> Color {

    if depth > 3 {
        return Color::new(179, 179, 179); // Color de fondo si se alcanza la profundidad máxima
    }

    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY; // Inicializar el zbuffer con el valor máximo

    // Bucle para encontrar la intersección más cercana
    for object in objects {
        let i = object.ray_intersect(&ray_origin, &ray_direction);
        if i.is_intersecting && i.distance < zbuffer {
            zbuffer = i.distance; // Actualizar el zbuffer con la distancia más cercana
            intersect = i; // Actualizar el objeto de intersección
        }
    }

    // Si no hay intersección, devolver el color de fondo
    if !intersect.is_intersecting {
        // Cambiar el color de fondo dependiendo de las propiedades de daylight
        let background_color = calculate_background_color(daylight);
        return background_color;
    }
    // Función auxiliar para calcular la luz total
    let calculate_light_intensity = |light: &Light| {
        let light_dir = (light.position - intersect.point).normalize();
        let view_dir = (ray_origin - intersect.point).normalize();
        let reflect_dir = reflect(&-light_dir, &intersect.normal).normalize();
        
        // Intensidad de la sombra
        let shadow_intensity = cast_shadow(&intersect, light, objects);
        let light_intensity = light.intensity * (1.0 - shadow_intensity);
        
        // Cálculo de iluminación difusa
        let diffuse_intensity = intersect.normal.dot(&light_dir).max(0.0);
        let diffuse_color = intersect.material.get_diffuse_color(intersect.uv.0, intersect.uv.1);
        let diffuse = ((light.color * 0.09) + diffuse_color) * intersect.material.albedo[0] * diffuse_intensity * light_intensity;

        // Cálculo de iluminación especular
        let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(intersect.material.spec);
        let specular = light.color * intersect.material.albedo[1] * specular_intensity * light_intensity;

        // Combinar difuso y especular
        diffuse + specular
    };

    // Calcular la luz total (luz del día + otras fuentes de luz)
    let mut total_light = calculate_light_intensity(daylight);
    for light in other_lights {
        total_light = total_light + calculate_light_intensity(light);
    }

    // Color final (sin reflejos y refracciones)
    let reflectivity = intersect.material.reflectivity;
    let transparency = intersect.material.transparency;

    // Reflejos
    let mut reflect_color = Color::black();
    if reflectivity > 0.0 {
        let reflect_dir = reflect(&ray_direction, &intersect.normal).normalize();
        let reflect_origin = intersect.point + intersect.normal * 0.001; // Ajustar el origen del rayo
        reflect_color = cast_ray(&reflect_origin, &reflect_dir, objects, daylight, other_lights, depth + 1);
    }

    // Refracción
    let mut refract_color = Color::black();
    if transparency > 0.0 {
        let refract_dir = refract(&ray_direction, &intersect.normal, intersect.material.refraction_index).normalize();
        let refract_origin = offset_origin(&intersect, &refract_dir);
        refract_color = cast_ray(&refract_origin, &refract_dir, objects, daylight, other_lights, depth + 1);
    }

    // Combinación final de colores
    total_light * (1.0 - reflectivity - transparency) + (reflect_color * reflectivity) + (refract_color * transparency)
}

fn calculate_background_color(daylight: &Light) -> Color {
    let angle = daylight.day_angle;

    const DAY_START: f32 = 0.0;                           // Amanecer
    const DAY_MID: f32 = std::f32::consts::PI / 3.0;     // Medio día (reducido)
    const DAY_END: f32 = std::f32::consts::PI * 2.0 / 3.0; // Atardecer (reducido)
    const NIGHT_START: f32 = DAY_END + 2.0;               // Inicio de la Noche   

    let r: u8;
    let g: u8;
    let b: u8;

    if angle >= DAY_START && angle < DAY_MID {
        // Gradiente de azul oscuro a amarillo-naranja y luego a celeste (Amanecer)
        let ratio = angle / (DAY_MID - DAY_START);
        
        // De azul oscuro (0) a naranja (255) a celeste (135)
        if ratio < 0.5 {
            let sub_ratio = ratio * 2.0; // Escalamos a [0, 1]
            r = (0.0 * (1.0 - sub_ratio) + 220.0 * sub_ratio) as u8; // Azul oscuro a naranja (220)
            g = (0.0 * (1.0 - sub_ratio) + 162.0 * sub_ratio) as u8; // Azul oscuro a naranja (162)
            b = (50.0 * (1.0 - sub_ratio) + 30.0 * sub_ratio) as u8; // Azul oscuro a naranja (30)
        } else {
            let sub_ratio = (ratio - 0.5) * 2.0; // Escalamos a [0, 1]
            r = (220.0 * (1.0 - sub_ratio) + 135.0 * sub_ratio) as u8; // Naranja (220) a celeste (135)
            g = (162.0 * (1.0 - sub_ratio) + 206.0 * sub_ratio) as u8; // Naranja (162) a celeste (206)
            b = (30.0 * (1.0 - sub_ratio) + 250.0 * sub_ratio) as u8; // Naranja (30) a celeste (250)
        }

    } else if angle >= DAY_MID && angle < DAY_END {
        // Día: Celeste
        r = 135;
        g = 206;
        b = 250;

    } else if angle >= DAY_END && angle < NIGHT_START {
        // Gradiente de celeste a amarillo-naranja y luego a azul oscuro (Atardecer)
        let ratio = (angle - DAY_END) / (NIGHT_START - DAY_END);
        
        // De celeste (135) a amarillo-naranja (255)
        if ratio < 0.5 {
            let sub_ratio = ratio * 2.0; // Escalamos a [0, 1]
            r = (135.0 * (1.0 - sub_ratio) + 220.0 * sub_ratio) as u8; // Celeste a naranja (220)
            g = (206.0 * (1.0 - sub_ratio) + 162.0 * sub_ratio) as u8; // Celeste a naranja (162)
            b = (250.0 * (1.0 - sub_ratio) + 30.0 * sub_ratio) as u8; // Celeste a naranja (30)
        } else {
            let sub_ratio = (ratio - 0.5) * 2.0; // Escalamos a [0, 1]
            r = (220.0 * (1.0 - sub_ratio) + 0.0 * sub_ratio) as u8; // Naranja (220) a azul oscuro (0)
            g = (162.0 * (1.0 - sub_ratio) + 0.0 * sub_ratio) as u8; // Naranja (162) a azul oscuro (0)
            b = (30.0 * (1.0 - sub_ratio) + 50.0 * sub_ratio) as u8; // Naranja (30) a azul oscuro (50)
        }

    } else {
        // Noche: Azul oscuro
        r = 0; 
        g = 0; 
        b = 50; // Azul oscuro
    }

    Color::new(r as i32, g as i32, b as i32)
}

