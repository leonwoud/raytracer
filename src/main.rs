//use raytracer::tuple::Tuple;
use raytracer::color::Color;
use raytracer::canvas::Canvas;
use raytracer::geometry::Shape;
use raytracer::transform;
use raytracer::tuple::Tuple;
use raytracer::ray::Ray;
use raytracer::intersection::Intersection;
use raytracer::spheres::Sphere;

// struct Projectile {
//     position: Tuple,
//     velocity: Tuple,
// }


fn main() -> std::io::Result<()> {
    let width = 512;
    let height = 512;
    let mut canvas = Canvas::new(width, height);

    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let mut sphere = Sphere::new();
    sphere.set_transform(transform::scale(0.75, 0.75, 0.75));
    
    for y in 0..height {
        for x in 0..width {
            let world_x = (x as f64 / width as f64) * 2.0 - 1.0;
            let world_y = (y as f64 / height as f64) * 2.0 - 1.0;
            let position = Tuple::point(world_x, world_y, 0.0);
            let direction = (position - ray_origin).normalize();
            let ray = Ray::new(ray_origin, direction);
            let intersections = sphere.intersect(&ray);
            if !intersections.is_empty() {
                canvas.write_pixel(x, y, Color::new(1.0, 0.0, 0.0));
            }
        }
    }

    canvas.write("canvas.ppm")?;
    println!("Wrote canvas.ppm");
    Ok(())
}