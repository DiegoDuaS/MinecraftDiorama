use nalgebra_glm::{Vec3, dot};
use crate::ray_intersect::{Intersect, RayIntersect};
use crate::material::Material;

pub struct Cube {
    pub min: Vec3, // Punto mínimo del cubo (vértice inferior izquierdo)
    pub max: Vec3, // Punto máximo del cubo (vértice superior derecho)
    pub material: Material,
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let mut tmin = (self.min.x - ray_origin.x) / ray_direction.x;
        let mut tmax = (self.max.x - ray_origin.x) / ray_direction.x;

        if tmin > tmax {
            std::mem::swap(&mut tmin, &mut tmax);
        }

        let mut tymin = (self.min.y - ray_origin.y) / ray_direction.y;
        let mut tymax = (self.max.y - ray_origin.y) / ray_direction.y;

        if tymin > tymax {
            std::mem::swap(&mut tymin, &mut tymax);
        }

        if (tmin > tymax) || (tymin > tmax) {
            return Intersect::empty();
        }

        if tymin > tmin {
            tmin = tymin;
        }

        if tymax < tmax {
            tmax = tymax;
        }

        let mut tzmin = (self.min.z - ray_origin.z) / ray_direction.z;
        let mut tzmax = (self.max.z - ray_origin.z) / ray_direction.z;

        if tzmin > tzmax {
            std::mem::swap(&mut tzmin, &mut tzmax);
        }

        if (tmin > tzmax) || (tzmin > tmax) {
            return Intersect::empty();
        }

        if tzmin > tmin {
            tmin = tzmin;
        }

        if tzmax < tmax {
            tmax = tzmax;
        }

        // Si tmin es positivo, es una intersección en la dirección del rayo
        if tmin > 0.0 {
            let point = ray_origin + ray_direction * tmin;
            let normal = self.calculate_normal(&point); // Calcula la normal en el punto de intersección
            let distance = tmin;

            // Calculate UV coordinates
            let (u, v) = self.calculate_uv(&point);
            
            return Intersect::new(point, normal, distance, self.material.clone(), (u, v));
        }

        Intersect::empty()
    }
}


impl Cube {
    fn calculate_normal(&self, point: &Vec3) -> Vec3 {
        // Comparamos el punto de intersección con las caras del cubo para determinar la normal
        let epsilon = 1e-4; // Un pequeño valor para la precisión

        if (point.x - self.min.x).abs() < epsilon {
            return Vec3::new(-1.0, 0.0, 0.0); // Cara izquierda
        } else if (point.x - self.max.x).abs() < epsilon {
            return Vec3::new(1.0, 0.0, 0.0); // Cara derecha
        } else if (point.y - self.min.y).abs() < epsilon {
            return Vec3::new(0.0, -1.0, 0.0); // Cara inferior
        } else if (point.y - self.max.y).abs() < epsilon {
            return Vec3::new(0.0, 1.0, 0.0); // Cara superior
        } else if (point.z - self.min.z).abs() < epsilon {
            return Vec3::new(0.0, 0.0, -1.0); // Cara trasera
        } else if (point.z - self.max.z).abs() < epsilon {
            return Vec3::new(0.0, 0.0, 1.0); // Cara frontal
        }

        Vec3::new(0.0, 0.0, 0.0) // Normal por defecto (si no se encuentra coincidencia)
    }

    fn calculate_uv(&self, point: &Vec3) -> (f32, f32) {
        let epsilon = 1e-4;
    
        if (point.x - self.min.x).abs() < epsilon {
            // Cara izquierda (eje X negativo)
            let u = (point.z - self.min.z) / (self.max.z - self.min.z);
            let v = (self.max.y - point.y) / (self.max.y - self.min.y); 
            return(u, v);
        } else if (point.x - self.max.x).abs() < epsilon {
            // Cara derecha (eje X positivo)
            let u = (point.z - self.min.z) / (self.max.z - self.min.z);
            let v = (self.max.y - point.y) / (self.max.y - self.min.y); 
            return(u, v);
        } else if (point.y - self.min.y).abs() < epsilon {
            // Cara inferior (eje Y negativo)
            let u = (point.x - self.min.x) / (self.max.x - self.min.x);
            let v = (point.z - self.min.z) / (self.max.z - self.min.z);
            return(u, v);
        } else if (point.y - self.max.y).abs() < epsilon {
            // Cara superior (eje Y positivo) - Ya funciona bien
            let u = (point.x - self.min.x) / (self.max.x - self.min.x);
            let v = (point.z - self.min.z) / (self.max.z - self.min.z);
            return(u, v);
        } else if (point.z - self.min.z).abs() < epsilon {
            // Cara trasera (eje Z negativo)
            let u = (self.max.x - point.x) / (self.max.x - self.min.x);
            let v = (self.max.y - point.y) / (self.max.y - self.min.y);
            return(u, v);
        } else {
            // Cara frontal (eje Z positivo) - Ya funciona bien
            let u = (point.x - self.min.x) / (self.max.x - self.min.x);
            let v = (self.max.y - point.y) / (self.max.y - self.min.y);
            return(u, v);
        }

        (0.0, 0.0) // Default UV if no face is matched
    }
}
