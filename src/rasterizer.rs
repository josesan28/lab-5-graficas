use crate::fragment::Fragment;
use crate::triangle::{filled_barycentric, triangle};
use crate::vertex::Vertex;

#[derive(Clone, Copy)]
pub enum TriangleMode {
    Wireframe,
    FilledBarycentric,
    FilledScanline,
}

pub fn rasterize(triangles: &[[Vertex; 3]], mode: TriangleMode) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    for tri in triangles {
        let triangle_fragments = match mode {
            TriangleMode::Wireframe => triangle(&tri[0], &tri[1], &tri[2]),
            TriangleMode::FilledBarycentric => filled_barycentric(&tri[0], &tri[1], &tri[2]),
            TriangleMode::FilledScanline => {
                unimplemented!("El relleno por filas aún no está implementado")
            }
        };

        fragments.extend(triangle_fragments);
    }

    fragments
}

#[cfg(test)]
mod tests {
    use raylib::prelude::Vector3;

    use super::{TriangleMode, rasterize};
    use crate::vertex::Vertex;

    fn vertex(x: f32, y: f32) -> Vertex {
        Vertex::new_with_color(Vector3::new(x, y, 0.0), Vector3::new(1.0, 1.0, 1.0))
    }

    #[test]
    fn gathers_fragments_from_all_triangles() {
        let triangles = [
            [vertex(0.0, 0.0), vertex(1.0, 0.0), vertex(0.0, 1.0)],
            [vertex(2.0, 2.0), vertex(3.0, 2.0), vertex(2.0, 3.0)],
        ];

        let fragments = rasterize(&triangles, TriangleMode::Wireframe);

        assert_eq!(fragments.len(), 12);
    }
}
