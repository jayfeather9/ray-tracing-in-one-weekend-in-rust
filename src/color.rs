use super::vec3::Vec3;
pub type Color = Vec3;

pub fn write_color(s: &mut String, pixel_color: Color) {
    let ir = (255.999 * pixel_color.x()) as i32;
    let ig = (255.999 * pixel_color.y()) as i32;
    let ib = (255.999 * pixel_color.z()) as i32;

    s.push_str(&format!("{ir} {ig} {ib}\n"));
}
