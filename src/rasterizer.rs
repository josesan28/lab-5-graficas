use crate::fragment::Fragment;
use crate::triangle::{filled_barycentric, filled_scanline, triangle};
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
            TriangleMode::FilledScanline => filled_scanline(&tri[0], &tri[1], &tri[2]),
        };

        fragments.extend(triangle_fragments);
    }

    fragments
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use raylib::prelude::Vector3;

    use super::{TriangleMode, rasterize};
    use crate::fragment::Fragment;
    use crate::vertex::Vertex;

    fn vertex(x: f32, y: f32) -> Vertex {
        Vertex::new_with_color(Vector3::new(x, y, 0.0), Vector3::new(1.0, 1.0, 1.0))
    }

    fn triangle(points: [(f32, f32); 3]) -> [Vertex; 3] {
        points.map(|(x, y)| vertex(x, y))
    }

    fn positions(fragments: Vec<Fragment>) -> HashSet<(i32, i32)> {
        fragments
            .into_iter()
            .map(|fragment| (fragment.position.x as i32, fragment.position.y as i32))
            .collect()
    }

    fn filled_positions(triangle: [Vertex; 3], mode: TriangleMode) -> HashSet<(i32, i32)> {
        positions(rasterize(&[triangle], mode))
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

    #[test]
    fn filled_modes_generate_the_same_positions() {
        let cases = [
            [(1.0, 1.0), (4.0, 2.0), (2.0, 5.0)],
            [(10.0, 10.0), (110.0, 35.0), (40.0, 140.0)],
            [(2.0, 2.0), (8.0, 2.0), (5.0, 7.0)],
            [(3.0, 1.0), (3.0, 8.0), (9.0, 4.0)],
            [(2.0, 5.0), (4.0, 2.0), (1.0, 1.0)],
            [(-3.0, 2.0), (4.0, -2.0), (8.0, 7.0)],
            [(1.0, 1.0), (4.0, 4.0), (7.0, 7.0)],
        ];

        for points in cases {
            let barycentric = filled_positions(triangle(points), TriangleMode::FilledBarycentric);
            let scanline = filled_positions(triangle(points), TriangleMode::FilledScanline);

            assert_eq!(barycentric, scanline, "El caso {points:?} no coincide");
        }

        let degenerate = [(1.0, 1.0), (4.0, 4.0), (7.0, 7.0)];
        assert!(filled_positions(triangle(degenerate), TriangleMode::FilledBarycentric).is_empty());
        assert!(filled_positions(triangle(degenerate), TriangleMode::FilledScanline).is_empty());
    }

    #[test]
    fn reversing_vertices_keeps_the_same_positions() {
        let forward = [(1.0, 1.0), (4.0, 2.0), (2.0, 5.0)];
        let reversed = [forward[2], forward[1], forward[0]];

        for mode in [
            TriangleMode::FilledBarycentric,
            TriangleMode::FilledScanline,
        ] {
            let forward_positions = filled_positions(triangle(forward), mode);
            let reversed_positions = filled_positions(triangle(reversed), mode);

            assert_eq!(forward_positions, reversed_positions);
        }
    }
}
