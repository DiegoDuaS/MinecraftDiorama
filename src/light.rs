use nalgebra_glm::Vec3;
use crate::color::Color;

#[derive(Debug, Clone)]
pub struct Light {
    pub position: Vec3,
    pub color: Color,
    pub intensity: f32,
    pub day_angle: f32,  // Ángulo que representa el progreso del ciclo del día
}

impl Light {
    pub fn new(position: Vec3, color: Color, intensity: f32) -> Self {
        Light {
            position,
            color,
            intensity,
            day_angle: 0.0,  // Iniciar el ángulo en el amanecer
        }
    }

    pub fn update_light(&mut self) {
        // Cambiar la posición de la luz en función del ángulo del día
        let radius = 10.0;  // Radio de la trayectoria del sol
        self.position = Vec3::new(
            radius * self.day_angle.cos(),  // Movimiento en el plano XZ
            radius * self.day_angle.sin(),  // Movimiento vertical (Y)
            6.0
        );
    
        // Condiciones para el amanecer/atardecer (cuando el sol está cerca del horizonte)
        let sunrise_threshold = 0.5;  // Cercanía al horizonte por la mañana (0 radianes)
        let sunset_threshold = std::f32::consts::PI - 0.5;  // Cercanía al horizonte por la tarde (PI radianes)
    
        if self.day_angle.sin() > 0.0 {
            // Día (sol arriba del horizonte)
            if self.day_angle < sunrise_threshold || self.day_angle > sunset_threshold {
                // Amanecer o atardecer (naranja cálido)
                self.intensity = 1.0;
                self.color = Color::new(255, 185, 46);  // Luz naranja
            } else {
                // Día pleno (luz amarilla suave)
                self.intensity = 1.2;
                self.color = Color::new(255, 255, 204);  // Luz amarilla suave
            }
        } else {
            // Noche (sol debajo del horizonte)
            self.intensity = 0.1;
            self.color = Color::new(64, 64, 128);  // Luz tenue azulada
        }
    }

    pub fn advance_day_cycle(&mut self, delta_angle: f32) {
        // Actualizar el ángulo del día (debe estar entre 0 y 2 * PI)
        self.day_angle = (self.day_angle + delta_angle) % (2.0 * std::f32::consts::PI);
    }
}
