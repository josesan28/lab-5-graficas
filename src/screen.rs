use raylib::prelude::{Vector2, Vector3};

use crate::Uniforms;

pub fn preparar_modelo(
    vertices: &[Vector3],
    ancho_ventana: i32,
    alto_ventana: i32,
    margen: f32,
) -> Vec<Vector3> {
    if vertices.is_empty() {
        return Vec::new();
    }

    let mut minimo_x = f32::INFINITY;
    let mut maximo_x = f32::NEG_INFINITY;
    let mut minimo_y = f32::INFINITY;
    let mut maximo_y = f32::NEG_INFINITY;
    let mut minimo_z = f32::INFINITY;
    let mut maximo_z = f32::NEG_INFINITY;

    for vertice in vertices {
        minimo_x = minimo_x.min(vertice.x);
        maximo_x = maximo_x.max(vertice.x);
        minimo_y = minimo_y.min(vertice.y);
        maximo_y = maximo_y.max(vertice.y);
        minimo_z = minimo_z.min(vertice.z);
        maximo_z = maximo_z.max(vertice.z);
    }

    let centro_x = (minimo_x + maximo_x) / 2.0;
    let centro_y = (minimo_y + maximo_y) / 2.0;
    let centro_z = (minimo_z + maximo_z) / 2.0;

    let radio_modelo = vertices
        .iter()
        .map(|vertice| {
            let x = vertice.x - centro_x;
            let y = vertice.y - centro_y;
            let z = vertice.z - centro_z;
            (x * x + y * y + z * z).sqrt()
        })
        .fold(0.0_f32, f32::max);

    let diametro_modelo = radio_modelo * 2.0;
    let espacio_disponible =
        (ancho_ventana as f32 - margen * 2.0).min(alto_ventana as f32 - margen * 2.0);
    let escala = if diametro_modelo > 0.0 {
        espacio_disponible / diametro_modelo
    } else {
        1.0
    };

    vertices
        .iter()
        .map(|vertice| {
            Vector3::new(
                (vertice.x - centro_x) * escala,
                (vertice.y - centro_y) * escala,
                (vertice.z - centro_z) * escala,
            )
        })
        .collect()
}

pub fn transformar_a_pantalla(
    vertices: &[Vector3],
    uniforms: &Uniforms,
    ancho_ventana: i32,
    alto_ventana: i32,
) -> Vec<Vector2> {
    let centro_x = ancho_ventana as f32 / 2.0;
    let centro_y = alto_ventana as f32 / 2.0;

    vertices
        .iter()
        .map(|vertice| {
            let transformado = vertice.transform_with(uniforms.model_matrix);

            Vector2::new(centro_x + transformado.x, centro_y - transformado.y)
        })
        .collect()
}
