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
    let shadow_ray_origin = intersect.point;
    let mut shadow_intensity = 0.0;

    for object in objects {
        let shadow_intersect = object.ray_intersect(&shadow_ray_origin, &light_dir);
        
        if shadow_intersect.is_intersecting {
            let distance_to_light = (shadow_intersect.point - light.position).magnitude();
            
            let intensity = 1.0 / (distance_to_light + 1.0); 
            shadow_intensity = intensity.clamp(0.0, 1.0);
            break;
        }
    }

    shadow_intensity
}


pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Cube], light: &Light, depth: u32) -> Color {

    if depth > 3 {
        return Color::new(179, 179, 179)
    }

    let mut intersect =  Intersect::empty();
    let mut zbuffer = INFINITY;

    for object in objects {
        let i = object.ray_intersect(&ray_origin, &ray_direction);
        if i.is_intersecting && i.distance < zbuffer {
            zbuffer = i.distance;
            intersect = i;
        }
    }

    if !intersect.is_intersecting{
        return Color::new(179, 179, 179);
    }

    let light_dir = (light.position - intersect.point).normalize();
    let view_dir = (ray_origin - intersect.point).normalize();
    let reflect_dir = reflect(&-light_dir, &intersect.normal).normalize();
    
    let shadow_intensity = cast_shadow(&intersect, light, objects);
    let light_intensity = light.intensity * (1.0 - shadow_intensity);

    let diffuse_intensity = intersect.normal.dot(&light_dir);
    let diffuse_color = intersect.material.get_diffuse_color(intersect.uv.0, intersect.uv.1);
    let diffuse = diffuse_color * intersect.material.albedo[0] * diffuse_intensity * light_intensity;

    let specular_intesity = view_dir.dot(&reflect_dir).max(0.0).powf(intersect.material.spec);
    let specular = light.color * intersect.material.albedo[1] * specular_intesity * light_intensity;

    let mut reflect_color = Color::black();
    let reflectivity = intersect.material.reflectivity;
    if reflectivity > 0.0 {
        let reflect_dir = reflect(&ray_direction, &intersect.normal).normalize();
        let reflect_origin = intersect.point;
        reflect_color = cast_ray(&reflect_origin, &reflect_dir, objects, light, depth + 1);
    }

    let mut refract_color = Color::black();
    let transparency = intersect.material.transparency;
    if transparency > 0.0 {
        let refract_dir = refract(&ray_direction, &intersect.normal, intersect.material.refraction_index).normalize();
        let refract_origin = offset_origin(&intersect, &refract_dir);
        refract_color = cast_ray(&refract_origin, &refract_dir, objects, light, depth + 1);
    }


    (diffuse + specular) * (1.0 - reflectivity - transparency) + (reflect_color * reflectivity) + (refract_color * transparency)
}