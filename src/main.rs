//use raytracer::tuple::Tuple;
use raytracer::color::Color;
use raytracer::canvas::Canvas;


// struct Projectile {
//     position: Tuple,
//     velocity: Tuple,
// }


fn main() -> std::io::Result<()> {
    let width = 128;
    let height = 128;
    let mut canvas = Canvas::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let red = x as f64 / (width - 1) as f64;
            let green = y as f64 / (height - 1) as f64;

            canvas.write_pixel(x, y, Color::new(red, green, 0.0));
        }
    }
    canvas.write("canvas.ppm")?;
    println!("Wrote canvas.ppm");
    Ok(())
}