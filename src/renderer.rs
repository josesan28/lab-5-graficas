use raylib::prelude::*;

use crate::line::triangle;

pub fn draw_model(
    dibujo: &mut RaylibDrawHandle,
    vertices: &[Vector2],
    indices: &[usize],
    color: Color,
) {
    for indices_triangulo in indices.chunks_exact(3) {
        let a = vertices[indices_triangulo[0]];
        let b = vertices[indices_triangulo[1]];
        let c = vertices[indices_triangulo[2]];

        triangle(dibujo, a, b, c, color);
    }
}
