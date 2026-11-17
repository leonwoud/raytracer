use crate::color::Color;

/// Scales a 0.0–1.0 colour channel to 0–255, clamping out-of-range values.
fn to_byte(channel: f64) -> u8 {
    (channel * 255.0).round().clamp(0.0, 255.0) as u8
}

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![vec![Color::new(0.0, 0.0, 0.0); width]; height];
        Canvas {
            width,
            height,
            pixels,
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            self.pixels[y][x] = color;
        }
    }

    // fn pixel_at(&self, x: usize, y: usize) -> Color {
    //     if x < self.width && y < self.height {
    //         self.pixels[y][x]
    //     } else {
    //         Color::new(0.0, 0.0, 0.0)
    //     }
    // }

    fn to_ppm(&self) -> String {
        let mut ppm = String::new();
        ppm.push_str(&format!("P3\n{} {}\n255\n", self.width, self.height));
        for row in &self.pixels {
            let mut line = String::new();
            for color in row {
                for channel in [color.red, color.green, color.blue].map(to_byte) {
                    let value = channel.to_string();
                    let separator_len = if line.is_empty() { 0 } else { 1 };

                    if line.len() + separator_len + value.len() > 70 {
                        ppm.push_str(&line);
                        ppm.push('\n');
                        line.clear();
                    }

                    if !line.is_empty() {
                        line.push(' ');
                    }

                    line.push_str(&value);
                }
            }
            if !line.is_empty() {
                ppm.push_str(&line);
                ppm.push('\n');
            }
        }
        ppm
    }

    pub fn write(&self, filename: &str) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;

        let ppm_data = self.to_ppm();
        let mut file = File::create(filename).expect("Unable to create file");
        file.write_all(ppm_data.as_bytes())
            .expect("Unable to write data");
        Ok(())
    }

    pub fn write_ppm_binary<W: std::io::Write>(&self, out: &mut W) -> std::io::Result<()> {
        write!(out, "P6\n{} {}\n255\n", self.width, self.height)?;
        let mut bytes = Vec::with_capacity(self.width * 3);
        for row in &self.pixels {
            bytes.clear();
            for color in row {
                bytes.extend([to_byte(color.red), to_byte(color.green), to_byte(color.blue)]);
            }
            out.write_all(&bytes)?;
        }
        Ok(())
    }

    pub fn write_binary(&self, filename: &str) -> std::io::Result<()> {
        use std::io::Write;
        let mut file = std::io::BufWriter::new(std::fs::File::create(filename)?);
        self.write_ppm_binary(&mut file)?;
        file.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ppm_header() {
        let canvas = Canvas::new(5, 3);
        let ppm = canvas.to_ppm();
        let lines: Vec<&str> = ppm.lines().collect();
        assert_eq!(lines[0], "P3");
        assert_eq!(lines[1], "5 3");
        assert_eq!(lines[2], "255");
    }

    #[test]
    fn test_ppm_pixel_data() {
        let mut canvas = Canvas::new(5, 3);
        let c1 = Color::new(1.0, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(0.0, 0.0, 1.0);
        canvas.write_pixel(0, 0, c1);
        canvas.write_pixel(2, 1, c2);
        canvas.write_pixel(4, 2, c3);
        let ppm = canvas.to_ppm();
        let lines: Vec<&str> = ppm.lines().collect();
        assert_eq!(lines[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(lines[4], "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
        assert_eq!(lines[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
    }

    #[test]
    fn test_ppm_pixel_data_is_clamped() {
        let mut canvas = Canvas::new(5, 3);
        canvas.write_pixel(0, 0, Color::new(1.5, 0.0, 0.0));
        canvas.write_pixel(2, 1, Color::new(0.0, 0.5, 0.0));
        canvas.write_pixel(4, 2, Color::new(-0.5, 0.0, 1.0));
        let ppm = canvas.to_ppm();
        let lines: Vec<&str> = ppm.lines().collect();
        assert_eq!(lines[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(lines[4], "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
        assert_eq!(lines[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
    }

    #[test]
    fn test_ppm_line_wrap() {
        let mut canvas = Canvas::new(10, 2);
        let color = Color::new(1.0, 0.8, 0.6);
        for y in 0..canvas.height {
            for x in 0..canvas.width {
                canvas.write_pixel(x, y, color);
            }
        }
        let ppm = canvas.to_ppm();
        let lines: Vec<&str> = ppm.lines().collect();

        assert_eq!(
            lines[3],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
        );
        assert_eq!(
            lines[4],
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
        assert_eq!(
            lines[5],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
        );
        assert_eq!(
            lines[6],
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
    }

    #[test]
    fn test_binary_ppm_header_and_clamped_pixels() {
        let mut canvas = Canvas::new(2, 1);
        canvas.write_pixel(0, 0, Color::new(1.5, 0.0, 0.5));
        canvas.write_pixel(1, 0, Color::new(-0.5, 0.2, 1.0));
        let mut out = Vec::new();
        canvas.write_ppm_binary(&mut out).unwrap();
        let header = b"P6\n2 1\n255\n";
        assert_eq!(&out[..header.len()], header);
        assert_eq!(&out[header.len()..], &[255, 0, 128, 0, 51, 255]);
    }
}
