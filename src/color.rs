use crate::Interval;
use crate::Vec3;
pub type Color = Vec3;

pub fn write_color(s: &mut String, pixel_color: Color, samples_per_pixel: i32) {
    let scale = 1.0 / samples_per_pixel as f64;

    let r = scale * pixel_color.x();
    let g = scale * pixel_color.y();
    let b = scale * pixel_color.z();

    let intensity = Interval::new(0.0, 0.999);

    let ir = (256.0 * intensity.clamp(r)) as i32;
    let ig = (256.0 * intensity.clamp(g)) as i32;
    let ib = (256.0 * intensity.clamp(b)) as i32;

    s.push_str(&format!("{ir} {ig} {ib}\n"));
}
