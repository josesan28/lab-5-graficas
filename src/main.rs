use raylib::prelude::*;

mod fragment;
mod framebuffer;
mod line;
mod triangle;
mod vertex;

use framebuffer::Framebuffer;
use triangle::triangle;
use vertex::Vertex;

pub struct Uniforms {
    pub model_matrix: Matrix,
}

fn render(framebuffer: &mut Framebuffer, _uniforms: &Uniforms, vertex_array: &[Vertex]) {
    for vertices in vertex_array.chunks_exact(3) {
        let fragments = triangle(&vertices[0], &vertices[1], &vertices[2]);

        for fragment in fragments {
            framebuffer.point(
                fragment.position.x as i32,
                fragment.position.y as i32,
                fragment.color,
            );
        }
    }
}

fn main() {
    let (mut window, thread) = raylib::init()
        .size(800, 600)
        .title("Primer triángulo")
        .build();

    window.set_target_fps(60);

    let mut framebuffer = Framebuffer::new(800, 600);
    framebuffer.set_background_color(Vector3::new(0.0, 0.0, 0.0));
    framebuffer.init_texture(&mut window, &thread);

    let positions = [
        Vector3::new(300.0, 200.0, 0.0),
        Vector3::new(400.0, 400.0, 0.0),
        Vector3::new(500.0, 200.0, 0.0),
    ];

    let vertex_array: Vec<Vertex> = positions
        .into_iter()
        .map(|position| {
            let mut vertex = Vertex::new_with_color(position, Vector3::new(1.0, 1.0, 1.0));

            vertex.transformed_position = position;
            vertex
        })
        .collect();

    let uniforms = Uniforms {
        model_matrix: Matrix::identity(),
    };

    while !window.window_should_close() {
        framebuffer.clear();
        render(&mut framebuffer, &uniforms, &vertex_array);
        framebuffer.swap_buffers(&mut window, &thread);
    }
}
