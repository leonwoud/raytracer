//use raytracer::tuple::Tuple;
use raytracer::canvas::Canvas;
use raytracer::color::Color;
use raytracer::geometry::Shape;
use raytracer::intersection::hit;
use raytracer::lighting::{PointLight, lighting};
use raytracer::material::Material;
use raytracer::ray::Ray;
use raytracer::spheres::Sphere;
use raytracer::transform;
use raytracer::tuple::Tuple;

fn main() -> std::io::Result<()> {
    let width = 512;
    let height = 512;
    let start = std::time::Instant::now();
    let mut canvas = Canvas::new(width, height);

    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let mut sphere = Sphere::new();

    sphere.set_transform(transform::scale(0.75, 0.75, 0.75));

    let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let mut material = Material::default();
    material.color = Color::new(1.0, 0.2, 1.0);

    sphere.set_material(material);

    for y in 0..height {
        for x in 0..width {
            let world_x = (x as f64 / width as f64) * 2.0 - 1.0;
            let world_y = (y as f64 / height as f64) * 2.0 - 1.0;
            let position = Tuple::point(world_x, world_y, 0.0);
            let direction = (position - ray_origin).normalize();
            let ray = Ray::new(ray_origin, direction);
            let intersections = sphere.intersect(&ray);
            if !intersections.is_empty() {
                let hit = hit(&intersections).unwrap();
                let point = ray.position(hit.t);
                let normal = hit.object.normal_at(point);
                let eye = -ray.direction;
                let color = lighting(&hit.object.material(), &light, point, eye, normal);
                canvas.write_pixel(x, y, color);
            }
        }
    }
    println!("Rendered {width}x{height} in {:.2?}", start.elapsed());
    let file_start = std::time::Instant::now();
    canvas.write_binary("canvas.ppm")?;
    let file_elapsed = file_start.elapsed();
    println!("Wrote canvas.ppm to disk in: {:.2?}", file_elapsed);
    Ok(())
}
