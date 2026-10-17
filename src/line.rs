use raylib::prelude::*;

pub fn dibujar_punto(dibujo: &mut RaylibDrawHandle, posicion: Vector2, color: Color) {
    dibujo.draw_pixel(posicion.x.round() as i32, posicion.y.round() as i32, color);
}

pub fn line(dibujo: &mut RaylibDrawHandle, start: Vector2, end: Vector2, color: Color) {
    let mut x1 = start.x.round() as i32;
    let mut y1 = start.y.round() as i32;
    let x2 = end.x.round() as i32;
    let y2 = end.y.round() as i32;

    let dx = (x2 - x1).abs();
    let dy = -(y2 - y1).abs();
    let paso_x = if x1 < x2 { 1 } else { -1 };
    let paso_y = if y1 < y2 { 1 } else { -1 };
    let mut error = dx + dy;

    loop {
        dibujar_punto(dibujo, Vector2::new(x1 as f32, y1 as f32), color);

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
}

pub fn triangle(dibujo: &mut RaylibDrawHandle, a: Vector2, b: Vector2, c: Vector2, color: Color) {
    line(dibujo, a, b, color);
    line(dibujo, b, c, color);
    line(dibujo, c, a, color);
}
