use crate::fragment::Fragment;
use crate::line::line;
use crate::vertex::Vertex;

pub fn triangle(a: &Vertex, b: &Vertex, c: &Vertex) -> Vec<Fragment> {
    let mut fragments = line(a, b);
    fragments.extend(line(b, c));
    fragments.extend(line(c, a));
    fragments
}

pub fn filled_barycentric(a: &Vertex, b: &Vertex, c: &Vertex) -> Vec<Fragment> {
    let a_position = a.transformed_position;
    let b_position = b.transformed_position;
    let c_position = c.transformed_position;
    let area = edge(a_position, b_position, c_position.x, c_position.y);

    if area.abs() <= f32::EPSILON {
        return Vec::new();
    }

    let min_x = a_position.x.min(b_position.x).min(c_position.x).floor() as i32;
    let max_x = a_position.x.max(b_position.x).max(c_position.x).ceil() as i32;
    let min_y = a_position.y.min(b_position.y).min(c_position.y).floor() as i32;
    let max_y = a_position.y.max(b_position.y).max(c_position.y).ceil() as i32;
    let mut fragments = Vec::new();

    for y in min_y..max_y {
        for x in min_x..max_x {
            let pixel_x = x as f32 + 0.5;
            let pixel_y = y as f32 + 0.5;
            let weight_a = edge(b_position, c_position, pixel_x, pixel_y) / area;
            let weight_b = edge(c_position, a_position, pixel_x, pixel_y) / area;
            let weight_c = edge(a_position, b_position, pixel_x, pixel_y) / area;

            if weight_a >= 0.0 && weight_b >= 0.0 && weight_c >= 0.0 {
                fragments.push(Fragment::new(x as f32, y as f32, a.color, 0.0));
            }
        }
    }

    fragments
}

fn edge(a: raylib::prelude::Vector3, b: raylib::prelude::Vector3, x: f32, y: f32) -> f32 {
    (b.x - a.x) * (y - a.y) - (b.y - a.y) * (x - a.x)
}
