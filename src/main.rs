use std::hint::black_box;
use std::time::{Duration, Instant};

use raylib::prelude::*;

mod fragment;
mod framebuffer;
mod line;
mod primitive;
mod rasterizer;
mod shaders;
mod triangle;
mod vertex;

use framebuffer::Framebuffer;
use primitive::primitive_assembly;
use rasterizer::{TriangleMode, rasterize};
use shaders::{fragment_shader, vertex_shader};
use vertex::Vertex;

pub struct Uniforms {
    pub model_matrix: Matrix,
}

const BENCHMARK_RUNS: usize = 100;
const BENCHMARK_SAMPLES: usize = 9;

fn render(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
    triangle_mode: TriangleMode,
) {
    // 1. Procesamiento de vértices
    let transformed_vertices: Vec<Vertex> = vertex_array
        .iter()
        .map(|vertex| vertex_shader(vertex, uniforms))
        .collect();

    // 2. Ensamblaje de primitivas
    let triangles = primitive_assembly(&transformed_vertices);

    // 3. Rasterización
    let fragments = rasterize(&triangles, triangle_mode);

    // 4. Procesamiento de fragmentos
    for fragment in fragments {
        let color = fragment_shader(&fragment);

        framebuffer.point(
            fragment.position.x as i32,
            fragment.position.y as i32,
            color,
        );
    }
}

fn main() {
    if std::env::args().any(|argument| argument == "--benchmark-triangles") {
        benchmark_triangles();
        return;
    }

    let (mut window, thread) = raylib::init().size(800, 600).title("Laboratorio 5").build();

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
    let triangle_mode = TriangleMode::FilledScanline;

    while !window.window_should_close() {
        framebuffer.clear();
        render(&mut framebuffer, &uniforms, &vertex_array, triangle_mode);
        framebuffer.swap_buffers(&mut window, &thread);
    }
}

fn benchmark_triangles() {
    let triangles = benchmark_cases();

    for (name, mode) in [
        ("Baricéntrico", TriangleMode::FilledBarycentric),
        ("Por filas", TriangleMode::FilledScanline),
    ] {
        let fragment_count = rasterize(&triangles, mode).len();
        let mut measurements = Vec::with_capacity(BENCHMARK_SAMPLES);

        for _ in 0..BENCHMARK_SAMPLES {
            measurements.push(measure_rasterization(&triangles, mode));
        }

        measurements.sort();
        let median = measurements[BENCHMARK_SAMPLES / 2];

        println!(
            "{name}: mediana de {median:?} para {BENCHMARK_RUNS} ejecuciones, {fragment_count} fragmentos",
        );
    }
}

fn measure_rasterization(triangles: &[[Vertex; 3]], mode: TriangleMode) -> Duration {
    let start = Instant::now();

    for _ in 0..BENCHMARK_RUNS {
        let fragments = rasterize(black_box(triangles), mode);
        black_box(fragments);
    }

    start.elapsed()
}

fn benchmark_cases() -> Vec<[Vertex; 3]> {
    let cases = [
        [(1.0, 1.0), (4.0, 2.0), (2.0, 5.0)],
        [(10.0, 10.0), (110.0, 35.0), (40.0, 140.0)],
        [(2.0, 2.0), (8.0, 2.0), (5.0, 7.0)],
        [(3.0, 1.0), (3.0, 8.0), (9.0, 4.0)],
        [(2.0, 5.0), (4.0, 2.0), (1.0, 1.0)],
        [(-3.0, 2.0), (4.0, -2.0), (8.0, 7.0)],
        [(1.0, 1.0), (4.0, 4.0), (7.0, 7.0)],
    ];

    cases
        .map(|points| {
            points.map(|(x, y)| {
                Vertex::new_with_color(Vector3::new(x, y, 0.0), Vector3::new(1.0, 1.0, 1.0))
            })
        })
        .into_iter()
        .collect()
}
