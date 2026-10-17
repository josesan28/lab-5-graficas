use crate::fragment::Fragment;
use crate::vertex::Vertex;

pub fn line(start: &Vertex, end: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();
    let mut x1 = start.transformed_position.x.round() as i32;
    let mut y1 = start.transformed_position.y.round() as i32;
    let x2 = end.transformed_position.x.round() as i32;
    let y2 = end.transformed_position.y.round() as i32;

    let dx = (x2 - x1).abs();
    let dy = -(y2 - y1).abs();
    let paso_x = if x1 < x2 { 1 } else { -1 };
    let paso_y = if y1 < y2 { 1 } else { -1 };
    let mut error = dx + dy;

    loop {
        fragments.push(Fragment::new(
            x1 as f32,
            y1 as f32,
            start.color,
            start.transformed_position.z,
        ));

        if x1 == x2 && y1 == y2 {
            break;
        }

        let doble_error = 2 * error;

        if doble_error >= dy {
            error += dy;
            x1 += paso_x;
        }

        if doble_error <= dx {
            error += dx;
            y1 += paso_y;
        }
    }

    fragments
}
