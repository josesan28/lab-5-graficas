use raylib::prelude::*;

use crate::Uniforms;
use crate::fragment::Fragment;
use crate::line::triangle;
use crate::screen::transformar_a_pantalla;

pub fn draw_fragments(dibujo: &mut RaylibDrawHandle, fragments: &[Fragment]) {
    for fragment in fragments {
        let color = Color::new(
            (fragment.color.x.clamp(0.0, 1.0) * 255.0) as u8,
            (fragment.color.y.clamp(0.0, 1.0) * 255.0) as u8,
            (fragment.color.z.clamp(0.0, 1.0) * 255.0) as u8,
            255,
        );

        dibujo.draw_pixel(
            fragment.position.x as i32,
            fragment.position.y as i32,
            color,
        );
    }
}

pub fn render(
    dibujo: &mut RaylibDrawHandle,
    uniforms: &Uniforms,
    vertex_array: &[Vector3],
    indices: &[usize],
    color: Color,
    ancho_ventana: i32,
    alto_ventana: i32,
) {
    let vertices = transformar_a_pantalla(vertex_array, uniforms, ancho_ventana, alto_ventana);

    for indices_triangulo in indices.chunks_exact(3) {
        let a = vertices[indices_triangulo[0]];
        let b = vertices[indices_triangulo[1]];
        let c = vertices[indices_triangulo[2]];

        triangle(dibujo, a, b, c, color);
    }
}
