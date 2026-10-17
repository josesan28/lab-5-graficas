use raylib::prelude::*;

mod controls;
mod line;
mod obj;
mod renderer;
mod screen;

use controls::RotationControls;
use obj::load_obj;
use renderer::draw_model;
use screen::{preparar_modelo, rotar};

const ANCHO_VENTANA: i32 = 800;
const ALTO_VENTANA: i32 = 600;
const MARGEN: f32 = 50.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let modelo = load_obj("assets/nave-graficas.obj")?;
    let vertices_preparados =
        preparar_modelo(&modelo.vertices, ANCHO_VENTANA, ALTO_VENTANA, MARGEN);
    let color_lineas = Color::new(64, 224, 176, 255);
    let mut controles = RotationControls::new();

    let (mut raylib, hilo) = raylib::init()
        .size(ANCHO_VENTANA, ALTO_VENTANA)
        .title("Laboratorio 4")
        .build();

    raylib.set_target_fps(60);

    while !raylib.window_should_close() {
        controles.update(&raylib);
        let (angulo_x, angulo_y) = controles.angles();

        let vertices_pantalla = rotar(
            &vertices_preparados,
            angulo_x,
            angulo_y,
            ANCHO_VENTANA,
            ALTO_VENTANA,
        );

        let mut dibujo = raylib.begin_drawing(&hilo);
        dibujo.clear_background(Color::BLACK);
        draw_model(
            &mut dibujo,
            &vertices_pantalla,
            &modelo.indices,
            color_lineas,
        );
    }

    Ok(())
}
