use raylib::prelude::*;

mod controls;
mod fragment;
mod line;
mod obj;
mod renderer;
mod screen;

use controls::RotationControls;
use fragment::Fragment;
use obj::load_obj;
use renderer::{draw_fragments, render};
use screen::preparar_modelo;

const ANCHO_VENTANA: i32 = 800;
const ALTO_VENTANA: i32 = 600;
const MARGEN: f32 = 50.0;

pub struct Uniforms {
    pub model_matrix: Matrix,
}

fn create_model_matrix(rotation: Vector3) -> Matrix {
    Matrix::rotate_x(rotation.x) * Matrix::rotate_y(rotation.y)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let modelo = load_obj("assets/nave-graficas.obj")?;
    let vertices_preparados =
        preparar_modelo(&modelo.vertices, ANCHO_VENTANA, ALTO_VENTANA, MARGEN);
    let color_lineas = Color::new(64, 224, 176, 255);
    let mut controles = RotationControls::new();
    let fragments = vec![
        Fragment::new(100.0, 80.0, Vector3::new(1.0, 0.0, 0.0), 0.0),
        Fragment::new(101.0, 80.0, Vector3::new(0.0, 1.0, 0.0), 0.0),
        Fragment::new(102.0, 80.0, Vector3::new(0.0, 0.0, 1.0), 0.0),
        Fragment::new(100.0, 80.0, Vector3::new(1.0, 1.0, 0.0), 0.0),
        Fragment::new(-1.0, 80.0, Vector3::new(1.0, 1.0, 1.0), 0.0),
    ];

    let (mut raylib, hilo) = raylib::init()
        .size(ANCHO_VENTANA, ALTO_VENTANA)
        .title("Laboratorio 4")
        .build();

    raylib.set_target_fps(60);

    while !raylib.window_should_close() {
        controles.update(&raylib);
        let (angulo_x, angulo_y) = controles.angles();
        let model_matrix = create_model_matrix(Vector3::new(angulo_x, angulo_y, 0.0));
        let uniforms = Uniforms { model_matrix };

        let mut dibujo = raylib.begin_drawing(&hilo);
        dibujo.clear_background(Color::BLACK);
        render(
            &mut dibujo,
            &uniforms,
            &vertices_preparados,
            &modelo.indices,
            color_lineas,
            ANCHO_VENTANA,
            ALTO_VENTANA,
        );
        draw_fragments(&mut dibujo, &fragments);
    }

    Ok(())
}
