use crate::color::Color;
use crate::tuple::Tuple;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PointLight {
    pub position: Tuple,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(position: Tuple, intensity: Color) -> Self {
        PointLight {
            position,
            intensity,
        }
    }
}

pub fn lighting(
    material: &crate::material::Material,
    light: &PointLight,
    point: Tuple,
    eyev: Tuple,
    normalv: Tuple,
) -> Color {
    let effective_color = material.color * light.intensity;
    let lightv = (light.position - point).normalize();
    let ambient = effective_color * material.ambient;

    let light_dot_normal = lightv.dot(&normalv);
    let (diffuse, specular) = if light_dot_normal < 0.0 {
        (Color::new(0.0, 0.0, 0.0), Color::new(0.0, 0.0, 0.0))
    } else {
        let diffuse = effective_color * material.diffuse * light_dot_normal;

        let reflectv = (-lightv).reflect(&normalv);
        let reflect_dot_eye = reflectv.dot(&eyev);

        let specular = if reflect_dot_eye <= 0.0 {
            Color::new(0.0, 0.0, 0.0)
        } else {
            let factor = reflect_dot_eye.powf(material.shininess);
            light.intensity * material.specular * factor
        };

        (diffuse, specular)
    };

    ambient + diffuse + specular
}

#[cfg(test)]
mod tests {
    use super::*;
    //use approx::assert_abs_diff_eq;

    #[test]
    fn test_point_light_creation() {
        let position = Tuple::point(0.0, 0.0, 0.0);
        let intensity = Color::new(1.0, 1.0, 1.0);
        let light = PointLight::new(position, intensity);

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
